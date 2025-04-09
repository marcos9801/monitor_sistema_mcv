use clap::{Parser, Subcommand};

use crate::core;
use crate::output::{log, json};
#[derive(Parser, Debug)]
#[command(
    name = "monitor_sistema_mcv",
    author = "Marcos Castellanos Villaseñor",
    version = "0.1.0",
    about = "Monitor del sistema en Rust",
    //curso = "Programacion de sistemas avanzados, Profesor: Jorge Ernesto Lopez Arce",
    long_about = "Una herramienta CLI para mostrar información del sistema (CPU, Memoria, Disco, Interfaces, Procesos) escrita en Rust.",
    display_order = 1
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    /// Muestra toda la información del sistema
    #[arg(long)]
    pub all: bool,

    /// Genera un log del sistema en formato JSON
    #[arg(long)]
    pub log: bool,
}
pub struct Monitor {
    pub cpu: core::cpu::CPUInfo,
    pub memoria: core::memoria::MemoriaInfo,
    pub discos: core::disco::DiscosInfo,
    pub interfaces: core::interfaces::InterfacesInfo,
    pub procesos: core::procesos::ProcesosInfo,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Cpu,
    Memoria,
    Disco,
    Interfaces,
    Procesos,
}

pub fn ejecutar(cli: Cli) {
    if cli.all {
        let m = Monitor {
            cpu: core::cpu::obtener_info_cpu(),
            memoria: core::memoria::obtener_info_memoria(),
            discos: core::disco::obtener_info_disco(),
            interfaces: core::interfaces::obtener_info_interfaces(),
            procesos: core::procesos::obtener_info_procesos(),
        };
        m.cpu.mostrar_info();
        m.memoria.mostrar_info();
        m.discos.mostrar_info();
        m.interfaces.mostrar_info();
        m.procesos.mostrar_info();
        if cli.log {
            log::generar_log_sistema(
                &json::exportar_procesos_json(&m.procesos),
                &json::exportar_interfaces_json(&m.interfaces),
                &json::exportar_discos_json(&m.discos),
                &json::exportar_memoria_json(&m.memoria),
                &json::exportar_cpu_json(&m.cpu),
            );
            println!("Log generado en el archivo log_sistema.json");
        } 
        else {
            println!("No se generó el log. Usa la opción --log para generarlo.");
        }
        return;
    }
    if cli.command.is_none() {
        println!("Usa `monitor_sistema_mcv -- help` o monitor_sistema_mcv -- h para ver las opciones disponibles.");
        return;
    }
    if cli.command.is_some() && cli.all {
        println!("No puedes usar la opción --all junto con un comando específico.");
        return;
    }

    match cli.command {
        Some(Commands::Cpu) =>{
            let cpu = core::cpu::obtener_info_cpu();
            cpu.mostrar_info();
        } 
        Some(Commands::Memoria) =>{
            let memoria = core::memoria::obtener_info_memoria();
            memoria.mostrar_info();
        } 
        Some(Commands::Disco) => {
            let disco = core::disco::obtener_info_disco();
            disco.mostrar_info();
        }
        Some(Commands::Interfaces) => {
            let interfaces = core::interfaces::obtener_info_interfaces();
            interfaces.mostrar_info();
        }
        Some(Commands::Procesos) => {
            let procesos = core::procesos::obtener_info_procesos();
            procesos.mostrar_info();
        }
        None => {
            println!("Usa `monitor --help` para ver las opciones disponibles.");
        }
    }
}
