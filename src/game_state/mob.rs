// defining a target type and a mob type

// defines a group of enemies
pub struct Mob {
    pub enemies: Vec<Unit>,
    pub path: Vec<[u32; 2]>,
    pub speed: u32,
    pub health: u32,
    pub x: u32,
    pub y: u32,
}

// implements Mob
impl Mob {
    // creates a new mob
    pub fn new(path: Vec<[u32; 2]>, speed: u32, health: u32, x: u32, y: u32) -> Mob {
        Mob {
            enemies: Vec::new(),
            path,
            speed,
            health,
            x,
            y,
        }
    }

    // draws the mob
    pub fn draw(&self, window: &mut Window) {
        for Unit in &self.enemies {
            Unit.draw(window);
        }
    }

    // moves the mob
    pub fn move_mob(&mut self) {
        for Unit in &mut self.enemies {
            Unit.move_Unit();
        }
    }

    // spawns a new Unit
    pub fn spawn(&mut self) {
        self.enemies.push(Unit::new(self.path[0], self.speed, self.health, self.x, self.y));
    }

    // damages the mob
    pub fn damage(&mut self, damage: u32) {
        for Unit in &mut self.enemies {
            Unit.damage(damage);
        }
    }

    // checks if the mob is dead
    pub fn is_dead(&self) -> bool {
        for Unit in &self.enemies {
            if !Unit.is_dead() {
                return false;
            }
        }
        true
    }

    // checks if the mob has reached the end of the path
    pub fn has_reached_end(&self) -> bool {
        for Unit in &self.enemies {
            if !Unit.has_reached_end() {
                return false;
            }
        }
        true
    }

    // checks if the mob is at a given point
    pub fn is_at(&self, x: u32, y: u32) -> bool {
        for Unit in &self.enemies {
            if !Unit.is_at(x, y) {
                return false;
            }
        }
        true
    }
}