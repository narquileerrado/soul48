# 📋 Soul 48: The Talking Dead — Hoja de Ruta y TODO

## 🗣️ 1. Concepto Principal: "The Talking Dead"
- [x] **Paredes Parlantes:** Susurran secretos y pistas. Bloquean la visión. Cada tramo tiene sus propias voces.
- [x] **Altáres de Ecos:** Sacrificás HP a cambio de revelar el piso entero.
- [x] **Diálogos con Espíritus:** Se puede calmar al *Ladrón de Ecos* pagando cordura.
- [x] **Sistema de Cordura:** Baja sola con los turnos, la Voluntad la frena, por debajo del umbral te tuerce los pasos y en cero te come el alma.

## ⚔️ 2. Profundidad Táctica en el Combate
- [x] **Ataques a Distancia y Magia:** Rayo, Bola de Fuego, Teletransporte e Invisibilidad. Matan de verdad y dan experiencia.
- [x] **Habilidades Activas:** Embestida (`E`) y Bloqueo / Parry (`B`).
- [x] **Efectos de Estado en los dos sentidos:** Veneno, Sangrado, Quemadura, Confusión y Ceguera, sobre el héroe y sobre los enemigos. Cada criatura declara el suyo en el catálogo.
- [x] **Enemigos que doblan la esquina:** campo de flujo por BFS. Antes se trababan contra los muros y pararse en un recodo era invulnerabilidad gratis.
- [ ] **Ataques a distancia sin pergamino** (arco, honda): hoy toda arma es cuerpo a cuerpo. El *Coro de Lamentos* es estático y pega de lejos, pero el héroe no tiene equivalente.
- [ ] **`Freeze` sigue sin dueño:** es la única variante de efecto de estado que nadie aplica.

## 🗺️ 3. Variedad de Terreno y Generación Procedural
- [x] **Cuatro tramos de doce pisos**, con paleta, pool de criaturas, susurros y Guardián propios.
- [x] **Peligros Ambientales con efecto:** pinchos, ácido, fuego y aceite —que ahora resbala, o prende si hay fuego al lado.
- [x] **Salas Especiales con contenido:** Armería (armas y coraza), Biblioteca (pergaminos) y Círculo Ritual (amuleto y fuego).
- [x] **Sistemas de Puertas:** madera, cerradas con llave y pasajes secretos. Las cerradas bloquean la visión y el paso de los enemigos.
- [x] **Ningún piso desierto:** la aparición sala por sala dejaba uno de cada diez sin una sola criatura.
- [ ] **Variedad de trazado:** todos los pisos son habitaciones rectangulares unidas por túneles en L. Faltan cuevas, pasillos largos, salas circulares. Es lo que más se nota al bajar seguido.
- [ ] **La dificultad escala sólo por profundidad** (`(depth - 1) * 2` sobre las estadísticas base). Los tramos podrían tener su propia curva.

## 🛡️ 4. Progresión del Personaje y Equipamiento
- [x] **Cinco ranuras de equipo** con efecto real: armadura y casco descuentan daño, el anillo suma fuerza, el amuleto sube el techo de cordura.
- [x] **Atributos con efecto:** Fuerza suma daño, Agilidad da esquiva, Voluntad frena el desgaste de cordura.
- [x] **Persistencia entre pisos:** nivel, experiencia, atributos, equipo e inventario cruzan la escalera intactos.
- [ ] **Meta-progresión entre partidas / Árbol de Bendiciones.** No existe: al morir se pierde todo y el permadeath lo hace definitivo. Es el próximo milestone natural.
- [ ] **El inventario son nueve ranuras y las teclas 1-9.** Con once criaturas y más objetos, empieza a quedar chico.

## 🎨 5. Feedback Visual y Pulido
- [x] **Paleta por tramo:** los muros y el suelo cambian cada doce pisos, y lo recordado hereda el tono solo.
- [x] **Efectos Visuales:** el marco del mapa se tiñe al recibir daño; el historial tiene un color por tipo de voz.
- [x] **Modo ASCII completo** para fuentes sin glifos de caja.
- [x] **Vida de los enemigos a la vista** en el panel LO QUE TE RODEA.
- [ ] **Faltan seis retratos 8-bit:** Rata, Osario, Sombra, Devorador, Coro y Heraldo, más los cinco jefes. `arte::de_criatura` devuelve `Option` y el Compendio cae al formato sin retrato, así que se agregan de a uno sin bloquear nada.
- [ ] **Audio.** No hay ninguna dependencia de audio en `Cargo.toml`. Es trabajo por hacer, no estructura lista.
- [ ] **Animaciones de golpe** y desplazamiento suave.

## 👾 6. Jefes y Final
- [x] **Un jefe cada seis pisos:** los de fin de tramo llevan su nombre, los de mitad son un eco más flojo.
- [x] **Los jefes están en el Compendio** con las mismas estadísticas que aparecen en el mapa.
- [x] **El Piso 48:** el Archidemonio del Silencio, y matarlo termina la corrida con pantalla de victoria.
- [ ] **El Archidemonio pelea como cualquier otro mob.** Se merece una mecánica propia: fases, invocar Heraldos, apagar el mapa.

## 🧹 7. Deuda técnica conocida
- [ ] **`Vec<Vec<char>>` como mapa.** Los accesos pasan todos por `game/map.rs`, pero un tipo `Grid` cerraría la puerta a volver a indexar a mano.
- [ ] **Los peligros son permanentes y se disparan cada vez** que pisás la casilla. Falta decidir si es intencional.
- [ ] **`interact_with_entity` sigue teniendo 300 líneas** y un `match` de ocho brazos. Cada brazo podría ser su propia función.
- [ ] **El compendio no dice en qué tramo vive cada criatura**, que ahora es información útil.
