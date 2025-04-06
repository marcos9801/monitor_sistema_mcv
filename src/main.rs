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
fn main() {
    let cpu = cpu::obtener_info_cpu();
    let memoria = memoria::obtener_info_memoria();
    let disco = disco::obtener_info_disco();
    let interfaces = interfaces::obtener_info_interfaces();
    //let interfaces = interfaces::obtener_info_interfaces();
    //let procesos = procesos::obtener_info_procesos();

    
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
    println!("Dirección IP: {:?}", interfaces.get_direccion_ip());
    println!("Dirección MAC: {}", interfaces.get_direccion_mac());
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
    //println!("\nInformación de los procesos:");

}

