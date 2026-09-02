//! Trabajo sobre la grilla del mapa.
//!
//! Todo acceso al mapa pasa por acá. Antes cada sitio repetía a mano
//! `pos.y < self.map.len() && pos.x < self.map[0].len()` —y `has_los` indexaba
//! sin chequear nada, apoyado en que quien lo llamara acotara los extremos.

use super::*;

impl App {
    /// Alto y ancho del mapa en casillas.
    pub fn dimensiones(&self) -> (usize, usize) {
        (
            self.map.len(),
            self.map.first().map(|f| f.len()).unwrap_or(0),
        )
    }

    /// El glifo de una casilla, o `None` si cae fuera del mapa.
    pub fn tile(&self, pos: Point) -> Option<char> {
        self.map.get(pos.y).and_then(|f| f.get(pos.x)).copied()
    }

    /// Igual que `tile`, para coordenadas con signo.
    pub fn tile_en(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.tile(Point::new(x as usize, y as usize))
    }

    /// Si se puede caminar por una casilla: suelo o escalera.
    pub fn es_transitable(&self, pos: Point) -> bool {
        matches!(self.tile(pos), Some('.') | Some('>'))
    }

    /// Si una casilla es suelo libre, sin contar la escalera.
    pub fn es_suelo(&self, pos: Point) -> bool {
        self.tile(pos) == Some('.')
    }

    /// Si una casilla corta la línea de visión.
    ///
    /// Las puertas cerradas cuentan: son entidades sobre suelo, así que mirar
    /// sólo el glifo del mapa las dejaba transparentes y se veía a través de
    /// ellas, aunque el TODO diera el bloqueo por hecho.
    pub fn bloquea_vision(&self, pos: Point) -> bool {
        if !self.es_transitable(pos) {
            return true;
        }
        self.entities.iter().any(|e| {
            e.pos == pos
                && matches!(
                    e.e_type,
                    EntityType::Door { open: false, .. } | EntityType::TalkingWall { .. }
                )
        })
    }

    /// Transforma los muros básicos en glifos de dibujo de caja para una mejor estética.
    pub fn smooth_walls(&mut self) {
        let mut new_map = self.map.clone();
        let height = self.map.len();
        let width = self.map[0].len();
        for y in 0..height {
            for x in 0..width {
                if self.map[y][x] == '#' {
                    let mut mask = 0;
                    if y > 0 && self.map[y - 1][x] == '#' {
                        mask += 1;
                    }
                    if y < height - 1 && self.map[y + 1][x] == '#' {
                        mask += 2;
                    }
                    if x < width - 1 && self.map[y][x + 1] == '#' {
                        mask += 4;
                    }
                    if x > 0 && self.map[y][x - 1] == '#' {
                        mask += 8;
                    }
                    let ch = match mask {
                        1..=3 => '║',
                        4 | 8 | 12 => '═',
                        5 => '╚',
                        6 => '╔',
                        9 => '╝',
                        10 => '╗',
                        7 => '╠',
                        11 => '╣',
                        13 => '╩',
                        14 => '╦',
                        15 => '╬',
                        _ => '■',
                    };
                    new_map[y][x] = ch;
                }
            }
        }
        self.map = new_map;
    }
}
