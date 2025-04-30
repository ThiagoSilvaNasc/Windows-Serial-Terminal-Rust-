use chrono::Local;
use eframe::{egui, App, CreationContext, Frame};
use serialport::{available_ports, FlowControl};
use rfd::FileDialog;
use std::fs;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.vsync = false;
    eframe::run_native(
        "Leitor Serial",
        options,
        Box::new(|_cc: &CreationContext| Box::new(MyApp::default())),
    )
}

struct MyApp {
    ports: Vec<String>,
    selected_port: Option<String>,
    prev_selection: Option<String>,
    rx: Option<Receiver<String>>,
    text_buffer: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let ports = match available_ports() {
            Ok(info) => info.into_iter().map(|p| p.port_name).collect(),
            Err(err) => { eprintln!("Erro ao listar portas: {}", err); Vec::new() }
        };
        Self { ports, selected_port: None, prev_selection: None, rx: None, text_buffer: String::new() }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Salvar").clicked() {
                    if let Some(path) = FileDialog::new()
                        .set_file_name("output.txt")
                        .add_filter("Text", &["txt"])
                        .save_file()
                    {
                        if let Err(err) = fs::write(&path, &self.text_buffer) {
                            eprintln!("Erro ao salvar arquivo: {}", err);
                        }
                    }
                }
                ui.separator();
                egui::ComboBox::from_label("COM")
                    .selected_text(self.selected_port.clone().unwrap_or_else(|| "Selecione...".into()))
                    .show_ui(ui, |ui| {
                        for p in &self.ports {
                            ui.selectable_value(&mut self.selected_port, Some(p.clone()), p);
                        }
                    });
            });
        });

        if self.selected_port != self.prev_selection {
            self.prev_selection = self.selected_port.clone();
            self.text_buffer.clear();
            self.rx = None;
            if let Some(port_name) = &self.selected_port {
                let (tx, rx) = mpsc::channel();
                let port_name = port_name.clone();
                thread::spawn(move || {
                    if let Ok(mut port) = serialport::new(port_name, 115_200)
                        .timeout(Duration::from_millis(10))
                        .flow_control(FlowControl::None)
                        .open()
                    {
                        // Forçar DTR inativo
                        let _ = port.write_data_terminal_ready(false);
                        // Pulsar RTS para reset do ESP32 (linha RST)
                        let _ = port.write_request_to_send(true);
                        thread::sleep(Duration::from_millis(100));
                        let _ = port.write_request_to_send(false);
                        thread::sleep(Duration::from_millis(100));

                        let mut buf = [0u8; 1024];
                        loop {
                            match port.read(&mut buf) {
                                Ok(n) if n > 0 => {
                                    let raw = String::from_utf8_lossy(&buf[..n]);
                                    for segment in raw.split_inclusive('\n') {
                                        let trimmed = segment.trim_end_matches('\n');
                                        let now = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                                        let entry = if segment.ends_with('\n') {
                                            format!("[{}] {}\n", now, trimmed)
                                        } else {
                                            format!("[{}] {}", now, trimmed)
                                        };
                                        if tx.send(entry).is_err() { break; }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                });
                self.rx = Some(rx);
            }
        }

        if let Some(rx) = &self.rx {
            for entry in rx.try_iter() {
                self.text_buffer.push_str(&entry);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Serial Output");
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.label(&self.text_buffer);
            });
        });

        ctx.request_repaint();
    }
}
