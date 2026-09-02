# Soul 48: The Talking Dead

```text
   ▄▀▀▀▄ ▄▀▀▀▄ █   █ █          ▄█  ▄▀▀▀▄
   ▀▄▄▄  █   █ █   █ █        ▄▀ █  ▀▄▄▄▀
   ▄   █ █   █ █   █ █        ▀▀▀█▀ █   █
    ▀▀▀   ▀▀▀   ▀▀▀  ▀▀▀▀▀       ▀   ▀▀▀

              t h e   t a l k i n g   d e a d
```

**Desocupado lector:** sin juramento me podrás creer que quisiera que aqueste juego,
como hijo del entendimiento, fuera el más hermoso que imaginarse pudiera. Es **Soul 48**
una prueba de ingenio —*roguelike* le dicen los que del arte entienden— labrada en Rust
para que corra en la terminal desnuda, sin más pinturas que las que caben en una celda
de letra. De la librería `ratatui` se vale para pintar cuanto se ve, que no es poco
siendo tan poco con lo que cuenta.

## ❦ Argumento de la obra

> Despierta vuesa merced en el umbral, y no halla voz con que quejarse. No es ya sino
> eco de aquel que fue, ánima atada a cuerpo que ha dejado de respirar. Cuarenta y ocho
> sótanos más abajo aguarda el Archidemonio del Silencio, que guarda lo que le quitó y
> se ríe de que no pueda nombrarlo. Para cobrar su voz y su ventura, menester es bajar
> hasta él. Mas tenga aviso: en aqueste dominio hasta las paredes tienen algo que decir,
> y la muerte no es sino el comienzo de otra plática.

## ❦ De las cosas notables que en esta obra se contienen

**Del descenso, repartido en cuatro tramos.** Los cuarenta y ocho sótanos se dividen en
**Las Criptas** (1-12), **Las Catacumbas** (13-24), **El Abismo** (25-36) y **El Silencio**
(37-48). Cada tramo tiene su color, sus criaturas, sus susurros y el Guardián que lo cierra.
Hállase un jefe cada seis pisos, y en el postrero aguarda el Archidemonio.

**De la victoria y de la muerte, que es sin remedio.** Quien matare al Archidemonio acaba
la jornada y la acaba bien. Guárdase la partida sola al mudar de piso; empero, morir —o
vencer— deshace el fragmento, que no se torna al sótano anterior para probar otra vez.

**De cómo se labran los sótanos.** Ninguno es igual a otro: aposentos, túneles y criaturas
se reparten por suerte, y por la misma suerte se apartan, que no queden dos cosas en una
casilla ni nada encima de la escalera.

**De la vista, que es corta.** Descúbrese el mapa a medida que vuesa merced camina, y lo
ya andado queda en un tono apagado, que la memoria no alumbra tanto como el ojo.

**De las presencias que hablan.** Las **Paredes de los Lamentos** (`W`) susurran secretos,
avisos y mentiras, y cada tramo tiene las suyas. Los **Altares de Ecos** (`A`) revelan el
piso entero a quien les ofrezca cinco puntos de su sangre.

**De los retratos.** Cada criatura del Compendio tiene el suyo, y las dos pantallas de
cierre su ilustración. Dibújanse con medio bloque (`▀`, U+2580): pinta cada celda dos
píxeles verticales, el de arriba en color de frente y el de abajo en el de fondo, con que
los píxeles salen cuadrados y no se pierde color ninguno. La rampa de tonos sale del mismo
color que la criatura tiene en el mapa. Por la misma traza está hecho el título.

**Del relato, que va rodando.** Bajo el título pasa la historia del difunto de derecha a
izquierda, sin descanso: léela entera quien se quedare, y no ocupa sitio a quien no.

**Del combate, que es por turnos.** Acomete vuesa merced al enemigo caminando hacia él. Sale
el daño de su arma más su Fuerza, menos la defensa del contrario; y el que recibe se descuenta
con la armadura y el yelmo, que su Agilidad le da esperanza de esquivarlo.

**De los enemigos, que doblan la esquina.** Calcúlase un campo de flujo desde vuesa merced,
con que las criaturas rodean los muros en lugar de trabarse contra ellos; y las puertas
cerradas les cortan el paso lo mesmo que a vuesa merced. Háylas dormidas, errantes,
agresivas, cobardes y quedas.

