use sysinfo::{Networks, IpNetwork};

pub struct InterfacesInfo {
    cantidad_interfaces: u64,
    interfaces: Vec<InterfaceInfo>,
    total_errores: u64,
    total_bytes_recibidos: u64,
    total_bytes_enviados: u64,
    total_paquetes_recibidos: u64,
    total_paquetes_enviados: u64,
    total_direcciones_ip: u64,
    total_direcciones_mac: u64,
    total_mtu: u64,
}

pub struct InterfaceInfo {
    nombre: String,
    bytes_recibidos: u64,
    bytes_enviados: u64,
    numero_paquetes_recibidos: u64,
    numero_paquetes_enviados: u64,
    total_errores: u64,
    total_errores_recibidos: u64,
    total_errores_enviados: u64,
    direccion_ip: Vec<IpNetwork>,
    direccion_mac: String,
    mtu: u64, // tamaño máximo de la unidad de transmisión
}

impl InterfacesInfo {
    // Getters
    pub fn get_cantidad_interfaces(&self) -> u64 { self.cantidad_interfaces }
    pub fn get_interfaces(&self) -> &Vec<InterfaceInfo> { &self.interfaces }
    pub fn get_total_errores(&self) -> u64 { self.total_errores }
    pub fn get_bytes_recibidos(&self) -> u64 { self.total_bytes_recibidos }
    pub fn get_bytes_enviados(&self) -> u64 { self.total_bytes_enviados }
    pub fn get_numero_paquetes_recibidos(&self) -> u64 { self.total_paquetes_recibidos }
    pub fn get_numero_paquetes_enviados(&self) -> u64 { self.total_paquetes_enviados }
    pub fn get_direccion_ip(&self) -> u64 { self.total_direcciones_ip }
    pub fn get_direccion_mac(&self) -> u64 { self.total_direcciones_mac }
    pub fn get_mtu(&self) -> u64 { self.total_mtu }
    pub fn get_interfaz(&self, index: u64) -> &InterfaceInfo {
        if index < self.interfaces.len().try_into().unwrap() {
            &self.interfaces[index as usize]
        } else {
            panic!("Índice fuera de rango");
        }
    }
    //pub fn get_errores_recibidos(&self) -> u64 { self.total_errores_recibidos }
    //pub fn get_interface(&self, index: usize) -> &InterfaceInfo {}

    pub fn new() -> Self {
        let interfaces = Networks::new_with_refreshed_list();
        Self::desde_sistema(&interfaces)
    }

    pub fn desde_sistema(interfaces: &Networks) -> Self {
        let mut cantidad_interfaces = 0;
        let mut total_errores = 0;
        let mut total_bytes_recibidos = 0;
        let mut total_bytes_enviados = 0;
        let mut total_paquetes_recibidos = 0;
        let mut total_paquetes_enviados = 0;
        let mut total_direcciones_ip = 0;   
        let mut total_direcciones_mac = 0;
        let mut total_mtu = 0;


        let mut interfaces_vec = Vec::new();
        for (interface_name, network) in interfaces {
            // Verifica si la interfaz ya existe en el vector
            cantidad_interfaces += 1;
            total_errores += network.total_errors_on_received() + network.total_errors_on_transmitted() ;//network.rx_errors() + network.tx_errors();
            total_bytes_recibidos += network.total_received();
            total_bytes_enviados += network.total_transmitted();
            total_paquetes_recibidos += network.total_packets_received();
            total_paquetes_enviados += network.total_packets_transmitted();
            total_direcciones_ip += network.ip_networks().len() as u64; 
            if network.mac_address().to_string() != "00:00:00:00:00:00" {
                total_direcciones_mac += 1;
            }
            total_mtu += network.mtu(); // Asumiendo que cada interfaz tiene un MTU
            // Crea una nueva instancia de InterfaceInfo            
            let interface = InterfaceInfo::desde_sistema(
                interface_name.to_string(),
                network.total_received(),
                network.total_received(),
                network.total_packets_received(),
                network.total_packets_transmitted(),
                network.total_errors_on_received() + network.total_errors_on_transmitted() ,
                network.total_errors_on_received(),
                network.total_errors_on_transmitted(),
                network.ip_networks().to_vec(),
                network.mac_address().to_string(),
                network.mtu(),
            );
            interfaces_vec.push(interface); // ¡usas un Vec aquí!
        }
            InterfacesInfo {
                cantidad_interfaces,
                interfaces: interfaces_vec,
                total_errores,
                total_bytes_recibidos,
                total_bytes_enviados,
                total_paquetes_recibidos,
                total_paquetes_enviados,
                total_direcciones_ip,
                total_direcciones_mac,
                total_mtu,
            }
    }
}

impl InterfaceInfo {
    // Getters
    pub fn get_nombre(&self) -> &String { &self.nombre }
    pub fn get_bytes_recibidos(&self) -> u64 { self.bytes_recibidos }
    pub fn get_bytes_enviados(&self) -> u64 { self.bytes_enviados }
    pub fn get_numero_paquetes_recibidos(&self) -> u64 { self.numero_paquetes_recibidos }
    pub fn get_numero_paquetes_enviados(&self) -> u64 { self.numero_paquetes_enviados }
    pub fn get_total_errores(&self) -> u64 { self.total_errores }
    pub fn get_total_errores_recibidos(&self) -> u64 { self.total_errores_recibidos }
    pub fn get_total_errores_enviados(&self) -> u64 { self.total_errores_enviados }
    pub fn get_direccion_ip(&self) -> &Vec<IpNetwork> { &self.direccion_ip }
    pub fn get_direccion_mac(&self) -> &String { &self.direccion_mac }
    pub fn get_mtu(&self) -> u64 { self.mtu }

    pub fn new() -> Self {
        InterfaceInfo {
            nombre: String::new(),
            bytes_recibidos: 0,
            bytes_enviados: 0,
            numero_paquetes_recibidos: 0,
            numero_paquetes_enviados: 0,
            total_errores: 0,
            total_errores_recibidos: 0,
            total_errores_enviados: 0,
            direccion_ip: Vec::<IpNetwork>::new(),
            direccion_mac: String::new(),
            mtu: 0,
        }
    }

    pub fn desde_sistema(nombre: String, bytes_recibidos: u64, bytes_enviados: u64, numero_paquetes_recibidos: u64, numero_paquetes_enviados: u64, total_errores: u64, total_errores_recibidos: u64, total_errores_enviados: u64, direccion_ip: Vec<IpNetwork>, direccion_mac: String, mtu: u64) -> Self {
        InterfaceInfo {
            nombre,
            bytes_recibidos,
            bytes_enviados,
            numero_paquetes_recibidos,
            numero_paquetes_enviados,
            total_errores,
            total_errores_recibidos,
            total_errores_enviados,
            direccion_ip,
            direccion_mac,
            mtu,
        }
    }
}

pub fn obtener_info_interfaces() -> InterfacesInfo {
    InterfacesInfo::new()
}
