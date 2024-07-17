use std::{
    fs::File,
    io::{self, Write},
    process::{Command, Output},
};
use sysinfo::{Components, Disks, Networks, System};

pub fn get_sn() -> String {
    let command = "wmic bios get serialnumber";
    let output: Output = Command::new("cmd")
        .args(&["/C", command])
        .output()
        .expect("Falha ao executar o comando");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    // Remover qualquer instância de "SerialNumber" ou "Serial number"
    let cleaned_output = stdout
        .replace("SerialNumber", "")
        .replace("Serial number", "")
        .trim()
        .to_string();

    if cleaned_output.is_empty() {
        println!("Número de série não encontrado!\nPor favor, digite o numero de série que será gravado no arquivo de informações.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        let trimmed_input = input.trim().to_uppercase();
        trimmed_input
    } else {
        cleaned_output
    }
}
pub fn write_system_info(file_path: String) -> io::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut file = File::create(&file_path)?;

    // Informações do sistema
    writeln!(file, "=> System:")?;
    writeln!(file, "name:             {:?}", System::name())?;
    writeln!(file, "kernel version:   {:?}", System::kernel_version())?;
    writeln!(file, "OS version:       {:?}", System::os_version())?;

    // Informações do CPU
    writeln!(file, "\n=> CPU:")?;
    writeln!(file, "Number of CPUs: {}", sys.cpus().len())?;

    // Informações da RAM
    writeln!(file, "\n=> Memory:")?;
    writeln!(file, "Total memory: {} bytes", sys.total_memory())?;

    // Informações dos discos
    writeln!(file, "\n=> Disks:")?;
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        writeln!(file, "{:?}", disk)?;
    }

    // Informações da rede
    writeln!(file, "\n=> Networks:")?;
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        writeln!(
            file,
            "{}: {} B (down) / {} B (up)",
            interface_name,
            data.total_received(),
            data.total_transmitted(),
        )?;
    }

    // Informações dos componentes
    writeln!(file, "\n=> Components:")?;
    let components = Components::new_with_refreshed_list();
    for component in &components {
        writeln!(file, "{:?}", component)?;
    }

    Ok(())
}
