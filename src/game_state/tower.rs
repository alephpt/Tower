// defining towers for tower defense
//

pub struct Tower {
    pub attack: u32,
    pub attack_speed: u32,
    pub range: u32,
    pub cost: u32,
    pub name: String,
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub color: (f32, f32, f32, f32),
}

impl Tower {
    pub fn new(attack: u32, attack_speed: u32, range: u32, cost: u32, name: String, id: u32, x: u32, y: u32, color: (f32, f32, f32, f32)) -> Tower {
        Tower {
            attack,
            attack_speed,
            range,
            cost,
            name,
            id,
            x,
            y,
            color,
        }
    }

    pub fn draw(&self, window: &mut Window) {
        let mut tower = window.add_cube(self.range as f32, self.range as f32, self.range as f32);
        tower.set_color(self.color.0, self.color.1, self.color.2);
        tower.set_local_translation(Translation3::new(self.x as f32, self.y as f32, 0.0));
    }

    pub fn find_target(&self, enemies: &Vec<Enemy>) -> Option<Enemy> {
        let mut target: Option<Enemy> = None;
        let mut distance: f32 = 0.0;
        for enemy in enemies {
            let enemy_distance = ((enemy.x - self.x) as f32).powf(2.0) + ((enemy.y - self.y) as f32).powf(2.0);
            if enemy_distance < distance || distance == 0.0 {
                distance = enemy_distance;
                target = Some(enemy.clone());
            }
        }
        target
    }

    pub fn attack(&self, target: &Enemy) {
        
    }
}
