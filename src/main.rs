mod client;
mod system_info;
mod test_utility;
mod user_interaction;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("                ██████╗  █████╗      ██╗ █████╗                       ");
    println!("                ██╔══██╗██╔══██╗     ██║██╔══██╗                      ");
    println!("                ██║  ██║███████║     ██║███████║                      ");
    println!("                ██║  ██║██╔══██║██   ██║██╔══██║                      ");
    println!("                ██████╔╝██║  ██║╚█████╔╝██║  ██║                      ");
    println!("                ╚═════╝ ╚═╝  ╚═╝ ╚════╝ ╚═╝  ╚═╝                      ");
    println!("                                                                      ");
    println!("██████╗ ██████╗ ███████╗    ████████╗███████╗███████╗████████╗███████╗");
    println!("██╔══██╗██╔══██╗██╔════╝    ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝");
    println!("██████╔╝██████╔╝█████╗         ██║   █████╗  ███████╗   ██║   █████╗  ");
    println!("██╔═══╝ ██╔══██╗██╔══╝         ██║   ██╔══╝  ╚════██║   ██║   ██╔══╝  ");
    println!("██║     ██║  ██║███████╗       ██║   ███████╗███████║   ██║   ███████╗");
    println!("╚═╝     ╚═╝  ╚═╝╚══════╝       ╚═╝   ╚══════╝╚══════╝   ╚═╝   ╚══════╝");

    //Obtem informações do sistema
    println!("\n\n\nObtendo informações do sistema...\n");
    let system_info = system_info::get_system_info();

    //produção
    let network_file_path = "\\\\Terminal\\pre-testes\\";
    //testes
    //let network_file_path = "\\\\192.168.1.10\\Repo\\";

    let file_path = format!("{}{}.txt", network_file_path, system_info.bios_sn);

    //Trabalha as informações
    println!("\nEscrevendo informações do sistema\n");
    utils::write_system_info(system_info.clone(), file_path.clone())?;
    println!("\nInformações escritas com sucesso em {}\n", file_path);

    //Utilitario de testes
    if user_interaction::ask_user_to_run_test_utility() {
        println!("Executando o utilitário de execução de testes...");
        test_utility::run_test_utility();
    }

    //Cliente que se conecta no servidor do terminal
    client::send_sn(system_info.bios_sn).await?;
    Ok(())
}
