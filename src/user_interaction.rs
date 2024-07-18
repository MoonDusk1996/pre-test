use std;

// Solicita manualmente o numero de série do usuário
pub fn request_serial_number() -> String {
    println!("Número de série não encontrado!\nPor favor, digite o número de série que será gravado no arquivo de informações.");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    input.trim().to_uppercase()
}
