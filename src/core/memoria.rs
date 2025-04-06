/*
Modulo para obtener la informacion de la memoria
    - Ram
        - total
        -libre
        -usada
    - Swap
        - total
        - libre
        - usada
    
Guarda las cantidades en MB

Creado en 2025-04-05
TODO: implementar mediciones para cache
*/
use sysinfo::{System, RefreshKind};

//conversiones de bytes a MB y GB
const B_TO_MB: u64 = 1024 * 1024;

pub struct MemoriaInfo {
    total: u64,
    libre: u64,
    usada: u64,
    swap_total: u64,
    swap_libre: u64,
    swap_usada: u64,
    //cache_total: u64,
    //cache_libre: u64,
    //cache_usada: u64,
}

impl MemoriaInfo{
    //getters
    pub fn get_memoria_total(&self) -> u64 {self.total}
    pub fn get_memoria_libre(&self) -> u64 {self.libre}
    pub fn get_memoria_usada(&self) -> u64 {self.usada}
    pub fn get_swap_total(&self) -> u64 {self.swap_total}
    pub fn get_swap_libre(&self) -> u64 {self.swap_libre}
    pub fn get_swap_usada(&self) -> u64 {self.swap_usada}
    //pub fn get_cache_total(&self) -> u64 {self.cache_total}
    //pub fn get_cache_libre(&self) -> u64 {self.cache_libre}
    //pub fn get_cache_usada(&self) -> u64 {self.cache_usada}
    pub fn get_memoria_total_sistema(&self) -> u64 {self.total + self.swap_total }
    //obtener nueva instancia

    pub fn new() -> Self{
        let mut  s = System::new_with_specifics(RefreshKind::everything()); //obtener unicamente informacion de memoria
        s.refresh_memory(); //actualizar la memoria
        Self::desde_sistema(&s)
    }

    pub fn desde_sistema(s: &System) -> Self{
        MemoriaInfo {
            total: s.total_memory() / B_TO_MB,
            usada: s.used_memory() / B_TO_MB,
            libre: (s.total_memory() - s.used_memory()) / B_TO_MB,
            swap_total: s.total_swap() / B_TO_MB,
            swap_libre: s.free_swap() / B_TO_MB,
            swap_usada: s.used_swap() / B_TO_MB,
            //cache_total: s.total_cache(),
            //cache_libre: s.free_cache(),
            //cache_usada: s.used_cache(),
        }
    }

}
pub fn obtener_info_memoria() -> MemoriaInfo {
    MemoriaInfo::new()
}