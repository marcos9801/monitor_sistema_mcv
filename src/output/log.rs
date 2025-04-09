use chrono::Utc;
use serde_json::json;
use std::fs::File;
use std::io::Write;

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

    // Obtener timestamp actual en formato ISO 8601
    let timestamp = Utc::now().to_rfc3339();

    // Crear estructura JSON
    let log_json = json!({
        "timestamp": timestamp,
        "procesos": serde_json::from_str::<serde_json::Value>(&procesos).unwrap(),
        "interfaces": serde_json::from_str::<serde_json::Value>(&interfaces).unwrap(),
        "discos": serde_json::from_str::<serde_json::Value>(&discos).unwrap(),
        "memoria": serde_json::from_str::<serde_json::Value>(&memoria).unwrap(),
        "cpu": serde_json::from_str::<serde_json::Value>(&cpu).unwrap(),
    });

    // Serializar a string bonita
    let log_str = serde_json::to_string_pretty(&log_json).unwrap();

    // Guardar a archivo
    let mut archivo = File::create("log_sistema.json").expect("No se pudo crear el archivo");
    archivo.write_all(log_str.as_bytes()).expect("No se pudo escribir en el archivo");
}
