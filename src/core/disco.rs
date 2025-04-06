/*
Modulo que implementa la obtencion de la informacion de los discos
    - Cantidad de discos
    - Espacio total
    - Espacio libre
    - Espacio usado
    - Nombre del disco
    - Sistema de archivos
    - Espacio total del disco
    - Espacio libre del disco
    - Espacio usado del disco
    - Ruta del disco
    - Removible
    - Solo lectura

Creado en 2025-04-05
*/
use sysinfo::{Disks};

pub struct DiscosInfo {
    cantidad_discos: usize,
    espacio_total: f64,
    espacio_libre: f64,
    espacio_usado: f64,
    discos: Vec<DiscoInfo>,
}

pub struct DiscoInfo {
    nombre: String,
    sistema_archivos: String,
    espacio_total: f64,
    espacio_libre: f64,
    espacio_usado: f64,
    ruta: String,
    removible: bool,
    solo_lectura: bool,
    //velocidad: u64,
    //tiempo_respuesta: u64,
}

const B_TO_GB: u64 = 1024 * 1024 * 1024; // conversion de bytes a GB

impl DiscosInfo {
    // Getters
    pub fn get_cantidad_discos(&self) -> usize { self.cantidad_discos }
    pub fn get_espacio_total(&self) -> f64 { self.espacio_total }
    pub fn get_espacio_libre(&self) -> f64 { self.espacio_libre }
    pub fn get_espacio_usado(&self) -> f64 { self.espacio_usado }
    pub fn get_discos(&self) -> &Vec<DiscoInfo> { &self.discos }
    pub fn get_disco(&self, index: usize) -> &DiscoInfo {
        if index < self.discos.len() {
            &self.discos[index]
        } else {
            panic!("Índice fuera de rango");
        }
    }

    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        let mut discos: Vec<DiscoInfo> = Vec::new();
        let mut total = 0.0;
        let mut cantidad = 0;
        let mut libre = 0.0;

        for disk in disks.list() {
            if discos.iter().any(|d| d.nombre == disk.name().to_string_lossy().to_string()) {
                continue;
            }
            cantidad += 1;
            let t = disk.total_space() / B_TO_GB;
            let l = disk.available_space() / B_TO_GB;
            total += t as f64;
            libre += l as f64;
            discos.push(DiscoInfo::new(
                disk.name().to_string_lossy().to_string(),
                disk.file_system().to_string_lossy().to_string(),
                t as f64,
                l as f64,
                (t - l) as f64,
                disk.mount_point().to_string_lossy().to_string(),
                disk.is_removable(),
                disk.is_read_only(),
            ));
        }

        DiscosInfo {
            cantidad_discos: cantidad,
            espacio_total: total,
            espacio_libre: libre,
            espacio_usado: total - libre,
            discos,
        }
    }
}

impl DiscoInfo {
    // Getters
    pub fn get_nombre(&self) -> &str { &self.nombre }
    pub fn get_sistema_archivos(&self) -> &str { &self.sistema_archivos }
    pub fn get_espacio_total(&self) -> f64 { self.espacio_total }
    pub fn get_espacio_libre(&self) -> f64 { self.espacio_libre }
    pub fn get_espacio_usado(&self) -> f64 { self.espacio_usado }
    pub fn get_ruta(&self) -> &str { &self.ruta }
    pub fn get_removible(&self) -> bool { self.removible }
    pub fn get_solo_lectura(&self) -> bool { self.solo_lectura }
    //pub fn get_velocidad(&self) -> u64 { self.velocidad }
    //pub fn get_tiempo_respuesta(&self) -> u64 { self.tiempo_respuesta }
    pub fn new(nombre: String, sistema_archivos: String, espacio_total: f64, espacio_libre: f64, espacio_usado: f64, ruta: String, removible: bool, solo_lectura: bool) -> Self {
        DiscoInfo {
            nombre,
            sistema_archivos,
            espacio_total,
            espacio_libre,
            espacio_usado,
            ruta,
            removible,
            solo_lectura,
        }
    }
}

pub fn obtener_info_disco() -> DiscosInfo {
    DiscosInfo::new()
}
