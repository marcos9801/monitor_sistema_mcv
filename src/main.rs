// src/main.rs
// modulo principal donde se encuentran los modulos para obtener informacion del sistema
mod core;  
mod output; // modulo para la salida de informacion
mod cli; // modulo para la interfaz de linea de comandos
use clap::Parser;
//modulo para la interfaz de linea de comandos
use cli::{Cli, Commands};
//librerias externas 
use std::thread;
use std::time::Duration;
//importacion de modulos especificos
use core::cpu;  // modulo de cpu para informacion de cpu (marca, temperatura, cantidad de nucleos, frecuencia, uso por nucleo)
use core::memoria; // modulo de memoria para informacion de memoria (total, libre, usada, swap total, libre, usada, cache total, libre, usada)
use core::disco; // modulo de disco para informacion de disco (total, libre, usada, nombre, iops, velocidad, tiempo de respuesta)
use core::interfaces; // modulo de interfaces para informacion de interfaces (nombre, bits recibidos, bits enviados, numero de paquetes recibidos, numero de paquetes enviados, total de errores, direccion ip, direccion mac, mtu)
use core::procesos; // modulo de procesos para informacion de procesos (pid, nombre, tiempo de ejecucion, tiempo en cpu, uso cpu, uso memoria, uso memoria virtual, estado, hilos)


fn main() {
    let cli = Cli::parse();
    cli::ejecutar(cli);
}


#[cfg(test)]
mod tests {
    use super::*;  // Importa todas las funciones y módulos del archivo para los tests

    #[test]
    fn test_cpu_info() {
        let cpu = cpu::obtener_info_cpu();
        
        assert!(!cpu.get_brand().is_empty(), "La marca del CPU no debería estar vacía.");
        assert!(cpu.get_temperatura() > 0.0, "La temperatura del CPU debería ser mayor que 0.");
        assert!(cpu.get_cantidad_nucleos() > 0, "La cantidad de núcleos debería ser mayor que 0.");
        assert!(cpu.get_frecuencia() > 0, "La frecuencia del CPU debería ser mayor que 0.");
    }

    #[test]
    fn test_memoria_info() {
        let memoria = memoria::obtener_info_memoria();

        assert!(memoria.get_memoria_total() > 0, "La memoria total debería ser mayor que 0.");
        assert!(memoria.get_memoria_libre() >= 0, "La memoria libre no debería ser negativa.");
        assert!(memoria.get_memoria_usada() >= 0, "La memoria usada no debería ser negativa.");
        assert!(memoria.get_swap_total() >= 0, "El swap total no debería ser negativo.");
    }

    #[test]
    fn test_disco_info() {
        let disco = disco::obtener_info_disco();

        assert!(disco.get_cantidad_discos() > 0, "La cantidad de discos debería ser mayor que 0.");
        assert!(disco.get_espacio_total() > 0.0, "El espacio total del disco debería ser mayor que 0.");
        assert!(disco.get_espacio_libre() >= 0.0, "El espacio libre del disco no debería ser negativo.");
        assert!(disco.get_espacio_usado() >= 0.0, "El espacio usado del disco no debería ser negativo.");
    }

    #[test]
    fn test_interfaces_info() {
        let interfaces = interfaces::obtener_info_interfaces();

        assert!(interfaces.get_cantidad_interfaces() > 0, "La cantidad de interfaces debería ser mayor que 0.");
        assert!(interfaces.get_bytes_recibidos() >= 0, "Los bytes recibidos no deberían ser negativos.");
        assert!(interfaces.get_bytes_enviados() >= 0, "Los bytes enviados no deberían ser negativos.");
        assert!(interfaces.get_numero_paquetes_recibidos() >= 0, "El número de paquetes recibidos no debería ser negativo.");
        assert!(interfaces.get_numero_paquetes_enviados() >= 0, "El número de paquetes enviados no debería ser negativo.");
        assert!(interfaces.get_total_errores() >= 0, "El total de errores no debería ser negativo.");
    
        assert!(interfaces.get_direccion_mac() > 0, "No se están obteniendo direcciones MAC. cantidad de interfaces: {}", interfaces.get_direccion_mac());
        assert!(interfaces.get_mtu() > 0, "El MTU debería ser mayor que 0.");

        for i in 0..interfaces.get_cantidad_interfaces() {
            let interfaz = interfaces.get_interfaz(i);
            assert!(!interfaz.get_nombre().is_empty(), "El nombre de la interfaz no debería estar vacío.");
            assert!(interfaz.get_bytes_recibidos() >= 0, "Los bytes recibidos no deberían ser negativos.");
            assert!(interfaz.get_bytes_enviados() >= 0, "Los bytes enviados no deberían ser negativos.");
            assert!(interfaz.get_numero_paquetes_recibidos() >= 0, "El número de paquetes recibidos no debería ser negativo.");
            assert!(interfaz.get_numero_paquetes_enviados() >= 0, "El número de paquetes enviados no debería ser negativo.");
            assert!(interfaz.get_total_errores() >= 0, "El total de errores no debería ser negativo.");
        }
    }

    
}
