pub type Map = Vec<Vec<Tile>>;

pub fn gen_map(width: usize, height: usize) -> Map {
    let mut map = vec![vec![Tile::empty(); height]; width];

    map[20][10] = Tile::wall();
    map[27][30] = Tile::wall();
    map[40][2] = Tile::wall();
    map[15][27] = Tile::wall();

    map
}



#[derive(Clone, Copy)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, block_sight: false }
    }

    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true }
    }
}