//! El turno de todo lo que no sos vos.

use super::*;
use crate::balance;
use rand::Rng;

impl App {
    /// Procesa la lógica de turno para todas las entidades enemigas (IA).
    pub fn process_enemy_turns(&mut self) {
        let hx = self.player.pos.x as isize;
        let hy = self.player.pos.y as isize;
        let mut messages = Vec::new();

        // Procesar efectos de estado del héroe
        let mut i = 0;
        while i < self.player.status_effects.len() {
            let dmg = self.player.status_effects[i].damage_per_turn;
            if dmg > 0 {
                self.player.hp = (self.player.hp - dmg).max(0);
                self.add_log(
                    format!("> Padecéis {} de daño del mal que lleváis.", dmg),
                    LogType::Warning,
                );
            }
            self.player.status_effects[i].duration -= 1;
            if self.player.status_effects[i].duration == 0 {
                self.player.status_effects.remove(i);
            } else {
                i += 1;
            }
        }

        // Los mismos efectos, del otro lado. `Entity.status_effects` estaba
        // declarado y serializado desde siempre y no lo tickeaba nadie: un
        // enemigo envenenado no se moría nunca.
        for entidad in &mut self.entities {
            if entidad.status_effects.is_empty() {
                continue;
            }
            let mut dano = 0;
            entidad.status_effects.retain_mut(|ef| {
                dano += ef.damage_per_turn.max(0);
                ef.duration = ef.duration.saturating_sub(1);
                ef.duration > 0
            });
            if dano > 0 {
                if let EntityType::Mob { ref mut hp, .. } = entidad.e_type {
                    *hp -= dano;
                }
            }
        }
        // lo que se muera de veneno cae acá, con su experiencia
        self.reap_dead();

        // Decrementar invisibilidad y parpadeo de daño
        if self.player.damage_flash_turns > 0 {
            self.player.damage_flash_turns -= 1;
        }

        if self.player.invisible_turns > 0 {
            self.player.invisible_turns -= 1;
            if self.player.invisible_turns == 0 {
                self.add_log("> Deshácese la invisibilidad.".into(), LogType::Info);
            }
        }

        // Desgaste de cordura por turno, contenido por la voluntad
        let prob_desgaste = self.player.prob_desgaste_cordura();
        if self.rng.gen_bool(prob_desgaste) && self.player.sanity > 0 {
            self.player.sanity -= 1;
            if self.player.sanity == 0 {
                self.add_log(
                    "> DERRÚMBANSE VUESTROS PENSAMIENTOS EN EL SILENCIO.".into(),
                    LogType::Warning,
                );
            }
        }

        // Sin cordura, el silencio se come el alma: antes llegar a cero sólo
        // escribía una frase y la partida seguía igual.
        if self.player.sanity == 0 {
            self.player.hp = (self.player.hp - 1).max(0);
            self.player.damage_flash_turns = 1;
            self.add_log(
                "> Devora el silencio un pedaço de vuestra ánima.".into(),
                LogType::Whisper,
            );
        }

        // una sola pasada por el piso para todos los enemigos del turno
        let campo = self.flow_field();

        for i in 0..self.entities.len() {
            let (mut current_state, ai, ex, ey, name, pacified) = match &self.entities[i].e_type {
                EntityType::Mob {
                    state,
                    ai,
                    pacified,
                    ..
                } => (
                    state.clone(),
                    ai.clone(),
                    self.entities[i].pos.x as isize,
                    self.entities[i].pos.y as isize,
                    self.entities[i].name.clone(),
                    *pacified,
                ),
                _ => continue,
            };

            if pacified {
                continue;
            }

            let dist = (hx - ex).abs() + (hy - ey).abs();
            // Los mobs se mueven en ocho direcciones, así que la adyacencia
            // para pegar también tiene que contar las diagonales: con la
            // distancia Manhattan, uno parado en diagonal no atacaba nunca y
            // tampoco podía avanzar —la casilla del héroe está ocupada—, y
            // quedaba trabado ahí para siempre.
            let adyacente = (hx - ex).abs().max((hy - ey).abs()) == 1;

            if current_state == EnemyState::Asleep
                && dist < balance::percepcion::DISTANCIA_DESPERTAR
            {
                current_state = EnemyState::Aggressive;
                messages.push((format!("> ¡{} despierta!", name), LogType::Warning));
            }

            match current_state {
                EnemyState::Asleep => {}
                EnemyState::Wandering => {
                    if ai != EnemyAI::Stationary {
                        let dx = self.rng.gen_range(-1..=1);
                        let dy = self.rng.gen_range(-1..=1);
                        self.move_mob(i, dx, dy);
                    }
                    if dist < balance::percepcion::DISTANCIA_DETECCION
                        && self.has_los((ex, ey), (hx, hy))
                    {
                        current_state = EnemyState::Aggressive;
                    }
                }
                EnemyState::Aggressive => {
                    if adyacente {
                        if self.player.invisible_turns > 0 {
                            messages.push((
                                format!("> {} no os alcança a ver en las sombras.", name),
                                LogType::Info,
                            ));
                        } else if let EntityType::Mob {
                            min_dmg, max_dmg, ..
                        } = self.entities[i].e_type
                        {
                            let bruto = self.rng.gen_range(min_dmg..=max_dmg);
                            let esquiva = self.player.prob_esquiva();
                            // La armadura y el casco descuentan del golpe; el
                            // bloqueo parte lo que quede.
                            let mut dmg = self.player.dano_recibido(bruto);
                            if esquiva > 0.0 && self.rng.gen_bool(esquiva) {
                                dmg = 0;
                                messages.push((
                                    format!("> Hurtáis el cuerpo al golpe de {}.", name),
                                    LogType::Info,
                                ));
                            } else if self.player.parry_active {
                                dmg = (dmg / balance::combate::DIVISOR_PARRY)
                                    .max(balance::combate::DANO_MINIMO);
                                self.player.parry_active = false;
                                messages.push((
                                    format!(
                                        "> ¡REPARO! Desviáis el golpe de {} (no recebís sino {} de daño)",
                                        name, dmg
                                    ),
                                    LogType::Info,
                                ));
                            } else {
                                messages.push((
                                    format!("> {} os hiere ({} de daño)", name, dmg),
                                    LogType::Warning,
                                ));
                            }
                            self.player.hp = (self.player.hp - dmg).max(0);
                            self.player.damage_flash_turns = 1;

                            // «prefiere atacar y huir»: antes atacaba y se
                            // quedaba pegado al héroe hasta que uno de los dos
                            // caía
                            if ai == EnemyAI::Coward {
                                let desde = self.entities[i].pos;
                                if let Some((mx, my)) = self.paso_lejos_del_heroe(desde, &campo) {
                                    self.move_mob(i, mx, my);
                                }
                            }

                            // la serpiente envenena, el heraldo ciega: sale del
                            // catálogo, no de un `match` sobre el nombre
                            if dmg > 0 {
                                if let Some(efecto) = crate::bestiary::efecto_de(&name) {
                                    if !self
                                        .player
                                        .status_effects
                                        .iter()
                                        .any(|e| e.effect_type == efecto)
                                    {
                                        messages.push((
                                            format!("> {} os dexa su señal.", name),
                                            LogType::Warning,
                                        ));
                                        self.player.status_effects.push(StatusEffect {
                                            effect_type: efecto.clone(),
                                            duration: balance::efectos::duracion(&efecto),
                                            damage_per_turn: balance::efectos::dano(&efecto),
                                        });
                                    }
                                }
                            }
                        }
                    } else {
                        let desde = self.entities[i].pos;
                        match ai {
                            // el campo de flujo dobla las esquinas; el paso
                            // greedy de antes se trababa contra la pared
                            EnemyAI::Melee | EnemyAI::Wandering => {
                                if let Some((mx, my)) = self.paso_hacia_el_heroe(desde, &campo) {
                                    self.move_mob(i, mx, my);
                                }
                            }
                            EnemyAI::Coward => {
                                if let Some((mx, my)) = self.paso_lejos_del_heroe(desde, &campo) {
                                    self.move_mob(i, mx, my);
                                }
                            }
                            EnemyAI::Stationary => {}
                        }
                    }

                    if dist > balance::percepcion::DISTANCIA_PERDER_RASTRO {
                        current_state = EnemyState::Wandering;
                    }
                }
            }

            if let EntityType::Mob { ref mut state, .. } = self.entities[i].e_type {
                *state = current_state;
            }
        }
        for (m, t) in messages {
            self.add_log(m, t);
        }
    }

    /// Mueve a un mob validando colisiones con el mapa y otras entidades.
    fn move_mob(&mut self, idx: usize, dx: isize, dy: isize) {
        let new_pos = match Self::offset_point(self.entities[idx].pos, dx, dy) {
            Some(p) => p,
            None => return,
        };

        if self.es_suelo(new_pos)
            && new_pos != self.player.pos
            && !self
                .entities
                .iter()
                .enumerate()
                .any(|(i, e)| i != idx && e.pos == new_pos)
        {
            self.entities[idx].pos = new_pos;
        }
    }
}
