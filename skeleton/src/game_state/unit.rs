
// type for a single Unit
pub struct Unit {
    pub target: [u32; 2],
    pub speed: u32,
    pub health: u32,
    pub x: u32,
    pub y: u32,
}

impl Unit {
    // creates a new Unit
    pub fn new(target: [u32; 2], speed: u32, health: u32, x: u32, y: u32) -> Unit {
        Unit {
            target,
            speed,
            health,
            x,
            y,
        }
    }

    // draws the Unit
    pub fn draw(&self, window: &mut Window) {
        let mut Unit = window.add_cube(1.0, 1.0, 1.0);
        Unit.set_color(0.0, 0.0, 0.0);
        Unit.set_local_translation(Translation3::new(self.x as f32, self.y as f32, 0.0));
    }

    // moves the Unit
    pub fn move_Unit(&mut self) {
        if self.x < self.target[0] {
            self.x += self.speed;
        } else if self.x > self.target[0] {
            self.x -= self.speed;
        }

        if self.y < self.target[1] {
            self.y += self.speed;
        } else if self.y > self.target[1] {
            self.y -= self.speed;
        }
    }

    // checks if the Unit is dead
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    // checks if the Unit has reached the end of the path
    pub fn has_reached_end(&self) -> bool {
        self.x == self.target[0] && self.y == self.target[1]
    }

    // damages the Unit
    pub fn damage(&mut self, damage: u32) {
        self.health -= damage;
    }

    // checks if the Unit is at a given point
    pub fn is_at(&self, x: u32, y: u32) -> bool {
        self.x == x && self.y == y
    }
}

