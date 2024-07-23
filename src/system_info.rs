use crate::{user_interaction, utils};
use std::process::{Command, Output};
use sysinfo::{Components, Disks, Networks, System};

#[derive(Clone)]
pub struct SystemInfo {
    pub name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub num_cpus: usize,
    pub total_memory: u64,
    pub disks: Vec<String>,
    pub networks: Vec<(String, u64, u64)>,
    pub components: Vec<String>,
    pub bios_sn: String,
}

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let name = System::name().unwrap();
    let kernel_version = System::kernel_version().unwrap();
    let os_version = System::os_version().unwrap();
    let num_cpus = sys.cpus().len();
    let total_memory = sys.total_memory();
    let disks = Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| format!("{:?}", disk))
        .collect();
    let networks = Networks::new_with_refreshed_list()
        .iter()
        .map(|(interface_name, data)| {
            (
                interface_name.clone(),
                data.total_received(),
                data.total_transmitted(),
            )
        })
        .collect();
    let components = Components::new_with_refreshed_list()
        .iter()
        .map(|component| format!("{:?}", component))
        .collect();
    let bios_sn = {
        let command = "wmic bios get serialnumber";
        let output: Output = Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("Falha ao executar o comando");

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let cleaned_output = utils::clean_output(stdout);

        if cleaned_output.is_empty() {
            user_interaction::request_serial_number()
        } else {
            cleaned_output
        }
    };

    SystemInfo {
        name,
        kernel_version,
        os_version,
        num_cpus,
        total_memory,
        disks,
        networks,
        components,
        bios_sn,
    }
}
