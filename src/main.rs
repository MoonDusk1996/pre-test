mod system_info;
mod user_interaction;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    println!("\n\n\nObtendo informações do sistema...\n");
    let system_info = system_info::get_system_info();
    let network_path = "\\\\Terminal\\pre-testes\\";
    let file_path = format!("{}{}.txt", network_path, system_info.bios_sn);
    println!("\nEscrevendo informações do sistema em {}\n", file_path);
    utils::write_system_info(system_info, file_path)?;
    Ok(())
}
