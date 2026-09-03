//! Persistencia de la partida en disco.

use super::*;
use crate::balance;
use crate::player::Player;
use crate::settings::Settings;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// La misma forma que `SaveData`, pero prestada.
///
/// Guardar clonaba el mapa, las entidades y los 200 mensajes del historial
/// enteros sólo para entregárselos a `serde`. Escribir es de sólo lectura:
/// alcanza con prestarlos. `SaveData` sigue igual para el camino de vuelta,
/// donde sí hace falta ser dueño de los datos.
#[derive(Serialize)]
struct SaveDataRef<'a> {
    player: &'a Player,
    logs: &'a VecDeque<LogMessage>,
    map: &'a Vec<Vec<char>>,
    visible: &'a Vec<Vec<bool>>,
    explored: &'a Vec<Vec<bool>>,
    entities: &'a Vec<Entity>,
    inventory: &'a Vec<(Entity, usize)>,
    seed: u64,
    depth: u32,
}

/// Estructura serializable para la persistencia del juego en disco.
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub player: Player,
    pub logs: VecDeque<LogMessage>,
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
        let save_data = SaveDataRef {
            player: &self.player,
            logs: &self.logs,
            map: &self.map,
            visible: &self.visible,
            explored: &self.explored,
            entities: &self.entities,
            inventory: &self.inventory,
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

            resbalon_pendiente: false,
            drop_mode: false,
            show_descend_prompt: false,

            state: GameState::Playing,
            settings: Settings::default(),
            rng,
        };

        app.add_log(
            "> Cobrada la jornada, y en buena hora.".into(),
            LogType::Info,
        );
        Ok(app)
    }

    /// Lee lo mínimo de una partida guardada sin cargarla entera:
    /// piso, alma, alma máxima y semilla.
    /// Borra la partida guardada.
    ///
    /// La corrida es de ida: morir —o llegar al final— disuelve el fragmento.
    /// Que no exista el archivo no es un error, es el caso normal.
    pub fn borrar_save(ruta: &str) {
        let _ = std::fs::remove_file(ruta);
    }

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
