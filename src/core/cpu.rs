/*
Modulo para obtener la informacion de la CPU
    - Marca
    - Temperatura
    - Cantidad de nucleos
    - Frecuencia
    - Uso por nucleo

Creado en 2025-04-05
*/
use sysinfo::{System, RefreshKind, CpuRefreshKind};

pub struct CPUInfo {
    brand: String,
    temperatura: f32,
    cantidad_nucleos: usize,
    //cantidad_nucleos_e: usize,
    //cantidad_nucleos_p: usize,
    //nucleos_logicos: usize,
    frecuencia: u64,
    uso_nucleos: Vec<f32>,
}

impl CPUInfo {
    // Getters
    pub fn get_brand(&self) -> &str {&self.brand}
    pub fn get_temperatura(&self) -> f32 {self.temperatura}
    pub fn get_cantidad_nucleos(&self) -> usize {self.cantidad_nucleos}
    /*
    pub fn get_cantidad_nucleos_e(&self) -> usize {self.cantidad_nucleos_e}
    pub fn get_cantidad_nucleos_p(&self) -> usize {self.cantidad_nucleos_p}
    pub fn get_nucleos_logicos(&self) -> usize {self.nucleos_logicos}
    */
    pub fn get_frecuencia(&self) -> u64 {self.frecuencia}
    pub fn get_uso_nucleos(&self) -> &Vec<f32> {&self.uso_nucleos}

    // Método para crear nueva instancia
    pub fn new() -> Self {
        let mut s = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())); //unicamnete refrescar la CPU
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        s.refresh_cpu_all();
        Self {
            brand: s.cpus()[0].brand().to_string(),
            temperatura: 0.0, //PLaceholder TODO: Implementar la obtención de temperatura
            cantidad_nucleos: s.cpus().len(),
            //cantidad_nucleos_e: s.cpus().iter().filter(|cpu| cpu.is_stepping()).count(),
            //cantidad_nucleos_p: s.cpus().iter().filter(|cpu| cpu.is_pstate()).count(),
            //nucleos_logicos: s.cpus().iter().map(|cpu| cpu.logical_count()).sum(),
            frecuencia: s.cpus()[0].frequency(),
            uso_nucleos: s.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        }
    }
}

// Función auxiliar para obtener la información
pub fn obtener_info_cpu() -> CPUInfo {
    CPUInfo::new()
}
