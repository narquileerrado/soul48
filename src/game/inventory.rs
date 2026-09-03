//! Inventario: usar, equipar y soltar.

use super::*;
use crate::balance;
use rand::Rng;

impl App {
    /// Aplica el efecto de un objeto del inventario o lo equipa.
    pub fn use_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let item = self.inventory[index].0.clone();
        let mut item_used = false;
        // las ranuras de equipo consumen el objeto antes de guardar el viejo,
        // para que el slot que se libera cuente al chequear si hay lugar
        let mut ya_consumido = false;

        match item.e_type {
            EntityType::Item => {
                if item.name == crate::bestiary::POCION {
                    self.player.hp =
                        (self.player.hp + balance::objetos::CURA_POCION).min(self.player.max_hp);
                    self.add_log("> Sentíos mejorado.".into(), LogType::Item);
                    item_used = true;
                } else {
                    // antes esto era un silencio: la tecla no hacía nada y
                    // tampoco decía por qué
                    self.add_log(
                        format!("> {} no sirve de nada: sólo pesa.", item.name),
                        LogType::Warning,
                    );
                }
            }
            EntityType::Scroll { ref scroll_type } => match scroll_type {
                ScrollType::Lightning => {
                    let mut hit_msgs = Vec::new();
                    for entity in &mut self.entities {
                        if matches!(entity.e_type, EntityType::Mob { .. }) {
                            let dist = (self.player.pos.x as isize - entity.pos.x as isize).abs()
                                + (self.player.pos.y as isize - entity.pos.y as isize).abs();
                            if dist <= balance::objetos::RAYO.1 {
                                if let EntityType::Mob { ref mut hp, .. } = entity.e_type {
                                    *hp -= balance::objetos::RAYO.0;
                                    hit_msgs.push(format!(
                                        "> RAYO: ¡Dais en {} con {} de daño!",
                                        entity.name,
                                        balance::objetos::RAYO.0
                                    ));
                                }
                            }
                        }
                    }
                    if hit_msgs.is_empty() {
                        self.add_log(
                            "> Chisporrotea el pergamino del rayo sin blanco a que tirar.".into(),
                            LogType::Warning,
                        );
                    } else {
                        for msg in hit_msgs {
                            self.add_log(msg, LogType::Combat);
                        }
                    }
                    item_used = true;
                }
                ScrollType::Fireball => {
                    let mut hit_msgs = Vec::new();
                    for entity in &mut self.entities {
                        if matches!(entity.e_type, EntityType::Mob { .. }) {
                            let dist = (self.player.pos.x as isize - entity.pos.x as isize).abs()
                                + (self.player.pos.y as isize - entity.pos.y as isize).abs();
                            if dist <= balance::objetos::BOLA_DE_FUEGO.1 {
                                if let EntityType::Mob { ref mut hp, .. } = entity.e_type {
                                    *hp -= balance::objetos::BOLA_DE_FUEGO.0;
                                    hit_msgs.push(format!(
                                        "> ¡Padece {} {} de daño por fuego!",
                                        entity.name,
                                        balance::objetos::BOLA_DE_FUEGO.0
                                    ));
                                }
                            }
                        }
                    }
                    self.add_log(
                        "> BOLA DE FUEGO: ¡Rebienta el fuego aquí cerca!".into(),
                        LogType::Combat,
                    );
                    for msg in hit_msgs {
                        self.add_log(msg, LogType::Combat);
                    }
                    item_used = true;
                }
                ScrollType::Teleport => {
                    // con `thread_rng` la semilla que muestra la interfaz
                    // dejaba de describir la partida
                    let (alto, ancho) = self.dimensiones();
                    let mut destino = None;
                    for _ in 0..balance::objetos::INTENTOS_TELEPORT {
                        let rx = self.rng.gen_range(1..ancho - 1);
                        let ry = self.rng.gen_range(1..alto - 1);
                        if self.es_suelo(Point::new(rx, ry)) {
                            destino = Some(Point::new(rx, ry));
                            break;
                        }
                    }
                    match destino {
                        Some(p) => {
                            self.player.pos = p;
                            self.add_log(
                                "> TRASLADO: Desvanecéos, y aparecéis en otra parte.".into(),
                                LogType::Info,
                            );
                            self.calculate_fov();
                            item_used = true;
                        }
                        // antes el pergamino se gastaba igual y no pasaba nada
                        None => self.add_log(
                            "> No halla el pergamino dónde dexaros: en vuestra mano queda.".into(),
                            LogType::Warning,
                        ),
                    }
                }
                ScrollType::Invisibility => {
                    self.player.invisible_turns = balance::objetos::TURNOS_INVISIBLE;
                    self.add_log(
                        format!(
                            "> INVISIBILIDAD: Tórnase vuestro cuerpo transparente por {} turnos.",
                            balance::objetos::TURNOS_INVISIBLE
                        ),
                        LogType::Info,
                    );
                    item_used = true;
                }
            },
            EntityType::Weapon { min_dmg, max_dmg } => {
                let viejo = self.player.equipment.weapon.take().map(|w| {
                    Self::objeto_equipo(
                        &w.0,
                        EntityType::Weapon {
                            min_dmg: w.1,
                            max_dmg: w.2,
                        },
                    )
                });
                self.player.equipment.weapon = Some((item.name.clone(), min_dmg, max_dmg));
                self.cambiar_equipo(index, viejo, "", &item.name);
                ya_consumido = true;
                item_used = true;
            }
            EntityType::Armor { defense } => {
                let viejo = self
                    .player
                    .equipment
                    .armor
                    .take()
                    .map(|a| Self::objeto_equipo(&a.0, EntityType::Armor { defense: a.1 }));
                self.player.equipment.armor = Some((item.name.clone(), defense));
                self.cambiar_equipo(index, viejo, "armadura ", &item.name);
                ya_consumido = true;
                item_used = true;
            }
            EntityType::Helmet { defense } => {
                let viejo = self
                    .player
                    .equipment
                    .helmet
                    .take()
                    .map(|h| Self::objeto_equipo(&h.0, EntityType::Helmet { defense: h.1 }));
                self.player.equipment.helmet = Some((item.name.clone(), defense));
                self.cambiar_equipo(index, viejo, "casco ", &item.name);
                ya_consumido = true;
                item_used = true;
            }
            EntityType::Ring { stat_bonus } => {
                let viejo = self
                    .player
                    .equipment
                    .ring
                    .take()
                    .map(|r| Self::objeto_equipo(&r.0, EntityType::Ring { stat_bonus: r.1 }));
                self.player.equipment.ring = Some((item.name.clone(), stat_bonus));
                self.cambiar_equipo(index, viejo, "anillo ", &item.name);
                ya_consumido = true;
                item_used = true;
            }
            EntityType::Amulet { sanity_bonus } => {
                let viejo =
                    self.player.equipment.amulet.take().map(|a| {
                        Self::objeto_equipo(&a.0, EntityType::Amulet { sanity_bonus: a.1 })
                    });
                self.player.equipment.amulet = Some((item.name.clone(), sanity_bonus));
                self.cambiar_equipo(index, viejo, "amuleto ", &item.name);
                // sacarse un amuleto puede bajar el techo de cordura
                self.player.sanity = self.player.sanity.min(self.player.max_sanity_total());
                ya_consumido = true;
                item_used = true;
            }
            _ => {
                self.add_log(
                    format!("> No sabéis qué hazer con {}.", item.name),
                    LogType::Warning,
                );
            }
        }

