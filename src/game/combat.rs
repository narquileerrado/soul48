//! Combate: habilidades activas y la muerte de los enemigos.

use super::*;
use crate::balance;

impl App {
    /// Ejecuta la habilidad activa de Embestida (Pushback) sobre un enemigo adyacente.
    pub fn use_pushback(&mut self) -> bool {
        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in directions {
            if let Some(target_pos) = Self::offset_point(self.player.pos, dx, dy) {
                if let Some(idx) = self
                    .entities
                    .iter()
                    .position(|e| e.pos == target_pos && matches!(e.e_type, EntityType::Mob { .. }))
                {
                    let push_pos = Self::offset_point(target_pos, dx, dy);
                    if let Some(p_pos) = push_pos {
                        if self.es_suelo(p_pos) {
                            self.entities[idx].pos = p_pos;
                            self.add_log(
                                format!(
                                    "> EMBESTIDA: Empujas a {} hacia atrás.",
                                    self.entities[idx].name
                                ),
                                LogType::Combat,
                            );
                            return true;
                        }
                    }
                    self.add_log(
                        format!(
                            "> EMBESTIDA: Impactas a {} contra la pared.",
                            self.entities[idx].name
                        ),
                        LogType::Combat,
                    );
                    if let EntityType::Mob { ref mut hp, .. } = self.entities[idx].e_type {
                        *hp -= balance::combate::EMBESTIDA_CONTRA_MURO;
                    }
                    self.reap_dead();
                    return true;
                }
            }
        }
        self.add_log(
            "> No hay enemigos adyacentes para embestir.".into(),
            LogType::Warning,
        );
        false
    }

    /// Activa la postura de Bloqueo / Parry para reducir el daño del próximo turno.
    pub fn use_parry(&mut self) -> bool {
        self.player.parry_active = true;
        self.add_log(
            "> BLOQUEO ACTIVO: Te preparas para desviar el próximo impacto.".into(),
            LogType::Info,
        );
        true
    }

    /// Retira del mapa a todo mob sin vida y reparte su experiencia.
    ///
    /// Es el único lugar donde un enemigo muere. Cualquier vía de daño
    /// —cuerpo a cuerpo, pergamino, embestida, efecto de estado— sólo resta
    /// vida; después llama acá.
    pub fn reap_dead(&mut self) {
        let caidos: Vec<String> = self
            .entities
            .iter()
            .filter_map(|e| match e.e_type {
                EntityType::Mob { hp, .. } if hp <= 0 => Some(e.name.clone()),
                _ => None,
            })
            .collect();

        if caidos.is_empty() {
            return;
        }

        self.entities
            .retain(|e| !matches!(e.e_type, EntityType::Mob { hp, .. } if hp <= 0));

        for nombre in caidos {
            self.add_log(format!("> {} eliminada.", nombre), LogType::Combat);
            self.add_xp(crate::bestiary::xp_de(&nombre));
        }
    }
}
