use crate::procesos::ProcesosInfo;
use crate::interfaces::InterfacesInfo;
use crate::disco::DiscosInfo;
use crate::memoria::MemoriaInfo;
use crate::cpu::CPUInfo;
use serde_json::json;


pub fn exportar_procesos_json(procesos: &ProcesosInfo) -> String {
    let procesos_json = json!({
        "cantidad_procesos": procesos.get_cantidad_procesos(),
        "top 5 procesos uso cpu": procesos.get_top_procesos_uso_cpu(),
        "top 5 procesos uso memoria": procesos.get_top_procesos_uso_memoria(),
        "top 5 procesos tiempo en cpu": procesos.get_top_procesos_tiempo_cpu(),
        "top 5 procesos tiempo ejecucion": procesos.get_top_procesos_tiempo_ejecucion(),

    });
    let procesos_json_str = serde_json::to_string_pretty(&procesos_json).unwrap();
    return procesos_json_str;
}

pub fn exportar_interfaces_json(interfaces: &InterfacesInfo) -> String {
    let interfaces_json = json!({
        "cantidad_interfaces": interfaces.get_cantidad_interfaces(),
        "interfaces": interfaces.get_interfaces(),
        "total errors": interfaces.get_total_errores(),
        "total bytes recibidos": interfaces.get_bytes_recibidos(),
        "total bytes enviados": interfaces.get_bytes_enviados(),
        "total paquetes enviados": interfaces.get_numero_paquetes_enviados(),
        "total paquetes recibidos": interfaces.get_numero_paquetes_recibidos(),
        "total direcciones ip": interfaces.get_direccion_ip(),
        "total direcciones mac": interfaces.get_direccion_mac(),
        "interfaces": interfaces.get_interfaces()
    });
    let interfaces_json_str = serde_json::to_string_pretty(&interfaces_json).unwrap();
    return interfaces_json_str;
}

pub fn exportar_discos_json(discos: &DiscosInfo) -> String {
    let discos_json = json!({
        "cantidad_discos": discos.get_cantidad_discos(),
        "espacio_total": discos.get_espacio_total(),
        "espacio_libre": discos.get_espacio_libre(),
        "espacio_usado": discos.get_espacio_usado(),
        "discos": discos.get_discos()
    });
    let discos_json_str = serde_json::to_string_pretty(&discos_json).unwrap();
    return discos_json_str;
}

pub fn exportar_memoria_json(memoria: &MemoriaInfo) -> String {
    let memoria_json = json!({
        "memoria_total": memoria.get_memoria_total(),
        "memoria_libre": memoria.get_memoria_libre(),
        "memoria_usada": memoria.get_memoria_usada(),
        "memoria_ram_total": memoria.get_total_ram(),
        "memoria_ram_libre": memoria.get_libre_ram(),
        "memoria_ram_usada": memoria.get_usada_ram(),
        "memoria_swap_total": memoria.get_swap_total(),
        "memoria_swap_libre": memoria.get_swap_libre(),
        "memoria_swap_usada": memoria.get_swap_usada()
    });
    let memoria_json_str = serde_json::to_string_pretty(&memoria_json).unwrap();
    return memoria_json_str;
}

pub fn exportar_cpu_json(cpu: &CPUInfo) -> String {
    let cpu_json = json!({
        "cpu marca":cpu.get_brand(),
        "cantidad nucleos": cpu.get_cantidad_nucleos(),
        "frecuencia": cpu.get_frecuencia(),
        "uso nucleos": cpu.get_uso_nucleos(),
    });
    let cpu_json_str = serde_json::to_string_pretty(&cpu_json).unwrap();
    return cpu_json_str;
}