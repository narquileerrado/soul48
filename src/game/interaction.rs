//! Qué pasa cuando el héroe choca contra algo.
//!
//! Una función por familia de entidad; `interact_with_entity` sólo despacha.

use super::*;
use crate::balance;
use rand::Rng;

impl App {
    /// Si hay una casilla con fuego pegada a esta.
    fn hay_fuego_junto_a(&self, pos: Point) -> bool {
        self.entities.iter().any(|e| {
            matches!(
                e.e_type,
                EntityType::Hazard {
                    hazard_type: HazardType::Fire
                }
            ) && (e.pos.x as isize - pos.x as isize).abs() <= 1
                && (e.pos.y as isize - pos.y as isize).abs() <= 1
        })
    }

    /// Gestiona la interacción física o lógica con una entidad en el mapa.
    pub(super) fn interact_with_entity(&mut self, index: usize) -> (bool, bool) {
        let mut entity_clone = self.entities[index].clone();
        let mut move_allowed = true;
        // el aceite empuja un paso más en la misma dirección
        let mut resbala = false;
        let mut entity_index_to_remove = None;
        // casi todo choque consume el turno; los que no hacen nada, no
        let mut accion_real = true;

        match &mut entity_clone.e_type {
            EntityType::Mob {
                hp,
                state,
                defense,
                pacified,
                ..
            } => {
                if *pacified {
                    self.add_log(
                        format!("> {} os dexa passar en paz.", entity_clone.name),
                        LogType::Info,
                    );
                    move_allowed = false;
                } else if crate::bestiary::es_negociable(&entity_clone.name)
                    && *state != EnemyState::Aggressive
                    && self.player.sanity >= balance::cordura::MINIMA_PARA_NEGOCIAR
                {
                    // Oportunidad de negociación pacífica con el espíritu / espíritu errante
                    *pacified = true;
                    self.player.sanity -= balance::cordura::COSTO_NEGOCIACION;
                    self.add_log(
                        format!(
                            "> CONCIERTO: Aplacáis al {} dándole un pedaço de vuestra voz (-{} de seso).",
                            entity_clone.name,
                            balance::cordura::COSTO_NEGOCIACION
                        ),
                        LogType::Info,
                    );
                    self.entities[index] = entity_clone;
                    move_allowed = false;
                } else {
                    let (min_d, max_d) = self.player.equipment.dano_arma();
                    // la fuerza por encima de la base es el brazo detrás del arma
                    let mut damage = self.rng.gen_range(min_d..=max_d) + self.player.brazo();
                    damage = (damage - *defense).max(balance::combate::DANO_MINIMO);

                    if self.rng.gen_bool(balance::combate::PROB_CRITICO) {
                        damage *= balance::combate::MULT_CRITICO;
                        self.add_log(
                            format!("> ¡GOLPE CERTERO: {} de daño!", damage),
                            LogType::Combat,
                        );
                    } else {
                        self.add_log(
                            format!("> {} de daño a {}.", damage, entity_clone.name),
                            LogType::Combat,
                        );
                    }
                    *hp -= damage;
                    *state = EnemyState::Aggressive;
                    // La muerte no se resuelve acá: `reap_dead` la centraliza,
                    // así el rayo y la embestida matan igual que el cuerpo a cuerpo.
                    self.entities[index] = entity_clone;
                    move_allowed = false;
                }
            }
            EntityType::Chest { locked } => {
                move_allowed = false;
                if *locked {
                    if let Some(k_idx) = self
                        .inventory
                        .iter()
                        .position(|(i, _)| i.name == crate::bestiary::LLAVE)
                    {
                        if self.inventory[k_idx].1 > 1 {
                            self.inventory[k_idx].1 -= 1;
                        } else {
                            self.inventory.remove(k_idx);
                        }

                        self.add_log("> Abrís el cofre con la llave.".into(), LogType::Info);
                        let dmg_bonus = self.depth as i32;

                        self.entities[index] = Entity {
                            pos: entity_clone.pos,
                            glyph: '/',
                            color: crate::theme::AZUL_ALMA,
                            name: format!("Espada +{}", dmg_bonus),
                            e_type: EntityType::Weapon {
                                min_dmg: 3 + dmg_bonus,
                                max_dmg: 8 + dmg_bonus,
                            },
                            status_effects: Vec::new(),
                        };
                    } else {
                        self.add_log(
                            "> Cerrado está el cofre: menester es la llave.".into(),
                            LogType::Warning,
                        );
                    }
                }
            }
            EntityType::TalkingWall { message, whispered } => {
                move_allowed = false;
                if !*whispered {
                    let texto = message.clone();
                    *whispered = true;
                    self.entities[index] = entity_clone;
                    self.add_log(
                        format!("PARED DE LOS LAMENTOS \u{00ab}{}\u{00bb}", texto),
                        LogType::Whisper,
                    );
                } else {
                    self.add_log("Calla ya la pared.".into(), LogType::Whisper);
                }
            }
            EntityType::EchoAltar { used } => {
                move_allowed = false;
                if !*used {
                    if self.player.hp > balance::terreno::COSTO_ALTAR {
                        self.player.hp -= balance::terreno::COSTO_ALTAR;
                        *used = true;
                        self.entities[index] = entity_clone;
                        self.add_log(
                            format!(
                                "> PACTO DE SANGRE: Ofrecéis {} de ánima al Altar de los Ecos.",
                                balance::terreno::COSTO_ALTAR
                            ),
                            LogType::Warning,
                        );
                        self.add_log("> DESCÚBRESE EL SÓTANO ANTE VOS.".into(), LogType::Info);

                        // Revela todo el mapa explorado
                        let (map_height, map_width) = self.dimensiones();
                        for y in 0..map_height {
                            for x in 0..map_width {
                                self.explored[y][x] = true;
                            }
                        }
                    } else {
                        self.add_log(
                            "> Flaca anda vuestra ánima para ofrecer sangre.".into(),
                            LogType::Warning,
                        );
                    }
                } else {
                    self.add_log(
                        "> Consumió ya el Altar de los Ecos su tributo.".into(),
                        LogType::Info,
                    );
                }
            }
            EntityType::Door {
                locked,
                secret,
                open,
            } => {
                if *open {
                    move_allowed = true;
                } else if *locked {
                    move_allowed = false;
                    if let Some(k_idx) = self
                        .inventory
                        .iter()
                        .position(|(i, _)| i.name == crate::bestiary::LLAVE)
                    {
                        if self.inventory[k_idx].1 > 1 {
                            self.inventory[k_idx].1 -= 1;
                        } else {
                            self.inventory.remove(k_idx);
                        }
                        *locked = false;
                        *open = true;
                        entity_clone.glyph = '\'';
                        entity_clone.name = "Puerta Abierta".into();
                        self.add_log(
                            "> Franqueáis y abrís la puerta con la llave.".into(),
                            LogType::Info,
                        );
                        self.entities[index] = entity_clone;
                    } else {
                        self.add_log(
                            "> Cerrada está la puerta con llave.".into(),
                            LogType::Warning,
                        );
                    }
                } else {
                    move_allowed = false;
                    let is_sec = *secret;
                    *open = true;
                    entity_clone.glyph = '\'';
                    entity_clone.name = if is_sec {
                        "Passadizo Descubierto".into()
                    } else {
                        "Puerta Abierta".into()
                    };
                    self.add_log(
                        if is_sec {
                            "> ¡Descubrís un passadizo encubierto!".into()
                        } else {
                            "> Abrís la puerta.".into()
                        },
                        LogType::Info,
                    );
                    self.entities[index] = entity_clone;
                }
            }
            EntityType::Hazard { hazard_type } => {
                move_allowed = true;
                match hazard_type {
                    HazardType::Spikes => {
                        self.player.hp = (self.player.hp - balance::terreno::PINCHOS).max(0);
                        self.add_log(
                            format!(
                                "> TRAMPA DE PÚAS: ¡Padecéis {} de daño!",
                                balance::terreno::PINCHOS
                            ),
                            LogType::Warning,
                        );
                    }
                    HazardType::Acid => {
                        let (dano, turnos, por_turno) = balance::terreno::ACIDO;
                        self.player.hp = (self.player.hp - dano).max(0);
                        self.player.status_effects.push(StatusEffect {
                            effect_type: StatusEffectType::Poison,
                            duration: turnos,
                            damage_per_turn: por_turno,
                        });
                        self.add_log(
                            format!(
                                "> POÇO DE ÁCIDO: ¡Padecéis {} de daño y quedáis emponçoñado!",
                                dano
                            ),
                            LogType::Warning,
                        );
                    }
                    HazardType::Oil => {
                        // Antes esto era sólo una línea en el historial. Ahora
                        // el aceite hace lo que anuncia: o te arrastra un paso
                        // más, o se prende si hay fuego al lado.
                        let ardiendo = self.hay_fuego_junto_a(entity_clone.pos);
                        if ardiendo {
                            let (dano, turnos, por_turno) = balance::terreno::FUEGO;
                            self.player.hp = (self.player.hp - dano).max(0);
                            self.player.status_effects.push(StatusEffect {
                                effect_type: StatusEffectType::Burn,
                                duration: turnos,
                                damage_per_turn: por_turno,
                            });
                            self.add_log(
                                format!(
                                    "> ¡PRENDE EL AZEITE! Padecéis {} de daño y os quemáis.",
                                    dano
                                ),
                                LogType::Warning,
                            );
                        } else {
                            resbala = true;
                            self.add_log(
                                "> CHARCO DE AZEITE: Resbaláis, y os lleváis un passo de más."
                                    .into(),
                                LogType::Info,
                            );
                        }
                    }
                    HazardType::Fire => {
                        let (dano, turnos, por_turno) = balance::terreno::FUEGO;
                        self.player.hp = (self.player.hp - dano).max(0);
                        self.player.status_effects.push(StatusEffect {
                            effect_type: StatusEffectType::Burn,
                            duration: turnos,
                            damage_per_turn: por_turno,
                        });
                        self.add_log(
                            format!("> FUEGO: ¡Padecéis {} de daño y os quemáis!", dano),
                            LogType::Warning,
                        );
                    }
                }
            }
            EntityType::SpecialRoomMarker { room_type } => {
                move_allowed = true;
                match room_type {
                    SpecialRoomType::Armory => {
                        self.add_log(
                            "> LA ARMERÍA: Huele el aire a metal templado.".into(),
                            LogType::Info,
                        );
                    }
                    SpecialRoomType::Library => {
                        self.add_log(
                            "> LA LIBRERÍA: Reposan pergaminos arcanos en los estantes.".into(),
                            LogType::Info,
                        );
                    }
                    SpecialRoomType::RitualCircle => {
                        self.add_log(
                            "> CÍRCULO RITUAL: Corre por vos un frío de fuerça escura.".into(),
                            LogType::Warning,
                        );
                    }
                }
            }
            EntityType::Item
            | EntityType::Key
            | EntityType::Weapon { .. }
            | EntityType::Scroll { .. }
            | EntityType::Armor { .. }
            | EntityType::Helmet { .. }
            | EntityType::Ring { .. }
            | EntityType::Amulet { .. } => {
                let is_stackable = matches!(
                    entity_clone.e_type,
                    EntityType::Item | EntityType::Key | EntityType::Scroll { .. }
                );

                let stack_index = if is_stackable {
                    self.inventory
                        .iter()
                        .position(|(i, _)| i.name == entity_clone.name)
                } else {
                    None
                };

                if let Some(idx) = stack_index {
                    self.inventory[idx].1 += 1;
                    let new_count = self.inventory[idx].1;
                    self.add_log(
                        format!("> Tomáis {} (x{}).", entity_clone.name, new_count),
                        LogType::Item,
                    );
                    entity_index_to_remove = Some(index);
                } else if self.inventory.len() < balance::objetos::SLOTS_INVENTARIO {
                    self.add_log(format!("> Tomáis {}.", entity_clone.name), LogType::Item);
                    self.inventory.push((entity_clone, 1));
                    entity_index_to_remove = Some(index);
                } else {
                    self.add_log("> No cabe más en vuestras manos.".into(), LogType::Warning);
                    move_allowed = false;
                    // no pasó nada: cobrar un turno por eso regalaba
                    // movimientos gratis a los enemigos
                    accion_real = false;
                }
            }
        }

        if let Some(i) = entity_index_to_remove {
            self.entities.remove(i);
        }

        self.resbalon_pendiente = resbala;
        (move_allowed, accion_real)
    }
}
