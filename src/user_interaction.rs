use std::io::{self, Write};

// Solicita manualmente o numero de série do usuário
pub fn request_serial_number() -> String {
    println!("Número de série não encontrado!\nPor favor, digite o número de série que será gravado no arquivo de informações.");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    input.trim().to_uppercase()
}
pub fn ask_user_to_run_test_utility() -> bool {
    println!("Você deseja executar o utilitário de execução de testes? (s/n): ");

    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "s" | "sim" => true,
        _ => false,
    }
}
