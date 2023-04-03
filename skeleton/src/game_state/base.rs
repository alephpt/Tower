

use kiss3d::{window::Window, nalgebra::{Translation3, Point3}};

pub enum BaseType {
    Spawn,
    Entry,
    Intermediate,
    Exit,
    Stronhold,
}

pub struct Base {
    pub node_type: BaseType,
    pub player_id: u32,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: [f32; 3],
    pub towers: Vec<Tower>,
}

impl Base {
    pub fn new(node_type: BaseType, player_id: u32, x: f32, y: f32, size: f32, color: [f32; 3]) -> Base {
        Base {
            node_type,
            player_id,
            x,
            y,
            size,
            color,
            next_base: None,
        }
    }

    pub fn set_next_base(&mut self, next_base: Base) {
        self.next_base = Some(Box::new(next_base));
    }

    pub fn get_next_base_coordinates(&self) -> Option<(f32, f32)> {
        match &self.next_base {
            Some(next_base) => Some((next_base.x, next_base.y)),
            None => None,
        }
    }

    pub fn draw(&self, window: &mut Window) {
        let mut base = window.add_cube(self.size, self.size, self.size);
        base.set_color(self.color[0], self.color[1], self.color[2]);
        base.set_local_translation(Translation3::new(self.x, self.y, 0.0));
    }

    pub fn draw_connections(&self, window: &mut Window) {
        match &self.next_base {
            Some(next_base) => {
                window.draw_line(
                    &Point3::origin(),
                    &Point3::new(self.x, self.y, 0.0),
                    &Point3::new(next_base.x, next_base.y, 0.0),
                );
            }
            None => (),
        }
    }
}