Windows Serial Terminal (Rust)



Terminal serial para Windows, otimizado para captura de logs em geral e testado com ESP32‑S3.

📦 Recursos

Leitura contínua e exibição de bytes recebidos na COM do Windows

Desabilita RTS/CTS para evitar modo bootloader no ESP32‑S3

Suporte a configurações de baud rate, paridade, bits de dados e timeout

Implementação simples usando o crate serialport

🚀 Instalação

Clone o repositório:

git clone https://github.com/ThiagoSilvaNasc/Windows-Serial-Terminal-Rust-.git
cd windows-serial-terminal

Instale o Rust (se ainda não tiver):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Compile e execute em modo release:

cargo run --release

⚙️ Configuração

No arquivo src/main.rs, ajuste o o baudrate desejado

        if let Ok(mut port) = serialport::new(port_name, 115_200)
                        .timeout(Duration::from_millis(10))
                        .flow_control(FlowControl::None)
                        .open()
                    

💡 Uso

Ao executar, a aplicação exibirá no console todas as mensagens recebidas pela porta COM.

Ideal para monitorar logs gerais de dispositivos embarcados, como ESP32‑S3.

> cargo run --release
Lidos 12 bytes: [0x45, 0x52, 0x52, 0x3A, 0x20, 0x53, 0x65, 0x6E, 0x68, 0x61, 0x20, 0x31]


📄 Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.