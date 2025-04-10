/// Generador de logs en formato JSON
///
/// Este módulo contiene la función `generar_log_sistema` que genera un log del sistema
/// en formato JSON. La función toma como parámetros las cadenas JSON de los procesos,
/// interfaces, discos, memoria y CPU, y las combina en un solo objeto JSON.
/// El log incluye información adicional como la versión, autor, fecha y hora.
///
/// # Ejemplo
/// ```
/// let procesos_json = exportar_procesos_json(&procesos);
/// let interfaces_json = exportar_interfaces_json(&interfaces);
/// let discos_json = exportar_discos_json(&discos);
/// let memoria_json = exportar_memoria_json(&memoria);
/// let cpu_json = exportar_cpu_json(&cpu);
/// generar_log_sistema(&procesos_json, &interfaces_json, &discos_json, &memoria_json, &cpu_json);
/// ```
/// # Creado en 2025-abr-08
/// 9-apr-2025 -> guardado de logs en carpeta logs/
/// TODO: implementar ruta de archivo



use chrono::Utc;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;
use chrono_tz::Tz;

use super::json::{
    exportar_procesos_json,
    exportar_interfaces_json,
    exportar_discos_json,
    exportar_memoria_json,
    exportar_cpu_json,
};

pub fn generar_log_sistema(
    procesos: &str,
    interfaces: &str,
    discos: &str,
    memoria: &str,
    cpu: &str,
) {
    // Obtener datos individuales
    let zona = chrono_tz::America::Mexico_City; // Zona horaria de México
    let fecha_hora = Utc::now().with_timezone(&zona);
    let fecha = fecha_hora.format("%Y-%m-%d").to_string();
    let hora = fecha_hora.format("%H:%M:%S").to_string();

    // Obtener timestamp actual en formato ISO 8601
    let timestamp = fecha_hora.format("%Y-%m-%dT%H:%M:%S%:z").to_string();

    // Crear estructura JSON
    let log_json = json!({
        "apendice":{
            "version": "1.0.0",
            "autor": "Marcos Castellanos Villaseñor",
            "fecha": timestamp,
            "dia": fecha_hora.format("%d").to_string(),
            "hora": hora,
        },
        "procesos": serde_json::from_str::<serde_json::Value>(&procesos).unwrap(),
        "interfaces": serde_json::from_str::<serde_json::Value>(&interfaces).unwrap(),
        "discos": serde_json::from_str::<serde_json::Value>(&discos).unwrap(),
        "memoria": serde_json::from_str::<serde_json::Value>(&memoria).unwrap(),
        "cpu": serde_json::from_str::<serde_json::Value>(&cpu).unwrap(),
    });

    // Serializar a string bonita
    let log_str = serde_json::to_string_pretty(&log_json).unwrap();

    // Guardar a archivo
    guardar_log_sistema(&log_str, &timestamp);
}
fn guardar_log_sistema(log_str: &str, timestamp: &str) {
    let carpeta_logs = "logs";

    if !Path::new(carpeta_logs).exists() {
        fs::create_dir_all(carpeta_logs).expect("No se pudo crear la carpeta log");
    }
    let filename = format!("{}/{}_log_sistema.json", carpeta_logs, timestamp);
    let mut archivo = File::create(&filename).expect("No se pudo crear el archivo");
    // Escribir el contenido
    archivo.write_all(log_str.as_bytes()).expect("No se pudo escribir en el archivo");

    println!("Log guardado en {}", filename);
}
    