# Soul 48: The Talking Dead

```text
 @@@@@@    @@@@@@   @@@  @@@  @@@                       @@@    @@@@@@   
@@@@@@@   @@@@@@@@  @@@  @@@  @@@                      @@@@   @@@@@@@@  
!@@       @@!  @@@  @@!  @@@  @@!                     @@!@!   @@!  @@@  
!@!       !@!  @!@  !@!  @!@  !@!                    !@!!@!   !@!  @!@  
!!@@!!    @!@  !@!  @!@  !@!  @!!       @!@!@!@!@   @!! @!!    !@!!@!   
 !!@!!!   !@!  !!!  !@!  !!!  !!!       !!!@!@!!!  !!!  !@!    !!@!!!   
     !:!  !!:  !!!  !!:  !!!  !!:                  :!!:!:!!:  !!:  !!!  
    !:!   :!:  !:!  :!:  !:!   :!:                 !:::!!:::  :!:  !:!  
:::: ::   ::::: ::  ::::: ::   :: ::::                  :::   ::::: ::  
:: : :     : :  :    : :  :   : :: : :                  :::    : :  :
```

**Soul 48** es una prueba de concepto (PoC) de un juego *roguelike* para terminal, desarrollado en Rust. Inspirado en la exploración de mazmorras y el combate por turnos, este proyecto utiliza la biblioteca `ratatui` para renderizar una interfaz de texto dinámica y atractiva.

> Despiertas en la penumbra del piso 48. No eres más que un eco de quien fuiste, un alma atada a un cuerpo que ya no respira. El demonio que te arrebató la vida te observa desde las profundidades, burlándose de tu silencio. Para recuperar tu voz y tu destino, debes ascender. Pero ten cuidado: en este dominio, hasta las paredes tienen algo que decir, y la muerte es solo el comienzo de una nueva conversación.

## 🚀 Características

- **Generación Procedural de Mazmorras:** Cada nivel es único, con habitaciones y pasillos generados aleatoriamente.
- **Sistema de Visión (FOV):** El mapa se revela a medida que exploras, manteniendo las áreas visitadas en un tono oscuro.
- **Combate por Turnos:** Ataca a los enemigos moviéndote hacia ellos. El daño se calcula basado en tu arma equipada.
- **Enemigos con IA Simple:** Los enemigos pueden estar dormidos, deambular o volverse agresivos cuando te ven.
- **Sistema de Inventario:** Recoge pociones, armas, llaves y otros objetos. Úsalos o descártalos según necesites.
- **Entidades Interactivas:** Encuentra cofres cerrados que requieren llaves y escaleras para descender al siguiente nivel.
- **Persistencia entre Niveles:** Tu salud, inventario y equipo se conservan al bajar de piso.
- **Interfaz Gráfica en Terminal:** Construida con `ratatui`, ofrece una experiencia de juego clara y organizada.
- **Soporte para Ratón:** Haz clic en una casilla visible para inspeccionarla y obtener información.

## 📋 Requisitos

Para compilar y ejecutar este proyecto, asegúrate de tener instalado lo siguiente:

-   **Rust:** Se recomienda la versión estable (1.70 o superior). Puedes instalarlo desde [rust-lang.org](https://www.rust-lang.org/tools/install).
-   **Cargo:** El gestor de paquetes de Rust (incluido con la instalación de Rust).
-   **Herramientas de compilación:**
    -   **Linux:** `build-essential` (o equivalente como `base-devel`).
    -   **macOS:** Xcode Command Line Tools (`xcode-select --install`).
    -   **Windows:** [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) con la carga de trabajo de C++.
-   **Git:** Para clonar el repositorio.

## 🛠️ Cómo Compilar y Ejecutar

Sigue estos pasos para poner en marcha el juego:

1.  **Clona el repositorio:**
    ```bash
    git clone https://github.com/narquileerrado/soul48.git
    cd spd-ascii-poc
    ```

2.  **Compila el proyecto:**
    ```bash
    cargo build --release
    ```
    *Nota: El ejecutable se generará en `target/release/spd_ascii_poc`.*

3.  **Ejecuta el juego:**
    ```bash
    cargo run --release
    ```
    *También puedes ejecutar el binario directamente una vez compilado.*

- **Bestiario:** Consulta información detallada y el trasfondo narrativo de las criaturas que encuentres en el Compendio.

## 🎮 Controles

### Menú Principal
- **Flechas Arriba/Abajo:** Navegar por las opciones.
- **Enter:** Seleccionar una opción.
- **Q / Esc:** Salir del juego.

### Durante el Juego
- **Flechas de Dirección:** Mover al personaje y atacar enemigos.
- **Q / Esc:** Salir al menú principal (desde el Bestiario) o cerrar el juego.
- **D:** Activar/desactivar el **modo Descartar**.
- **1-9:**
    - **Modo Normal:** Usar o equipar el objeto correspondiente del inventario.
    - **Modo Descartar:** Dejar el objeto en el suelo.
- **S / Enter:** Confirmar para descender por las escaleras (cuando se te pregunte).
- **N / Esc:** Cancelar el descenso.
- **Clic Izquierdo del Ratón:** Inspeccionar una casilla visible en el mapa para obtener información en el historial.

### Bestiario
- **Flechas Arriba/Abajo:** Seleccionar una criatura.
- **Q / Esc:** Volver al menú principal.

### Pantalla de Game Over
- **R:** Reiniciar la partida.
- **Q / Esc:** Salir del juego.

## 📂 Estructura del Proyecto

El código fuente está organizado en los siguientes módulos:

-   `main.rs`: Punto de entrada de la aplicación. Gestiona el bucle principal, la inicialización de la terminal (Crossterm) y el despacho de eventos de teclado y ratón.
-   `app.rs`: Define el núcleo de la lógica del juego, incluyendo el estado global (`App`), sistemas de combate, movimiento, inventario y gestión de entidades.
-   `map_builder.rs`: Responsable de la generación procedimental de los niveles. Implementa el algoritmo de excavación de habitaciones y túneles, así como la colocación aleatoria de enemigos y objetos.
-   `bestiary.rs`: Contiene las definiciones y descripciones narrativas de todas las criaturas del juego, integrando datos mecánicos con el trasfondo del mundo.
-   `ui.rs`: Gestiona el renderizado visual utilizando `ratatui`. Dibuja el mapa, la interfaz lateral (HUD), los menús de inventario y la pantalla de fin de juego.
-   `title.rs`: Se encarga exclusivamente de la lógica y presentación de la pantalla de título y el menú principal.

## 📚 Dependencias

-   [`ratatui`](https://crates.io/crates/ratatui): Para la creación de la interfaz de usuario en la terminal.
-   [`crossterm`](https://crates.io/crates/crossterm): Para el manejo de eventos y manipulación de la terminal.
-   [`rand`](https://crates.io/crates/rand) y [`rand_chacha`](https://crates.io/crates/rand_chacha): Para la generación de números aleatorios (usado en la creación de mazmorras y cálculo de daño).
