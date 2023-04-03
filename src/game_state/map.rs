// Type to store, create and update map data

pub enum Orientation{
    Vertical,
    Horizontal,
}

pub struct Map {
    pub n_teams: u32,
    pub orientation: Orientation,
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,
    pub center_x: u32,
    pub center_y: u32,
}

impl Map {
    
}