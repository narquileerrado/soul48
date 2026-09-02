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

- **Un Descenso en Cuatro Tramos:** Los 48 pisos se dividen en **Las Criptas** (1-12), **Las Catacumbas** (13-24), **El Abismo** (25-36) y **El Silencio** (37-48). Cada tramo tiene su paleta, su pool de criaturas, sus susurros y el Guardián que lo cierra. Un jefe cada seis pisos, y el Archidemonio del Silencio esperando en el 48.
- **Victoria y Permadeath:** Matar al Archidemonio termina la corrida. La partida se guarda sola al cambiar de piso, pero morir —o ganar— disuelve el fragmento: no se recarga el piso anterior para volver a intentarlo.
- **Generación Procedural de Mazmorras:** Cada nivel es único, con habitaciones, túneles y entidades distribuidas procedimentalmente.
- **Sistema de Visión (FOV):** El mapa se revela a medida que exploras, manteniendo las áreas visitadas en un tono tenue.
- **Entidades Narrativas Interactivas:**
  - **Paredes Parlantes (`W`):** Murallas antiguas que susurran fragmentos de lore, advertencias y secretos.
  - **Altáres de Ecos (`A`):** Estructuras místicas donde puedes ofrecer un pacto de sangre (5 HP) para revelar la totalidad del mapa del nivel.
- **Retratos 8-bit:** Cada criatura del Compendio tiene su retrato, y el menú principal y el fin de partida tienen ilustración. Se dibujan con medio bloque (`▀`, U+2580): cada celda pinta dos píxeles verticales, el de arriba en color de frente y el de abajo en color de fondo, así que los píxeles quedan cuadrados y no se pierde ningún color. La rampa de tonos de cada retrato sale del color que la criatura ya tiene en el mapa.
- **Sintonizar Alma:** Pantalla de ajustes con brillo de lo recordado, líneas del historial, glifos unicode/ASCII y guardado automático. Se guardan en `settings.json`.
- **Sistema de Guardado y Carga de Partida:** Persistencia completa del estado del juego (`savegame.json`). Tu progreso se guarda automáticamente al salir y puedes reanudarlo en cualquier momento desde el menú principal (*RECOGER FRAGMENTOS*).
- **Combate por Turnos:** Ataca a los enemigos moviéndote hacia ellos. El daño sale de tu arma más tu Fuerza, menos la defensa del enemigo; el suyo se descuenta con tu armadura y tu casco, y tu Agilidad te da una chance de esquivar.
- **Enemigos que Doblan la Esquina:** Un campo de flujo calculado por BFS desde el héroe: los mobs rodean los muros en vez de trabarse contra ellos, y las puertas cerradas les cortan el paso igual que a vos. Dormidos, errantes, agresivos, cobardes y estáticos.
- **Efectos de Estado en los Dos Sentidos:** Veneno, sangrado, quemadura, confusión y ceguera. La Serpiente envenena, el Heraldo te apaga la vista, y un enemigo envenenado se muere solo.
- **Once Criaturas y Cinco Jefes** documentados en el Compendio, con las mismas estadísticas que te vas a cruzar en el mapa.
- **Sistema de Inventario:** Recoge pociones, armas, llaves y otros objetos. Úsalos o descártalos según necesites.
- **Entidades Interactivas:** Cofres cerrados que requieren llaves y escaleras para descender de piso.
- **Persistencia entre Niveles:** Al descender conservás todo: salud, cordura, nivel, experiencia, atributos, las cinco ranuras de equipo y el inventario. Lo único que cambia es el piso.
- **La Cordura Importa:** El medidor de voz baja solo con los turnos —tu Voluntad frena la caída— y por debajo del umbral la penumbra empieza a torcerte los pasos. En cero, el silencio te come el alma turno a turno.
- **Salas Especiales con Contenido:** La Armería guarda armas y coraza, la Biblioteca pergaminos, y el Círculo Ritual un amuleto que cuesta caro alcanzar.
- **Interfaz Gráfica en Terminal:** Construida con `ratatui`, ofrece una experiencia de juego clara y organizada.
- **Soporte para Ratón:** Clic izquierdo en casillas visibles para inspeccionar entidades e información ambiental.

## 📋 Requisitos

Para compilar y ejecutar este proyecto, asegúrate de tener instalado lo siguiente:

