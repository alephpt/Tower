// create tile based game board

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: u32, height: u32, display: &glium::Display) -> Grid {
        let mut tiles = Vec::new();
        for x in 0..width {
            for y in 0..height {
                tiles.push(Tile::new(x, y, TileType::Empty, TileState::Empty, TileContent::Empty, display));
            }
        }
        Grid {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &Tile {
        &self.tiles[(x + y * self.width) as usize]
    }

    pub fn update_tile(&mut self, x: u32, y: u32, tile_type: TileType, tile_state: TileState, tile_content: TileContent) {
        self.tiles[(x + y * self.width) as usize].update(tile_type, tile_state, tile_content);
    }

    pub fn get_tile_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    pub fn get_tile_type(&self, x: u32, y: u32) -> TileType {
        self.tiles[(x + y * self.width) as usize].tile_type
    }

    pub fn get_tile_state(&self, x: u32, y: u32) -> TileState {
        self.tiles[(x + y * self.width) as usize].tile_state
    }

    pub fn get_tile_content(&self, x: u32, y: u32) -> TileContent {
        self.tiles[(x + y * self.width) as usize].tile_content
    }

    pub fn draw_grid(&self, target: &mut glium::Frame, program: &glium::Program, uniforms: &glium::uniforms::Uniforms) {
        for tile in &self.tiles {
            tile.draw(target, program, uniforms);
        }
    }
}