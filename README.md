# Monitor de Sistema MCV
## Descripción
Este proyecto es un monitor de sistema diseñado para capturar y mostrar información relevante del hardware y software del sistema operativo. Utiliza la crate `sysinfo` para obtener datos del sistema y el comando `sensors` para obtener información sobre la temperatura del CPU.
el sistema de archivos del proyecto es el siguiente:
 - src
   - core # carpeta con los modulos principales de los instrumentos para capturar la informacion del sistema
     - cpu.rs
     - discos.rs
     - interfaces.rs
     - memoria.rs
     - procesos.rs
   - output # carpeta con los modulos para exportar la informacion del sistema
     - json.rs
     - log.rs
   - cli # carpeta con los modulos para la interfaz de linea de comandos
     - mad.rs
   - main.rs


# requisitos
- Rust 1.60 o superior
- Crate `sysinfo` para la recolección de datos del sistema.
- Comando `sensors` para la recolección de datos de temperatura del CPU.
- Paquete `lm-sensors` instalado y configurado en el sistema.
- Sistema operativo Linux (probado en Ubuntu 22.04)
- Sistema de 64 bits (x86_64)

# Instalación
para poder instalar el proyecto, primero debes clonar el repositorio:
```bash
git clone https://github.com/marcos9801/monitor_sistema_mcv.git

```
Una vez descargado el repositorio, debes entrar en la carpeta del proyecto:
```bash
cargo install --path .
```
Esto instalará el proyecto en tu sistema. Para ejecutarlo, puedes usar el siguiente comando:
```bash
monitor_sistema_mcv --all
```
Para poder generar los logs, puedes usar el siguiente comando: esto exporta la salida a un archivo de log en la carpeta `logs`:
```bash
monitor_sistema_mcv --all --log
```
para poder hacer que el porgra,a se ejecute periodicamente y pueda estar hgaciendo snapshots de la informacion del sistema, puedes usar el siguiente comando:
este archivo generara un .timer  y un service en el sistema, para que el programa se ejecute cada 5 minutos:
```bash  
#!/bin/bash

# Ruta del ejecutable
BIN_PATH="$HOME/.cargo/bin/monitor_sistema_mcv"

# Directorio de systemd para servicios personalizados
SYSTEMD_DIR="/etc/systemd/system"

# Archivos systemd
SERVICE_FILE="monitor_sistema_mcv.service"
TIMER_FILE="monitor_sistema_mcv.timer"

# Verifica si el binario existe
if [ ! -f "$BIN_PATH" ]; then
  echo "❌ El binario no se encuentra en $BIN_PATH"
  echo "Ejecuta: cargo install --path ."
  exit 1
fi

echo "✅ Binario encontrado: $BIN_PATH"

# Crear archivo de servicio
echo "🛠️  Creando archivo de servicio..."
sudo tee "$SYSTEMD_DIR/$SERVICE_FILE" > /dev/null <<EOF
[Unit]
Description=Ejecuta el monitor del sistema

[Service]
ExecStart=$BIN_PATH
Restart=on-failure
EOF

# Crear archivo de timer
echo "🛠️  Creando archivo de timer..."
sudo tee "$SYSTEMD_DIR/$TIMER_FILE" > /dev/null <<EOF
[Unit]
Description=Ejecutar monitor_sistema_mcv cada 5 minutos

[Timer]
OnBootSec=5min
OnUnitActiveSec=5min
Unit=$SERVICE_FILE

[Install]
WantedBy=timers.target
EOF

# Recargar systemd
echo "🔄 Recargando systemd..."
sudo systemctl daemon-reload

# Habilitar y arrancar el timer
echo "🚀 Habilitando y arrancando el timer..."
sudo systemctl enable --now "$TIMER_FILE"

echo "✅ Timer activado. Puedes verificarlo con:"
echo "   systemctl list-timers --all | grep monitor_sistema_mcv"

```
recargar el demonio de systemd para que reconozca los nuevos archivos de servicio y timer:
```bash
sudo systemctl daemon-reload

```
y habilitar el timer para que se ejecute automáticamente al inicio del sistema:
```bash
sudo systemctl enable monitor_sistema_mcv.timer
```
# Crate `cli `