        if item_used {
            if !ya_consumido {
                self.consumir_inventario(index);
            }
            // un pergamino o una embestida también matan
            self.reap_dead();
            return true;
        }
        false
    }

    /// Reconstruye la entidad de un objeto que estaba equipado, para poder
    /// devolverlo al inventario o dejarlo en el suelo.
    fn objeto_equipo(nombre: &str, e_type: EntityType) -> Entity {
        let (glyph, color) = match e_type {
            EntityType::Weapon { .. } => ('/', crate::theme::AZUL_ALMA),
            EntityType::Armor { .. } => ('[', crate::theme::HUESO),
            EntityType::Helmet { .. } => ('^', crate::theme::HUESO),
            EntityType::Ring { .. } => ('=', crate::theme::HUESO),
            EntityType::Amulet { .. } => ('"', crate::theme::VIOLETA),
            _ => ('?', crate::theme::HUESO),
        };
        Entity {
            pos: Point::new(0, 0),
            glyph,
            color,
            name: nombre.to_string(),
            e_type,
            status_effects: Vec::new(),
        }
    }

    /// Descuenta una unidad del slot indicado del inventario.
    fn consumir_inventario(&mut self, index: usize) {
        if self.inventory[index].1 > 1 {
            self.inventory[index].1 -= 1;
        } else {
            self.inventory.remove(index);
        }
    }

    /// Guarda un objeto en el inventario, o lo deja a tus pies si no entra.
    ///
    /// Antes esto era un `push` directo: con el inventario lleno aparecía un
    /// slot 10 que ninguna tecla podía alcanzar.
    fn guardar_o_soltar(&mut self, mut item: Entity) {
        if self.inventory.len() < balance::objetos::SLOTS_INVENTARIO {
            item.pos = Point::new(0, 0);
            self.inventory.push((item, 1));
        } else {
            self.add_log(
                format!("> No os cabe {}: a vuestros pies queda.", item.name),
                LogType::Warning,
            );
            item.pos = self.player.pos;
            self.entities.push(item);
        }
    }

    /// Cambia una ranura de equipo: consume el objeto nuevo del inventario y
    /// se ocupa del que estaba puesto.
    fn cambiar_equipo(
        &mut self,
        index: usize,
        viejo: Option<Entity>,
        etiqueta: &str,
        nombre: &str,
    ) {
        self.consumir_inventario(index);
        if let Some(e) = viejo {
            self.guardar_o_soltar(e);
        }
        self.add_log(format!("> Vestís {}{}.", etiqueta, nombre), LogType::Info);
    }

    /// Descarta un objeto del inventario en la posición actual del héroe.
    pub fn drop_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }

        // cualquier cosa recogible ocupa el suelo: antes se podía tirar una
        // armadura encima de otra y la de abajo desaparecía de la vista
        if self
            .entities
            .iter()
            .any(|e| e.pos == self.player.pos && Self::es_recogible(&e.e_type))
        {
            self.add_log("> Ya hay cosa en el suelo aquí.".into(), LogType::Warning);
            return false;
        }

        let mut item = self.inventory[index].0.clone();
        item.pos = self.player.pos;

        if self.inventory[index].1 > 1 {
            self.inventory[index].1 -= 1;
            self.add_log(format!("> Dexáis un(a) {}.", item.name), LogType::Info);
        } else {
            self.inventory.remove(index);
            self.add_log(format!("> Dexáis {}.", item.name), LogType::Info);
        }

        self.entities.push(item);
        true
    }
}
