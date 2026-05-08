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

## 🛠️ Cómo Compilar y Ejecutar

Asegúrate de tener [Rust](https://www.rust-lang.org/tools/install) instalado en tu sistema.

1.  **Clona el repositorio:**
    ```bash
    git clone https://github.com/Leandro-Cardozo/spd-ascii-poc.git
    cd spd-ascii-poc
    ```

2.  **Compila el proyecto:**
    ```bash
    cargo build --release
    ```

3.  **Ejecuta el juego:**
    ```bash
    cargo run
    ```

## 🎮 Controles

### Menú Principal
- **Flechas Arriba/Abajo:** Navegar por las opciones.
- **Enter:** Seleccionar una opción.
- **Q / Esc:** Salir del juego.

### Durante el Juego
- **Flechas de Dirección:** Mover al personaje y atacar enemigos.
- **Q / Esc:** Salir del juego.
- **D:** Activar/desactivar el **modo Descartar**.
- **1-9:**
    - **Modo Normal:** Usar el objeto correspondiente del inventario.
    - **Modo Descartar:** Dejar el objeto en el suelo.
- **S / Enter:** Confirmar para descender por las escaleras.
- **N / Esc:** Cancelar el descenso.
- **Clic Izquierdo del Ratón:** Inspeccionar una casilla visible en el mapa.

### Pantalla de Game Over
- **R:** Reiniciar la partida.
- **Q / Esc:** Salir del juego.

## 📂 Estructura del Proyecto

El código fuente está organizado en los siguientes módulos:

-   `main.rs`: Contiene el bucle principal del juego, gestiona la inicialización de la terminal y el manejo de eventos (teclado, ratón, redimensionamiento).
-   `app.rs`: Define las estructuras de datos principales (`App`, `Entity`, `GameState`), la lógica de generación del mapa y las mecánicas centrales del juego (movimiento, combate, FOV, uso de objetos).
-   `ui.rs`: Se encarga de renderizar la interfaz de juego principal, incluyendo el mapa, las estadísticas del jugador, el inventario y el historial de mensajes. También dibuja la pantalla de "Game Over".
-   `title.rs`: Renderiza la pantalla de título y el menú principal del juego.

## 📚 Dependencias

-   [`ratatui`](https://crates.io/crates/ratatui): Para la creación de la interfaz de usuario en la terminal.
-   [`crossterm`](https://crates.io/crates/crossterm): Para el manejo de eventos y manipulación de la terminal.
-   [`rand`](https://crates.io/crates/rand) y [`rand_chacha`](https://crates.io/crates/rand_chacha): Para la generación de números aleatorios (usado en la creación de mazmorras y cálculo de daño).
