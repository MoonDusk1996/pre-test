use crate::system_info::SystemInfo;
use std::{
    fs::File,
    io::{self, Write},
};

// Limpa as saida de possiveis caracteres ou textos indesejados
pub fn clean_output(output: String) -> String {
    output
        .replace("SerialNumber", "")
        .replace("Serial number", "")
        .trim()
        .to_string()
}

// Escreve as informações do systema (recebe as informações e o local)
pub fn write_system_info(system_info: SystemInfo, file_path: String) -> io::Result<()> {
    let mut file = File::create(&file_path)?;

    // Informações do sistema
    writeln!(file, "=> Sistema:")?;
    writeln!(
        file,
        "Nome:                                {:?}",
        system_info.name
    )?;
    writeln!(
        file,
        "Versão do kernel:                    {:?}",
        system_info.kernel_version
    )?;
    writeln!(
        file,
        "Versão do sistema operacional:       {:?}",
        system_info.os_version
    )?;

    // Informações da placa mãe
    writeln!(file, "\n=> Placa mãe:")?;
    writeln!(file, "Número de série: {}", system_info.bios_sn)?;

    // Informações do CPU
    writeln!(file, "\n=> CPU:")?;
    writeln!(file, "Numero de CPUs: {}", system_info.num_cpus)?;

    // Informações da RAM
    writeln!(file, "\n=> Memoria:")?;
    writeln!(file, "Memoria total: {} bytes", system_info.total_memory)?;

    // Informações dos discos
    writeln!(file, "\n=> Discos:")?;
    for disk in system_info.disks {
        writeln!(file, "{}", disk)?;
    }

    // Informações da rede
    writeln!(file, "\n=> Rede:")?;
    for (interface_name, received, transmitted) in system_info.networks {
        writeln!(
            file,
            "{}: {} B (down) / {} B (up)",
            interface_name, received, transmitted,
        )?;
    }

    // Informações dos componentes
    writeln!(file, "\n=> Componentes:")?;
    for component in system_info.components {
        writeln!(file, "{}", component)?;
    }

    Ok(())
}
