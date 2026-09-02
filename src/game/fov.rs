//! Campo de visión y línea de visión.

use super::*;

impl App {
    /// Calcula el Campo de Visión (FOV) del héroe basado en un radio definido.
    pub fn calculate_fov(&mut self) {
        let hx = self.player.pos.x as isize;
        let hy = self.player.pos.y as isize;
        for row in &mut self.visible {
            for val in row {
                *val = false;
            }
        }
        for y in (hy - self.fov_radius)..=(hy + self.fov_radius) {
            for x in (hx - self.fov_radius)..=(hx + self.fov_radius) {
                if self.tile_en(x, y).is_some()
                    && (x - hx).pow(2) + (y - hy).pow(2) <= self.fov_radius.pow(2)
                    && self.has_los((hx, hy), (x, y))
                {
                    self.visible[y as usize][x as usize] = true;
                    self.explored[y as usize][x as usize] = true;
                }
            }
        }
    }

    /// Comprueba si existe línea de visión (LOS) entre dos puntos (Algoritmo de Bresenham).
    pub fn has_los(&self, p0: (isize, isize), p1: (isize, isize)) -> bool {
        let (mut x, mut y) = p0;
        let (x1, y1) = p1;
        let (dx, dy) = ((x1 - x).abs(), -(y1 - y).abs());
        let (sx, sy) = (
            if p0.0 < x1 { 1 } else { -1 },
            if p0.1 < y1 { 1 } else { -1 },
        );
        let mut err = dx + dy;
        loop {
            if x == x1 && y == y1 {
                return true;
            }
            // fuera del mapa no se ve nada; una puerta cerrada tampoco deja ver
            if x != p0.0 || y != p0.1 {
                match self.tile_en(x, y) {
                    None => return false,
                    Some(_) if self.bloquea_vision(Point::new(x as usize, y as usize)) => {
                        return false
                    }
                    Some(_) => {}
                }
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
