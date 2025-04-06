/// Módulo que implementa la obtención de la información de la memoria del sistema:
/// - Memoria RAM total, libre y usada.
/// - Memoria swap total, libre y usada.
///
/// Este módulo define una estructura principal:
///
/// ### `MemoriaInfo`
/// Representa la información detallada sobre el uso de memoria en el sistema.
/// - `total`: Memoria RAM total en MB.
/// - `libre`: Memoria RAM libre en MB.
/// - `usada`: Memoria RAM usada en MB.
/// - `swap_total`: Memoria swap total en MB.
/// - `swap_libre`: Memoria swap libre en MB.
/// - `swap_usada`: Memoria swap usada en MB.
///
/// #### Métodos de `MemoriaInfo`
/// - `get_memoria_total`: Devuelve la memoria RAM total del sistema.
/// - `get_memoria_libre`: Devuelve la memoria RAM libre del sistema.
/// - `get_memoria_usada`: Devuelve la memoria RAM usada del sistema.
/// - `get_swap_total`: Devuelve la memoria swap total del sistema.
/// - `get_swap_libre`: Devuelve la memoria swap libre del sistema.
/// - `get_swap_usada`: Devuelve la memoria swap usada del sistema.
/// - `get_memoria_total_sistema`: Devuelve la suma de RAM y swap totales.
/// - `new`: Constructor que crea una nueva instancia de `MemoriaInfo`.
/// - `desde_sistema`: Método auxiliar para construir `MemoriaInfo` desde una instancia de `System`.
///
/// ### Constantes
/// - `B_TO_MB`: Constante para la conversión de bytes a megabytes.
///
/// ### Función principal
/// - `obtener_info_memoria`: Devuelve una instancia de `MemoriaInfo` con la información actual de la memoria del sistema.
///
/// ###pendientes 
/// implementar mediciones para cache
///

/*
 Historial de cambios
 - `2025-05-05`: Creación del módulo y definición de la estructura `MemoriaInfo`.
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

/// A structure that provides information about the system's memory and swap usage.
///
/// # Methods
///
    /// ## Getters
impl MemoriaInfo {
    /// Devuelve la memoria total del sistema.
    ///
    /// # Retorno
    /// la memoria total del sistema en MB.
    pub fn get_memoria_total(&self) -> u64 {
        self.total
    }
    /// Devuelve la memoria libre del sistema.
    ///
    /// # Retorno
    /// la memoria libre del sistema en MB.
    pub fn get_memoria_libre(&self) -> u64 {
        self.libre
    }
    /// Devuelve la memoria usada del sistema.
    ///
    /// # Retorno
    /// la memoria usada del sistema en MB.
    pub fn get_memoria_usada(&self) -> u64 {
        self.usada
    }
    /// Devuelve la memoria swap total del sistema.
    ///
    /// # Retorno
    /// la memoria swap total del sistema en MB.
    pub fn get_swap_total(&self) -> u64 {
        self.swap_total
    }
    /// Devuelve la memoria swap libre del sistema.
    ///
    /// # Retorno
    /// la memoria swap libre del sistema en MB.
    pub fn get_swap_libre(&self) -> u64 {
        self.swap_libre
    }
    /// Devuelve la memoria swap usada del sistema.
    ///
    /// # Retorno
    /// la memoria swap usada del sistema en MB.
    pub fn get_swap_usada(&self) -> u64 {
        self.swap_usada
    }
    /// Devuelve la memoria total del sistema (RAM + Swap).
    ///
    /// # Retorno
    /// la memoria total del sistema (RAM + Swap) en MB.
    pub fn get_memoria_total_sistema(&self) -> u64 {
        self.total + self.swap_total
    }

    // Crear una nueva instancia
    /// Crea una nueva instancia de MemoriaInfo.
    ///
    /// # Retorno
    /// Una nueva instancia de MemoriaInfo con la información de memoria del sistema.
    pub fn new() -> Self {
        let mut s = System::new_with_specifics(RefreshKind::everything()); // Obtener únicamente información de memoria
        s.refresh_memory(); // Actualizar la memoria para obtener la información más reciente
        Self::desde_sistema(&s)
    }

    pub fn desde_sistema(s: &System) -> Self {
        MemoriaInfo {
            total: s.total_memory() / B_TO_MB,
            usada: s.used_memory() / B_TO_MB,
            libre: (s.total_memory() - s.used_memory()) / B_TO_MB,
            swap_total: s.total_swap() / B_TO_MB,
            swap_libre: s.free_swap() / B_TO_MB,
            swap_usada: s.used_swap() / B_TO_MB,
        }
    }
}

pub fn obtener_info_memoria() -> MemoriaInfo {
    MemoriaInfo::new()
}