Este proyecto proporciona una herramienta de línea de comandos (CLI) para mostrar información del sistema, como CPU, memoria, disco, interfaces de red y procesos. Está escrita en Rust y utiliza la librería `clap` para gestionar los argumentos y comandos.

## Descripción

`monitor_sistema_mcv` es una herramienta que permite monitorear el sistema y obtener información detallada sobre:

- **CPU**
- **Memoria**
- **Discos**
- **Interfaces de red**
- **Procesos**

Además, tiene la capacidad de generar un log del sistema en formato JSON.

## Instalación

1. Asegúrate de tener Rust instalado. Si no lo tienes, puedes instalarlo desde [aquí](https://www.rust-lang.org/).
2. Clona el repositorio y navega al directorio del proyecto.
3. Compila y ejecuta el proyecto con los siguientes comandos:

```bash
cargo build
cargo run
```



# Crate `output`
# modulo 'output/json.rs'

Este módulo contiene funciones para exportar la información del sistema a formato JSON. Cada función toma una referencia a una estructura de información del sistema y devuelve una cadena JSON que representa esa información.

## Descripción

El módulo proporciona funciones para exportar la información del sistema (procesos, interfaces de red, discos, memoria y CPU) a formato JSON utilizando la biblioteca `serde_json`.

## Funciones

### `exportar_procesos_json`

Esta función exporta la información sobre los procesos del sistema a formato JSON.

#### Parámetros
- `procesos`: Una referencia a una instancia de `ProcesosInfo`.
# Modulo 'output/log.rs'

Este módulo proporciona funciones para generar y guardar un log del sistema en formato JSON. El log incluye información sobre los procesos, interfaces de red, discos, memoria y CPU del sistema, junto con la fecha, hora y otros detalles relacionados.

## Descripción

La función `generar_log_sistema` crea un archivo de log que contiene los datos del sistema en formato JSON. La información se obtiene utilizando las funciones de exportación a JSON previamente definidas. El log se guarda en una carpeta llamada `logs`, y el nombre del archivo incluye un timestamp con la fecha y hora de la creación del log.

## Funciones

### `generar_log_sistema`

Esta función genera un log del sistema con la información sobre los procesos, interfaces de red, discos, memoria y CPU, y lo guarda en un archivo JSON.

#### Parámetros
- `procesos`: Una cadena JSON que contiene la información de los procesos del sistema.
- `interfaces`: Una cadena JSON que contiene la información de las interfaces de red del sistema.
- `discos`: Una cadena JSON que contiene la información de los discos del sistema.
- `memoria`: Una cadena JSON que contiene la información de la memoria del sistema.
- `cpu`: Una cadena JSON que contiene la información de la CPU del sistema.

# Módulo: `core/discos.rs`

### Descripción general
Este archivo define las estructuras y los métodos relacionados con la obtención de información sobre los discos del sistema. Forma parte del módulo central del proyecto y se encarga de capturar, procesar y mostrar métricas relacionadas con los discos, como el nombre, el sistema de archivos, el espacio total, el espacio libre, el espacio usado, entre otros.

### Estructuras principales

#### Estructura `DiscosInfo`
Contiene información general sobre los discos en el sistema:
- `cantidad_discos`: Número de discos detectados en el sistema.
- `espacio_total`: Espacio total de todos los discos combinados (en GB).
- `espacio_libre`: Espacio libre de todos los discos combinados (en GB).
- `espacio_usado`: Espacio usado de todos los discos combinados (en GB).
- `discos`: Un vector de `DiscoInfo` que almacena información detallada de cada disco.

#### Estructura `DiscoInfo`
Representa un disco específico y contiene detalles sobre su estado y propiedades:
- `nombre`: Nombre del disco.
- `sistema_archivos`: Sistema de archivos del disco.
- `espacio_total`: Espacio total del disco (en GB).
- `espacio_libre`: Espacio libre disponible en el disco (en GB).
- `espacio_usado`: Espacio usado del disco (en GB).
- `ruta`: Ruta de montaje del disco.
- `removible`: Si el disco es removible (valor booleano).
- `solo_lectura`: Si el disco es de solo lectura (valor booleano).

### Métodos

#### Para la estructura `DiscosInfo`:
- **`get_cantidad_discos()`**: Retorna la cantidad de discos detectados.
- **`get_espacio_total()`**: Retorna el espacio total de todos los discos en GB.
- **`get_espacio_libre()`**: Retorna el espacio libre de todos los discos en GB.
- **`get_espacio_usado()`**: Retorna el espacio usado de todos los discos en GB.
- **`get_discos()`**: Retorna la lista de discos como un vector de `DiscoInfo`.
- **`get_disco(index: usize)`**: Retorna el disco en la posición indicada por el índice.

#### Para la estructura `DiscoInfo`:
- **`get_nombre()`**: Retorna el nombre del disco.
- **`get_sistema_archivos()`**: Retorna el sistema de archivos del disco.
- **`get_espacio_total()`**: Retorna el espacio total del disco en GB.
- **`get_espacio_libre()`**: Retorna el espacio libre del disco en GB.
- **`get_espacio_usado()`**: Retorna el espacio usado del disco en GB.
- **`get_ruta()`**: Retorna la ruta de montaje del disco.
- **`get_removible()`**: Retorna si el disco es removible (valor booleano).
- **`get_solo_lectura()`**: Retorna si el disco es de solo lectura (valor booleano).

### Constructor
- **`new()`**: Constructor de la estructura `DiscosInfo`. Obtiene y organiza la información de los discos del sistema usando la crate `sysinfo` y devuelve una instancia con los datos de todos los discos.

### Observaciones
- La estructura `DiscosInfo` incluye un cálculo en tiempo real del espacio total, libre y usado de los discos detectados en el sistema.
- Se utilizan constantes para convertir los valores de espacio de bytes a gigabytes.
- Los discos se filtran para evitar la repetición en caso de múltiples particiones o unidades con el mismo nombre.
- El uso de la crate `sysinfo` permite obtener información actualizada sobre los discos del sistema.

### Funciones adicionales
- **`obtener_info_disco()`**: Función auxiliar para obtener una instancia de `DiscosInfo`, la cual contiene toda la información de los discos del sistema.

### Integración con la estructura del proyecto
Este módulo forma parte de la carpeta `core`, que contiene todos los archivos necesarios para capturar información del sistema de forma modular.

Este archivo es utilizado por los comandos definidos en la CLI del sistema, como:

## Módulo: `core/cpu.rs`

### Descripción general
Este archivo define la estructura y los métodos relacionados con la obtención de información del CPU del sistema. Forma parte del módulo central del proyecto y se encarga de capturar, procesar y mostrar métricas del procesador como su marca, temperatura, frecuencia, número de núcleos y uso individual de cada núcleo.

### Contenido principal

#### Estructura `CPUInfo`
Contiene la información principal del CPU:
- `brand`: Marca del CPU.
- `temperatura`: Vector de cadenas con la temperatura por núcleo.
- `cantidad_nucleos`: Cantidad de núcleos físicos.
- `frecuencia`: Frecuencia actual del CPU (en MHz).
- `uso_nucleos`: Porcentaje de uso por núcleo.

#### Métodos
- `new()`: Constructor. Inicializa los datos del CPU usando la crate `sysinfo` y el comando del sistema `sensors`.
- `mostrar_info()`: Muestra en consola toda la información recolectada del CPU.
- `obtener_temperaturas()`: Ejecuta el comando `sensors` y filtra las temperaturas relevantes de la salida.
- `extraer_temperatura(linea: &str)`: Parsea una línea de salida del comando `sensors` para extraer el nombre del núcleo y su temperatura.
- Getters públicos (`get_brand`, `get_temperatura`, etc.): Permiten acceso seguro a los campos internos de la estructura.
- `obtener_info_cpu()`: Función auxiliar expuesta para otros módulos, que retorna una instancia de `CPUInfo`.

### Observaciones
- Se integran pausas controladas para garantizar datos actualizados del uso del CPU (mínimo 100ms).
- Hay funcionalidades comentadas que permiten la futura expansión a arquitecturas híbridas (núcleos E/P, núcleos lógicos).
- El uso del comando `sensors` puede requerir que el paquete `lm-sensors` esté instalado y configurado correctamente en el sistema.

### Integración con la estructura del proyecto
Este módulo forma parte de la carpeta `core`, que contiene todos los archivos necesarios para capturar información del sistema de forma modular.

Este archivo es utilizado por los comandos definidos en la CLI del sistema, como:
# Módulo: core/interfaces.rs 

Este módulo proporciona una forma de obtener información detallada sobre las interfaces de red de un sistema. Los detalles incluyen:

- Cantidad de interfaces.
- Estadísticas de tráfico (bytes y paquetes transmitidos/recibidos).
- Número total de errores.
- Direcciones IP y MAC por interfaz.
- MTU de cada interfaz.

## Estructuras Principales

### `InterfacesInfo`

Esta estructura representa un resumen de todas las interfaces de red del sistema.

#### Campos:
- `cantidad_interfaces`: Número total de interfaces detectadas.
- `interfaces`: Vector con la información detallada de cada interfaz (`Vec<InterfaceInfo>`).
- `total_errores`: Total de errores en todas las interfaces.
- `total_bytes_recibidos`: Total de bytes recibidos.
- `total_bytes_enviados`: Total de bytes enviados.
- `total_paquetes_recibidos`: Total de paquetes recibidos.
- `total_paquetes_enviados`: Total de paquetes enviados.
- `total_direcciones_ip`: Total de direcciones IP encontradas.
- `total_direcciones_mac`: Total de direcciones MAC encontradas.
- `total_mtu`: Suma de los valores MTU de todas las interfaces.

#### Métodos:
- `new`: Constructor principal que obtiene la información desde el sistema.
- `desde_sistema`: Crea una instancia a partir de una referencia a `Networks`.
- Métodos *getter* para acceder a cada uno de los campos anteriores.

### `InterfaceInfo`

Esta estructura representa la información específica de una sola interfaz de red.

#### Campos:
- `nombre`: Nombre de la interfaz (por ejemplo, "en0").
- `bytes_recibidos`: Total de bytes procesados.
- `bytes_enviados`: Total de bytes enviados.
- `numero_paquetes_recibidos`: Total de paquetes recibidos.
- `numero_paquetes_enviados`: Total de paquetes enviados.
- `total_errores`: Total de errores combinados.
- `total_errores_recibidos`: Errores al recibir datos.
- `total_errores_enviados`: Errores al enviar datos.
- `direccion_ip`: Vector de direcciones IP asociadas (`Vec<IpNetwork>`).
- `direccion_mac`: Dirección MAC de la interfaz.
- `mtu`: Unidad máxima de transmisión.

#### Métodos:
- `new`: Constructor vacío.
- `desde_sistema`: Crea una instancia con datos del sistema.
- Métodos *getter* para acceder a cada uno de los campos.

## Función Principal

### `obtener_info_interfaces`

Esta función obtiene la información actual de las interfaces de red del sistema y devuelve una instancia de `InterfacesInfo` con los detalles.

#### Retorno:
- Una instancia de `InterfacesInfo` con la información obtenida del sistema.

## Ejemplo de Uso

```rust
use sysinfo::{Networks, NetworkExt};

fn main() {
    // Obtiene la información de las interfaces de red
    let info_interfaces = obtener_info_interfaces();

    // Muestra la información de las interfaces
    info_interfaces.mostrar_info();
}

```

# Módulo `core/memoria.rs`

Este módulo proporciona una forma de obtener información detallada sobre la memoria del sistema, incluyendo:

- Memoria RAM total, libre y usada.
- Memoria swap total, libre y usada.

### Estructuras Principales

### `MemoriaInfo`

Esta estructura representa la información detallada sobre el uso de memoria en el sistema.

#### Campos:
- `total`: Memoria total del sistema en MB (RAM + Swap).
- `libre`: Memoria libre del sistema en MB.
- `usada`: Memoria usada del sistema en MB.
- `total_ram`: Memoria RAM total en MB.
- `libre_ram`: Memoria RAM libre en MB.
- `usada_ram`: Memoria RAM usada en MB.
- `swap_total`: Memoria swap total en MB.
- `swap_libre`: Memoria swap libre en MB.
- `swap_usada`: Memoria swap usada en MB.

#### Métodos:
- `get_memoria_total`: Devuelve la memoria total del sistema.
- `get_memoria_libre`: Devuelve la memoria libre del sistema.
- `get_memoria_usada`: Devuelve la memoria usada del sistema.
- `get_total_ram`: Devuelve la memoria RAM total del sistema.
- `get_libre_ram`: Devuelve la memoria RAM libre del sistema.
- `get_usada_ram`: Devuelve la memoria RAM usada del sistema.
- `get_swap_total`: Devuelve la memoria swap total del sistema.
- `get_swap_libre`: Devuelve la memoria swap libre del sistema.
- `get_swap_usada`: Devuelve la memoria swap usada del sistema.
- `get_memoria_total_sistema`: Devuelve la memoria total del sistema (RAM + Swap).
- `new`: Constructor que crea una nueva instancia de `MemoriaInfo`.
- `desde_sistema`: Método auxiliar que construye `MemoriaInfo` desde una instancia de `System`.
- `mostrar_info`: Muestra la información detallada de la memoria en la consola.

##@ Función Principal

### `obtener_info_memoria`

Esta función devuelve una instancia de `MemoriaInfo` con la información actual sobre la memoria del sistema.

#### Retorno:
- Una instancia de `MemoriaInfo` con la información obtenida del sistema.

##@ Ejemplo de Uso

```rust
use sysinfo::{System, RefreshKind};

fn main() {
    // Obtiene la información de la memoria
    let info_memoria = obtener_info_memoria();

    // Muestra la información de la memoria
    info_memoria.mostrar_info();
}
```


# Módulo: `core/procesos.rs`
Este módulo implementa la obtención y análisis de los procesos del sistema. Proporciona información detallada sobre cada proceso, incluyendo su uso de CPU y memoria, tiempo de ejecución, y estado.

## Estructuras Principales

### `ProcesosInfo`

Representa un resumen general del estado de los procesos del sistema.

- `cantidad_procesos`: Número total de procesos detectados.
- `procesos`: Vector con la información detallada de todos los procesos (`Vec<ProcesoInfo>`).
- `top_procesos_uso_cpu`: Top 5 procesos que están consumiendo más CPU actualmente.
- `top_procesos_uso_memoria`: Top 5 procesos que están usando más memoria física.
- `top_procesos_tiempo_cpu`: Top 5 procesos con más tiempo acumulado en CPU.
- `top_procesos_tiempo_ejecucion`: Top 5 procesos con mayor tiempo de ejecución.

#### Métodos de `ProcesosInfo`

- `new()`: Constructor que obtiene la información directamente del sistema.
- `get_cantidad_procesos()`: Devuelve la cantidad total de procesos.
- `get_procesos()`: Devuelve la lista de todos los procesos.
- `get_proceso(index: usize)`: Devuelve el proceso en la posición `index`.
- `get_top_procesos_uso_cpu()`: Devuelve los procesos que consumen más CPU.
- `get_top_procesos_uso_memoria()`: Devuelve los procesos que consumen más memoria.
- `get_top_procesos_tiempo_cpu()`: Devuelve los procesos que más tiempo han estado en CPU.
- `get_top_procesos_tiempo_ejecucion()`: Devuelve los procesos que más tiempo han estado en ejecución.

### `ProcesoInfo`

Representa la información específica de un solo proceso del sistema.

- `pid`: Identificador único del proceso.
- `nombre`: Nombre del proceso.
- `tiempo_ejecucion`: Tiempo total que ha estado en ejecución.
- `tiempo_en_cpu`: Tiempo acumulado que ha estado en CPU.
- `uso_cpu`: Porcentaje de CPU usado actualmente.
- `uso_memoria`: Memoria física usada (en MB).
- `uso_memoria_virtual`: Memoria virtual usada (en MB).
- `estado`: Estado actual del proceso.

#### Métodos de `ProcesoInfo`

- `get_pid()`: Devuelve el PID del proceso.
- `get_nombre()`: Devuelve el nombre del proceso.
- `get_tiempo_ejecucion()`: Devuelve el tiempo total de ejecución.
- `get_tiempo_en_cpu()`: Devuelve el tiempo acumulado en CPU.
- `get_uso_cpu()`: Devuelve el porcentaje de CPU usado.
- `get_uso_memoria()`: Devuelve el uso de memoria física (en MB).
- `get_uso_memoria_virtual()`: Devuelve el uso de memoria virtual (en MB).
- `get_estado()`: Devuelve el estado del proceso.

## Funciones

### `obtener_info_procesos()`

Devuelve una instancia de `ProcesosInfo` con la información actual del sistema.

```rust
let info_procesos = obtener_info_procesos();
