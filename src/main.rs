// src/main.rs

// modulo principal donde se encuentran los modulos para obtener informacion del sistema
mod core;  

//librerias externas 
use std::thread;
use std::time::Duration;
//importacion de modulos especificos
use core::cpu;  // modulo de cpu para informacion de cpu (marca, temperatura, cantidad de nucleos, frecuencia, uso por nucleo)
use core::memoria; // modulo de memoria para informacion de memoria (total, libre, usada, swap total, libre, usada, cache total, libre, usada)

fn main() {
    let cpu = cpu::obtener_info_cpu();
    let mut memoria = memoria::obtener_info_memoria();
    
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
    thread::sleep(Duration::from_secs(3));
    memoria = memoria::obtener_info_memoria();
    println!("\nInformación de la memoria (actualizada):");
    println!("---------------------");
    println!("Total entre SWap y Ram: {} MB", memoria.get_memoria_total_sistema());
    println!("Total: {} MB", memoria.get_memoria_total());
    println!("Libre: {} MB", memoria.get_memoria_libre());
    println!("Usada: {} MB", memoria.get_memoria_usada());
    println!("Swap total: {} MB", memoria.get_swap_total());
    println!("Swap libre: {} MB", memoria.get_swap_libre());
    println!("Swap usada: {} MB", memoria.get_swap_usada());

}

