use std::error::Error;

mod system_info;

fn main() -> Result<(), Box<dyn Error>> {
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

    let bios_serial_number = system_info::get_sn();
    let network_path = "\\\\Terminal\\pre-testes\\";
    //let network_path = "\\\\192.168.1.10\\Projetos\\";
    let file_path = format!("{}{}.txt", network_path, bios_serial_number);
    system_info::write_system_info(file_path)?;
    Ok(())
}
