# 📋 Soul 48: The Talking Dead - Hoja de Ruta y TODO

## 🗣️ 1. Concepto Principal: "The Talking Dead"
- [x] **Paredes Parlantes (Talking Walls):** Susurran secretos y pistas al interactuar. Bloquean la visión.
- [x] **Altáres de Ecos (Echo Altars):** Sacrificás HP a cambio de revelar el piso entero.
- [x] **Diálogos con Espíritus y Mobs:** Se puede calmar al *Ladrón de Ecos* pagando cordura.
- [x] **Sistema de Cordura / Medidor de Voz:** Baja sola con los turnos, la Voluntad la frena, por debajo del umbral te tuerce los pasos y en cero te come el alma.

## ⚔️ 2. Profundidad Táctica en el Combate
- [x] **Ataques a Distancia y Magia:** Pergaminos de Rayo, Bola de Fuego, Teletransporte e Invisibilidad. Matan de verdad y dan experiencia.
- [x] **Habilidades Activas:** Embestida (`E`) y Bloqueo / Parry (`B`).
- [x] **Efectos de Estado sobre el héroe:** Veneno y Quemadura, con daño por turno y duración.
- [ ] **Efectos de Estado sobre los enemigos.** `Entity.status_effects` existe y nadie lo usa; `Bleed`, `Freeze`, `Confusion` y `Blindness` están declarados y no se aplican a nadie. O se cablean, o se recortan las variantes.
- [ ] **Ataques a distancia sin pergamino** (arco, honda): hoy toda arma es cuerpo a cuerpo.

## 🗺️ 3. Variedad de Terreno y Generación Procedural
- [x] **Peligros Ambientales:** Pinchos, ácido, aceite y fuego.
- [x] **Salas Especiales con contenido:** Armería (armas y coraza), Biblioteca (pergaminos) y Círculo Ritual (amuleto y fuego).
- [x] **Sistemas de Puertas:** Madera, cerradas con llave y pasajes secretos. Las cerradas bloquean la visión.
- [x] **Sin entidades apiladas:** nada queda tapado por otra cosa ni encima de la escalera.
- [ ] **El Charco de Aceite no hace nada** salvo escribir una línea en el historial. Le falta el efecto: resbalar, o prenderse con el fuego de al lado.
- [ ] **Variedad de trazado:** hoy todos los pisos son habitaciones rectangulares unidas por túneles en L. Faltan cuevas, pasillos largos, salas circulares.

## 🛡️ 4. Progresión del Personaje y Equipamiento
- [x] **Ranuras de Equipo Expandidas:** Armadura, Casco, Anillo y Amuleto, todas con efecto real.
- [x] **Atributos con efecto:** Fuerza suma daño, Agilidad da esquiva, Voluntad frena el desgaste de cordura.
- [x] **Persistencia entre pisos:** nivel, experiencia, atributos, equipo e inventario cruzan la escalera intactos.
- [ ] **Meta-progresión entre partidas / Árbol de Bendiciones.** No existe: al morir se pierde todo. Lo que hay es persistencia entre *pisos* de la misma partida.

## 🎨 5. Feedback Visual y Pulido (Juice & Polish)
- [x] **Efectos Visuales en Terminal:** El marco del mapa se tiñe al recibir daño; el historial tiene un color por tipo de voz.
- [x] **Retratos 8-bit** por criatura, con rampa de tonos derivada de su color en el mapa.
- [x] **Modo ASCII completo** para fuentes sin glifos de caja.
- [x] **Vida de los enemigos a la vista** en el panel LO QUE TE RODEA.
- [ ] **Audio.** No hay ninguna dependencia de audio en `Cargo.toml` ni módulo que lo prepare. Es trabajo por hacer, no estructura lista.
- [ ] **Animaciones de golpe** y desplazamiento suave.

## 👾 6. Jefes Finales e Identidad por Pisos
- [x] **Jefes de Piso:** Guardianes cada 5 niveles, escalados por profundidad.
- [x] **El Piso 48:** El Archidemonio del Silencio.
- [ ] **Identidad visual por tramo de pisos:** hoy los 48 pisos se ven igual; sólo cambian los números.
- [ ] **Los jefes no están en el Compendio** y tienen su experiencia en una tabla aparte de `bestiary::xp_de`.

## 🧹 7. Deuda técnica conocida
- [ ] **`Vec<Vec<char>>` como mapa.** Los accesos ya pasan todos por `game/map.rs`, pero un tipo `Grid` propio evitaría que se pueda volver a indexar a mano.
- [ ] **El teletransporte puede fallar en silencio:** si tras 100 intentos no encuentra suelo, consume el pergamino y no pasa nada.
- [ ] **Los peligros son permanentes y se disparan cada vez** que pisás la casilla. Habría que decidir si es intencional.