-   **Rust:** Se recomienda la versión estable (1.70 o superior). Puedes instalarlo desde [rust-lang.org](https://www.rust-lang.org/tools/install).
-   **Cargo:** El gestor de paquetes de Rust (incluido con la instalación de Rust).
-   **Herramientas de compilación:**
    -   **Linux:** `build-essential` (o equivalente como `base-devel`).
    -   **macOS:** Xcode Command Line Tools (`xcode-select --install`).
    -   **Windows:** [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) con la carga de trabajo de C++.
-   **Git:** Para clonar el repositorio.

## 🖥️ Fuente recomendada

El juego no elige la fuente: la elige tu terminal. Para que los muros, las barras y los
retratos se vean como corresponde, la fuente necesita **box drawing** (`U+2500–257F`) y
**block elements** (`U+2580–259F`).

- **JetBrainsMono Nerd Font** — cubre todo lo necesario y es la opción segura.
- Para el look 8-bit de verdad: **Cozette**, **unscii-16** (pensada para arte ASCII) o
  **PxPlus IBM VGA 8x16** del *Ultimate Oldschool PC Font Pack*, que es literalmente la
  fuente de los roguelikes de DOS.
- **Poné el interlineado en 1.0.** Cualquier espacio extra entre líneas abre huecos
  horizontales entre las filas de los retratos y arruina el efecto. Es el ajuste que más
  importa.

Si tu fuente no tiene los glifos de caja, entrá a **SINTONIZAR ALMA** y poné `GLIFOS` en
`ascii`: el mapa vuelve a `#` y `.`, las barras a `#` y `-`, y los retratos a caracteres
planos, todo conservando el mismo tamaño en pantalla.

## 🛠️ Cómo Compilar y Ejecutar

Sigue estos pasos para poner en marcha el juego:

1.  **Clona el repositorio:**
    ```bash
    git clone https://github.com/narquileerrado/soul48.git
    cd soul48
    ```

2.  **Compila el proyecto:**
    ```bash
    cargo build --release
    ```
    *Nota: El ejecutable se generará en `target/release/soul48`.*

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
- **S / Enter:** Confirmar para descender por las escaleras (cuando se te pregunte). La partida se guarda sola al bajar.
- **N / Esc:** Cancelar el descenso.
- **Clic Izquierdo del Ratón:** Inspeccionar una casilla visible en el mapa para obtener información en el historial.

### Bestiario
- **Flechas Arriba/Abajo:** Seleccionar una criatura.
- **Q / Esc:** Volver al menú principal.

### Pantalla de Game Over
- **R:** Reiniciar la partida.
- **Q / Esc:** Salir del juego.

## 📂 Estructura del Proyecto

El crate expone una biblioteca (`src/lib.rs`) y un binario delgado, para que los
tests de integración de `tests/` puedan armar partidas completas.

-   `main.rs`: Punto de entrada. Prende y apaga la terminal, dibuja la pantalla que corresponde al estado y despacha cada tecla. Nada más.
-   `game/`: El núcleo de la lógica, un archivo por sistema: `mod.rs` (estado y turno), `interaction.rs` (choques contra entidades), `combat.rs`, `inventory.rs`, `ai.rs`, `fov.rs`, `map.rs` y `save.rs`.
-   `ui/`: El dibujo, un archivo por pantalla: `widgets.rs` (piezas compartidas), `juego.rs`, `compendio.rs` y `opciones.rs`.
-   `map_builder.rs`: Responsable de la generación procedimental de los niveles. Implementa el algoritmo de excavación de habitaciones y túneles, así como la colocación aleatoria de enemigos y objetos.
-   `bestiary.rs`: El catálogo de criaturas. Es la **única** fuente: de acá salen los mobs que genera el mapa, los jefes y las fichas del Compendio, así que lo que leés en el compendio es lo que te vas a cruzar.
-   `world/tramo.rs`: Los cuatro tramos del descenso, con su paleta, sus voces y su Guardián.
-   `game/pathing.rs`: El campo de flujo que usan los enemigos para encontrarte.
-   `title.rs`: Se encarga exclusivamente de la lógica y presentación de la pantalla de título y el menú principal.
-   `player.rs`: El héroe: sus números, sus atributos, sus cinco ranuras de equipo y todo lo que de ellos se deriva.
-   `balance.rs`: Los números que definen cómo se siente el juego, agrupados por tema. El balance se ajusta acá sin leer la lógica.
-   `input.rs`: Traducción de teclas a intenciones, una por pantalla.
-   `menus.rs`: Los cursores de las pantallas con lista.
-   `settings.rs`: Los ajustes del jugador y su persistencia en `settings.json`.
-   `theme.rs`: El sistema de color. Cada color tiene un significado y uno solo.
-   `sprite.rs` y `arte.rs`: Los retratos 8-bit y cómo se dibujan con medio bloque.

## 🧪 Tests

```bash
cargo test                                   # todo
cargo fmt --check && cargo clippy -- -D warnings
```

-   `tests/basicos.rs`: mecánicas puntuales.
-   `tests/mecanicas.rs`: escenarios de varios turnos sobre una sala controlada (`App::arena`), reproducibilidad de la semilla y una partida larga que verifica invariantes.
-   `tests/pantallas.rs`: render de cada pantalla sobre el backend de prueba de `ratatui`, incluida una terminal de 40x15, y que cada tramo se vea distinto.

Entre ellos hay un descenso completo hasta el piso 48 que verifica que los 48 pisos se generen, que ninguno salga desierto y que los cuatro Guardianes aparezcan donde corresponde.

## 📚 Dependencias

-   [`ratatui`](https://crates.io/crates/ratatui): Para la creación de la interfaz de usuario en la terminal.
-   [`crossterm`](https://crates.io/crates/crossterm): Para el manejo de eventos y manipulación de la terminal.
-   [`rand`](https://crates.io/crates/rand) y [`rand_chacha`](https://crates.io/crates/rand_chacha): Para la generación de números aleatorios (usado en la creación de mazmorras y cálculo de daño).
