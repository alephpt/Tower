

pub struct Player {
    pub name: String,
    pub id: u32,
    pub team: u32,
    pub base_location: (u32, u32),
}

impl Player {
    pub fn new(name: String, id: u32, team: u32, base_location: (u32, u32)) -> Player {
        Player {
            name,
            id,
            team,
            base_location,
        }
    }

    pub fn update_state(&mut self) {
        
    }
}