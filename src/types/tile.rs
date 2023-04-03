// tile information for a game square

// TyleType is an enum that can be one of the following: Empty, Wall, Floor, Mountain
pub enum TyleType
{
    Empty,
    Wall,
    Floor,
    Mountain,
}

// TileState is an enum that can be one of the following: Empty, Occupied, Blocked
pub enum TileState
{
    Empty,
    Occupied,
    Blocked,
}

// TileContent is an enum that can be one of the following: Empty, Player, Tower
pub enum TileContent
{
    Empty,
    Player,
    Tower,
}

pub struct Tile {
    pub x: u32,
    pub y: u32,
    pub tile_type: TileType,
    pub tile_state: TileState,
    pub tile_content: TileContent,
}

impl Tile {
    pub fn new(x: u32, y: u32, tile_type: TileType, tile_state: TileState, tile_content: TileContent) -> Tile {
        Tile {
            x,
            y,
            tile_type,
            tile_state,
            tile_content,
        }
    }
}