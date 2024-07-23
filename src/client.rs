use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct TextData {
    text: String,
}

pub async fn send_sn(serial_number: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let text_data = TextData {
        text: serial_number,
    };

    let res = client
        .post("http://192.168.100.11:30001/set_clipboard")
        .json(&text_data)
        .send()
        .await?;

    if res.status().is_success() {
        println!("Texto enviado com sucesso!");
    } else {
        println!("Falha ao enviar o texto: {}", res.status());
    }

    Ok(())
}
