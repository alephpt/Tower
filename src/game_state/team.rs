

pub enum TeamDirection {
    LeftFacing,
    RightFacing,
    TopDown,
    BottomUp,
}

pub struct Team {
    pub name: String,
    pub id: u32,
    pub direction: TeamDirection,
    pub players: Vec<Player>,
    pub bases: Vec<Base>,
    pub mobs: Vec<Mob>,
}

impl Team {
    pub fn new(name: String, id: u32) -> Team {
        Team {
            name: name,
            id: id,
            direction: TeamDirection::LeftFacing,
            players: Vec::new(),
            bases: Vec::new(),
            mobs: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn add_base(&mut self, base: Base) {
        self.bases.push(base);
    }

    pub fn add_mob(&mut self, mob: Mob) {
        self.mobs.push(mob);
    }

    pub fn draw(&self, window: &mut Window) {
        for base in &self.bases {
            base.draw(window);
        }
        for mob in &self.mobs {
            mob.draw(window);
        }
    }

    pub fn move_mobs(&mut self) {
        for mob in &mut self.mobs {
            mob.move_mob();
        }
    }

    pub fn spawn_mobs(&mut self) {
        for mob in &mut self.mobs {
            mob.spawn();
        }
    }

    pub fn update_player_state(&mut self) {
        for player in &mut self.players {
            player.update_state();
        }
    }
}
