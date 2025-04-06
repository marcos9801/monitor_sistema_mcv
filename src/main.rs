// src/main.rs

// modulo principal donde se encuentran los modulos para obtener informacion del sistema
mod core;  

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
    let cpu = cpu::obtener_info_cpu();
    let memoria = memoria::obtener_info_memoria();
    let disco = disco::obtener_info_disco();
    let interfaces = interfaces::obtener_info_interfaces();
    let procesos = procesos::obtener_info_procesos();

    
    println!("\nInformación del CPU:");
    println!("---------------------");
    println!("Marca: {}", cpu.get_brand());
    println!("Temperatura: {} °C", cpu.get_temperatura());
    println!("Cantidad de núcleos: {}", cpu.get_cantidad_nucleos());
    //println!("Cantidad de núcleos E: {}", cpu.get_cantidad_nucleos_e());
    //println!("Cantidad de núcleos P: {}", cpu.get_cantidad_nucleos_p());
    //println!("Núcleos lógicos: {}", cpu.get_nucleos_logicos());
    println!("Frecuencia: {} MHz", cpu.get_frecuencia());
    println!("Uso de núcleos: {:?}", cpu.get_uso_nucleos());

    println!("\nInformación de la memoria:");
    println!("---------------------");
    println!("Total entre SWap y Ram: {} MB", memoria.get_memoria_total_sistema());
    println!("Total: {} MB", memoria.get_memoria_total());
    println!("Libre: {} MB", memoria.get_memoria_libre());
    println!("Usada: {} MB", memoria.get_memoria_usada());
    println!("Swap total: {} MB", memoria.get_swap_total());
    println!("Swap libre: {} MB", memoria.get_swap_libre());
    println!("Swap usada: {} MB", memoria.get_swap_usada());
    //println!("Cache total: {} MB", memoria.get_cache_total());
    //println!("Cache libre: {} MB", memoria.get_cache_libre());
    //println!("Cache usada: {} MB", memoria.get_cache_usada());
    println!("---------------------");
    println!("\nInformación del disco:");
    println!("---------------------");
    println!("Cantidad de discos: {}", disco.get_cantidad_discos());
    println!("Espacio total: {} GB", disco.get_espacio_total());
    println!("Espacio libre: {} GB", disco.get_espacio_libre());
    println!("Espacio usado: {} GB", disco.get_espacio_usado());
    println!("---------------------");
    for i in 0..disco.get_cantidad_discos() {
        println!("Disco {}: ", i);
        println!("Nombre: {}", disco.get_disco(i).get_nombre());
        println!("Sistema de archivos: {}", disco.get_disco(i).get_sistema_archivos());
        println!("Espacio total: {} GB", disco.get_disco(i).get_espacio_total());
        println!("Espacio libre: {} GB", disco.get_disco(i).get_espacio_libre());
        println!("Espacio usado: {} GB", disco.get_disco(i).get_espacio_usado());
        println!("Ruta: {}", disco.get_disco(i).get_ruta());
        println!("Removible: {}", disco.get_disco(i).get_removible());
        println!("Solo lectura: {}", disco.get_disco(i).get_solo_lectura());
        println!("---------------------");
    }
    println!("\nInformación de las interfaces:");
    println!("---------------------");
    println!("Cantidad de interfaces: {}", interfaces.get_cantidad_interfaces());
    println!("Bytes recibidos: {}", interfaces.get_bytes_recibidos());
    println!("Bytes enviados: {}", interfaces.get_bytes_enviados());
    println!("Número de paquetes recibidos: {}", interfaces.get_numero_paquetes_recibidos());
    println!("Número de paquetes enviados: {}", interfaces.get_numero_paquetes_enviados());
    println!("Total de errores: {}", interfaces.get_total_errores());
    println!("Direcciones IP: {:?}", interfaces.get_direccion_ip());
    println!("Direcciones MAC: {}", interfaces.get_direccion_mac());
    println!("MTU: {}", interfaces.get_mtu());
    
    println!("---------------------");
    for i in 0..interfaces.get_cantidad_interfaces() {
        println!("Interfaz {}: ", i);
        println!("Nombre: {}", interfaces.get_interfaz(i).get_nombre());
        println!("Bytes recibidos: {}", interfaces.get_interfaz(i).get_bytes_recibidos());
        println!("Bytes enviados: {}", interfaces.get_interfaz(i).get_bytes_enviados());
        println!("Número de paquetes recibidos: {}", interfaces.get_interfaz(i).get_numero_paquetes_recibidos());
        println!("Número de paquetes enviados: {}", interfaces.get_interfaz(i).get_numero_paquetes_enviados());
        println!("Total de errores: {}", interfaces.get_interfaz(i).get_total_errores());
        println!("Dirección IP: {:?}", interfaces.get_interfaz(i).get_direccion_ip());
        println!("Dirección MAC: {}", interfaces.get_interfaz(i).get_direccion_mac());
        println!("MTU: {}", interfaces.get_interfaz(i).get_mtu());
        println!("---------------------");
    }
    println!("\nInformación de los procesos:");
    println!("---------------------");
    println!("Cantidad de procesos: {}", procesos.get_cantidad_procesos());

    println!("---------------------");
    for i in 0..procesos.get_cantidad_procesos() {
        println!("Proceso {}: ", i);
        println!("PID: {}", procesos.get_proceso(i).get_pid());
        println!("Nombre: {}", procesos.get_proceso(i).get_nombre());
        println!("Tiempo de ejecución: {} ms", procesos.get_proceso(i).get_tiempo_ejecucion());
        println!("Tiempo en CPU: {} ms", procesos.get_proceso(i).get_tiempo_en_cpu());
        println!("Uso CPU: {} %", procesos.get_proceso(i).get_uso_cpu());
        println!("Uso memoria: {} MB", procesos.get_proceso(i).get_uso_memoria());
        println!("Uso memoria virtual: {} MB", procesos.get_proceso(i).get_uso_memoria_virtual());
        println!("Estado: {}", procesos.get_proceso(i).get_estado());
        println!("---------------------");
    }
    println!("\ntop procesos por uso de CPU:");
    for proceso in procesos.get_top_procesos_uso_cpu() {
        println!("{}\n{}", proceso, "-".repeat(40));
    }
    println!("\ntop procesos por uso de memoria:");
    for proceso in procesos.get_top_procesos_uso_memoria() {
        println!("{}\n{}", proceso, "-".repeat(40));
    }
    println!("\ntop procesos por tiempo de CPU:");
    for proceso in procesos.get_top_procesos_tiempo_cpu() {
        println!("{}\n{}", proceso, "-".repeat(40));
    }
    println!("\ntop procesos por tiempo de ejecucion:");
    for proceso in procesos.get_top_procesos_tiempo_ejecucion() {
        println!("{}\n{}", proceso, "-".repeat(40));
    }
    println!("---------------------");

    println!()



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