**De los males que se pegan, y van en ambas direcciones.** Veneno, sangre, quemadura,
confusión y ceguera. La Serpiente emponzoña, el Heraldo apaga la vista, y criatura
emponzoñada se muere sola.

**De once criaturas y cinco jefes**, todos asentados en el Compendio con las mesmas
cuentas que vuesa merced habrá de hallar en el mapa, que no le mienta el libro.

**De la cordura, que importa y mucho.** Baja sola con los turnos —su Voluntad detiene la
caída— y por debajo del umbral la penumbra le empieza a torcer los pasos. En llegando a
cero, el silencio le come el ánima turno a turno.

**De los aposentos señalados.** La Armería guarda armas y coraza, la Biblioteca pergaminos,
y el Círculo Ritual un amuleto que cuesta caro alcanzar.

**De lo que se lleva y lo que queda.** Al bajar conserva vuesa merced todo: salud, cordura,
nivel, experiencia, atributos, las cinco piezas de equipo y cuanto carga. Lo único que muda
es el sótano.

**De la hacienda.** Recoja pociones, armas, llaves y lo demás; úselo o déjelo caer según le
convenga. Los cofres cerrados piden llave, y las escaleras, decisión.

**De sintonizar el ánima.** Pantalla de ajustes con el brillo de lo recordado, las líneas
del historial, los glifos unicode o llanos y el guardado. Asiéntanse en `settings.json`.

**Del ratón.** Clic izquierdo sobre casilla visible, y dirásele a vuesa merced qué hay en ella.

## ❦ De lo que es menester tener antes de comenzar

