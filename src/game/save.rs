//! Persistencia de la partida en disco.

use super::*;
use crate::balance;
use crate::player::Player;
use crate::settings::Settings;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// Estructura serializable para la persistencia del juego en disco.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub player: Player,
    pub logs: Vec<LogMessage>,
    pub map: Vec<Vec<char>>,
    pub visible: Vec<Vec<bool>>,
    pub explored: Vec<Vec<bool>>,
    pub entities: Vec<Entity>,
    pub inventory: Vec<(Entity, usize)>,
    pub seed: u64,
    pub depth: u32,
}

impl App {
    /// Guarda el estado actual de la partida en el archivo especificado.
    pub fn save_to_file(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let save_data = SaveData {
            player: self.player.clone(),
            logs: self.logs.clone(),
            map: self.map.clone(),
            visible: self.visible.clone(),
            explored: self.explored.clone(),
            entities: self.entities.clone(),
            inventory: self.inventory.clone(),
            seed: self.seed,
            depth: self.depth,
        };

        let json = serde_json::to_string_pretty(&save_data)?;
        std::fs::write(filepath, json)?;
        Ok(())
    }

    /// Carga el estado guardado desde el archivo especificado.
    pub fn load_from_file(filepath: &str) -> Result<App, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(filepath)?;
        let save_data: SaveData = serde_json::from_str(&content)?;

        let rng = ChaCha8Rng::seed_from_u64(save_data.seed);

        let mut app = App {
            player: save_data.player,
            logs: save_data.logs,
            map: save_data.map,
            visible: save_data.visible,
            explored: save_data.explored,
            fov_radius: balance::percepcion::RADIO_FOV,
            entities: save_data.entities,
            inventory: save_data.inventory,
            seed: save_data.seed,
            depth: save_data.depth,

            drop_mode: false,
            show_descend_prompt: false,

            state: GameState::Playing,
            settings: Settings::load(crate::settings::RUTA_AJUSTES),
            rng,
        };

        app.add_log("> Partida cargada exitosamente.".into(), LogType::Info);
        Ok(app)
    }

    /// Lee lo mínimo de una partida guardada sin cargarla entera:
    /// piso, alma, alma máxima y semilla.
    pub fn peek_save(ruta: &str) -> Option<(u32, i32, i32, u64)> {
        let contenido = std::fs::read_to_string(ruta).ok()?;
        let datos: SaveData = serde_json::from_str(&contenido).ok()?;
        Some((
            datos.depth,
            datos.player.hp,
            datos.player.max_hp,
            datos.seed,
        ))
    }
}
