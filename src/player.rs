//! El héroe: sus números, su equipo y lo que de ellos se deriva.
//!
//! Antes estos campos vivían sueltos en `App` y se repetían enteros en
//! `SaveData`, en `App::new` y en `App::load_from_file`: agregar un atributo
//! obligaba a tocar cinco lugares y era fácil que uno quedara atrás —así se
//! perdían el nivel y el equipo al bajar de piso—. Acá el héroe es un solo
//! valor que se mueve completo.

use crate::app::{Point, StatusEffect, StatusEffectType};
use crate::balance;
use serde::{Deserialize, Serialize};

/// Un objeto equipado: cómo se llama y qué aporta.
pub type Ranura = Option<(String, i32)>;
/// El arma aporta dos números: daño mínimo y máximo.
pub type RanuraArma = Option<(String, i32, i32)>;

/// Los tres atributos del héroe.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub willpower: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            strength: balance::atributos::BASE,
            agility: balance::atributos::BASE,
            willpower: balance::atributos::BASE,
        }
    }
}

/// Las cinco ranuras de equipo.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub weapon: RanuraArma,
    pub armor: Ranura,
    pub helmet: Ranura,
    pub ring: Ranura,
    pub amulet: Ranura,
}

impl Equipment {
    /// Defensa que suman armadura y casco.
    pub fn defensa(&self) -> i32 {
        let armadura = self.armor.as_ref().map(|a| a.1).unwrap_or(0);
        let casco = self.helmet.as_ref().map(|h| h.1).unwrap_or(0);
        armadura + casco
    }

    /// Daño del arma, o los puños si no hay nada equipado.
    pub fn dano_arma(&self) -> (i32, i32) {
        let (mut min, mut max) = self
            .weapon
            .as_ref()
            .map(|w| (w.1, w.2))
            .unwrap_or(balance::combate::PUNOS);
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
        (min, max)
    }
}

/// El héroe completo.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub pos: Point,
    pub hp: i32,
    pub max_hp: i32,
    pub sanity: i32,
    pub max_sanity: i32,
    pub level: u32,
    pub xp: u32,
    pub next_level_xp: u32,
    pub stats: Stats,
    pub equipment: Equipment,
    pub status_effects: Vec<StatusEffect>,
    pub parry_active: bool,
    pub invisible_turns: usize,
    pub damage_flash_turns: usize,
}

impl Player {
    /// Un héroe recién despertado en el piso 1.
    pub fn nuevo(pos: Point) -> Player {
        Player {
            pos,
            hp: balance::progresion::VIDA_INICIAL,
            max_hp: balance::progresion::VIDA_INICIAL,
            sanity: balance::progresion::CORDURA_INICIAL,
            max_sanity: balance::progresion::CORDURA_INICIAL,
            level: 1,
            xp: 0,
            next_level_xp: balance::progresion::XP_PRIMER_NIVEL,
            stats: Stats::default(),
            equipment: Equipment::default(),
            status_effects: Vec::new(),
            parry_active: false,
            invisible_turns: 0,
            damage_flash_turns: 0,
        }
    }

    /* ─────────── números derivados ───────────
     *
     * Se calculan al vuelo en vez de acumularse en campos: así sacarse el
     * amuleto no deja rastro y no hay estado que pueda desincronizarse.
     */

    /// Defensa total del equipo puesto.
    pub fn defensa_total(&self) -> i32 {
        self.equipment.defensa()
    }

    /// Fuerza real: la propia más la del anillo.
    pub fn fuerza_efectiva(&self) -> i32 {
        self.stats.strength + self.equipment.ring.as_ref().map(|r| r.1).unwrap_or(0)
    }

    /// Cordura máxima real: la del nivel más la que sostiene el amuleto.
    pub fn max_sanity_total(&self) -> i32 {
        self.max_sanity + self.equipment.amulet.as_ref().map(|a| a.1).unwrap_or(0)
    }

    /// Daño extra que suma la fuerza por encima de la base.
    pub fn brazo(&self) -> i32 {
        (self.fuerza_efectiva() - balance::atributos::BASE).max(0)
            * balance::atributos::DANO_POR_FUERZA
    }

    /// Probabilidad de esquivar un golpe, según la agilidad.
    pub fn prob_esquiva(&self) -> f64 {
        let excedente = (self.stats.agility - balance::atributos::BASE).max(0) as f64;
        (excedente * balance::atributos::ESQUIVA_POR_AGILIDAD)
            .min(balance::atributos::ESQUIVA_MAXIMA)
    }

    /// Probabilidad de perder cordura este turno; la voluntad la contiene.
    pub fn prob_desgaste_cordura(&self) -> f64 {
        let excedente = (self.stats.willpower - balance::atributos::BASE).max(0) as f64;
        let aguante = (excedente * balance::atributos::AGUANTE_POR_VOLUNTAD).min(1.0);
        balance::cordura::PROB_DESGASTE * (1.0 - aguante)
    }

    /// Daño que llega al héroe después de la defensa del equipo.
    pub fn dano_recibido(&self, bruto: i32) -> i32 {
        (bruto - self.defensa_total()).max(balance::combate::DANO_MINIMO)
    }

    /// Si el héroe carga un efecto de estado determinado.
    pub fn tiene(&self, efecto: &StatusEffectType) -> bool {
        self.status_effects.iter().any(|e| &e.effect_type == efecto)
    }

    /// La cordura nunca puede pasarse de su techo actual.
    pub fn ajustar_cordura(&mut self) {
        self.sanity = self.sanity.min(self.max_sanity_total());
    }

    /// Suma experiencia y devuelve un mensaje por cada nivel alcanzado.
    pub fn ganar_xp(&mut self, amount: u32) -> Vec<String> {
        self.xp += amount;
        let mut subidas = Vec::new();

        while self.xp >= self.next_level_xp {
            self.xp -= self.next_level_xp;
            self.level += 1;
            self.next_level_xp =
                (self.next_level_xp as f32 * balance::progresion::FACTOR_XP) as u32;

            self.max_hp += balance::progresion::VIDA_POR_NIVEL;
            self.hp = self.max_hp;
            self.max_sanity += balance::progresion::CORDURA_POR_NIVEL;
            self.sanity = self.max_sanity_total();
            self.stats.strength += 1;
            self.stats.agility += 1;
            self.stats.willpower += 1;

            subidas.push(format!(
                "> ¡SUBIDA DE NIVEL! Alcanzas el Nivel {}. Atributos incrementados.",
                self.level
            ));
        }
        subidas
    }
}