-   **Rust:** 1.87 o superior (usa el código `is_multiple_of`, que allí se asentó).
    Hállase en [rust-lang.org](https://www.rust-lang.org/tools/install).
-   **Cargo:** que viene con Rust y no se pide aparte.
-   **Herramientas de fábrica:**
    -   **Linux:** `build-essential`, o su equivalente `base-devel`.
    -   **macOS:** las Xcode Command Line Tools (`xcode-select --install`).
    -   **Windows:** las [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
        con la carga de C++.
-   **Git:** para traer el repositorio.

## ❦ De la letra y del molde

No escoge el juego la letra: escógela la terminal de vuesa merced. Para que los muros, las
barras y los retratos salgan como es debido, ha menester la letra de **box drawing**
(`U+2500–257F`) y de **block elements** (`U+2580–259F`).

- **JetBrainsMono Nerd Font** — cúbrelo todo, y es la elección segura.
- Para el aire de ocho bits verdadero: **Cozette**, **unscii-16** (pensada para arte de
  caracteres) o **PxPlus IBM VGA 8x16** del *Ultimate Oldschool PC Font Pack*, que es
  literalmente la letra de los roguelikes de DOS.
- **Póngase el interlineado en 1.0.** Cualquier holgura de más entre renglones abre huecos
  entre las filas de los retratos y da al traste con el efecto. Es el ajuste que más monta.

Si la letra de vuesa merced careciere de los glifos de caja, entre en **SINTONIZAR ALMA** y
ponga `GLIFOS` en `ascii`: tórnase el mapa a `#` y `.`, las barras a `#` y `-`, y los
retratos a caracteres llanos, guardando todo el mesmo tamaño en pantalla.

## ❦ De cómo se ha de fabricar y poner en obra

1.  **Tráigase el repositorio:**
    ```bash
    git clone https://github.com/narquileerrado/soul48.git
    cd soul48
    ```

2.  **Fabríquese:**
    ```bash
    cargo build --release
    ```
    *El ejecutable queda en `target/release/soul48`.*

3.  **Póngase en obra:**
    ```bash
    cargo run --release
    ```
    *Puede asimismo llamarse al binario derechamente, una vez fabricado.*

## ❦ Del gobierno de las teclas

### En el menú principal
- **Flechas arriba y abajo:** andar por las opciones.
- **Enter:** escoger.
- **Q / Esc:** volver al silencio.

### Durante la jornada
- **Flechas:** caminar, y acometer al enemigo caminando hacia él.
- **E:** embestida, que aparta al contrario de un empujón.
- **B:** bloqueo, que parte en dos el golpe que viniere.
- **D:** entrar y salir del **modo descartar**.
- **1-9:**
    - **De ordinario:** usar o vestir la pieza que corresponda.
    - **En modo descartar:** dejarla en el suelo.
- **S / Enter:** confirmar el descenso cuando se le preguntare. Guárdase la partida sola.
- **N / Esc:** quedarse en el piso.
- **Q / Esc:** cerrar el juego.
- **Clic izquierdo:** mirar una casilla visible, y sabráse qué hay en ella.

### En el Compendio
- **Flechas arriba y abajo:** escoger criatura.
- **Q / Esc:** tornar al menú principal.

### En las pantallas de cierre
Al caer (**FIN DE LA PARTIDA**) o al vencer al Archidemonio (**EL ORIGEN**):
- **R:** emprender otra bajada desde el primer sótano.
- **Q / Esc:** salir.

En ambos casos se deshace el fragmento guardado: la jornada es de ida.

## ❦ De la traza y repartimiento de los papeles

Expone el crate una biblioteca (`src/lib.rs`) y un binario delgado, para que las pruebas de
`tests/` puedan armar partidas enteras.

-   `main.rs`: la puerta. Enciende y apaga la terminal, pinta la pantalla que toca al estado
    y reparte cada tecla. Nada más.
-   `game/`: el corazón de la lógica, un fichero por sistema: `mod.rs` (estado y turno),
    `interaction.rs` (los choques contra las cosas), `combat.rs`, `inventory.rs`, `ai.rs`,
    `pathing.rs` (el campo de flujo con que las criaturas hallan a vuesa merced), `fov.rs`,
    `map.rs` y `save.rs`.
-   `ui/`: el dibujo, un fichero por pantalla: `widgets.rs` (las piezas comunes), `juego.rs`,
    `compendio.rs`, `opciones.rs` y `final_.rs` (las dos de cierre).
-   `world/`: cómo está hecho cada sótano. `map_builder.rs` cava aposentos y túneles y reparte
    lo que en ellos vive; `tramo.rs` asienta los cuatro tramos, con su color, sus voces y su
    Guardián.
-   `bestiary.rs`: el catálogo de las criaturas. Es la **única** fuente: de aquí salen las que
    el mapa engendra, los jefes y las fichas del Compendio, con que lo que se lee en el libro
    es lo que se topa en el camino.
-   `title.rs`: la pantalla de título, el menú y el relato que va rodando.
-   `player.rs`: el difunto —sus cuentas, sus atributos, sus cinco piezas de equipo y cuanto
    de ellas se deriva.
-   `balance.rs`: los números que gobiernan cómo se siente el juego, juntos y por materias.
    Ajústase el balance aquí sin leer la lógica.
-   `input.rs`: la traducción de teclas a intenciones, una por pantalla.
-   `menus.rs`: los cursores de las pantallas con lista.
-   `settings.rs`: los ajustes y su asiento en `settings.json`.
-   `theme.rs`: el sistema de color. Cada color tiene un significado, y uno solo.
-   `sprite.rs` y `arte.rs`: los retratos de ocho bits y la traza con que se dibujan.

## ❦ De las pruebas y experiencias

```bash
cargo test                                   # todas
cargo fmt --check && cargo clippy --all-targets -- -D warnings
```

-   `tests/basicos.rs`: mecánicas sueltas, los tramos y los retratos.
-   `tests/mecanicas.rs`: lances de varios turnos sobre una sala gobernada (`App::arena`), la
    fidelidad de la semilla y una partida larga que vigila que nada se desmande.
-   `tests/pantallas.rs`: el dibujo de cada pantalla sobre el backend de prueba de `ratatui`,
    incluida una terminal de 40x15, y que cada tramo se vea distinto del anterior.

Hállase entre ellas un descenso entero hasta el sótano cuarenta y ocho, que comprueba que los
cuarenta y ocho se labren, que ninguno salga desierto y que los cuatro Guardianes aparezcan
donde deben.

## ❦ De a quién se deben las herramientas

-   [`ratatui`](https://crates.io/crates/ratatui): para levantar la interfaz en la terminal.
-   [`crossterm`](https://crates.io/crates/crossterm): para el gobierno de la terminal y sus sucesos.
-   [`rand`](https://crates.io/crates/rand) y [`rand_chacha`](https://crates.io/crates/rand_chacha):
    para la suerte, que labra los sótanos y reparte los golpes.
-   [`serde`](https://crates.io/crates/serde) y [`serde_json`](https://crates.io/crates/serde_json):
    para asentar la partida y los ajustes en disco.

---

*Vale.*
