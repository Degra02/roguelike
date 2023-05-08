#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapTile {
    Entrance{pos: (u32, u32), to: Direction},
    Exit{ pos: (u32, u32), from: Direction},
    Path {pos: (u32, u32), from: Direction, to: Direction},
    Empty {pos: (u32, u32)},
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub start: (u32, u32),
    pub end: (u32, u32),
    pub map_tiles: Vec<MapTile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let start = (rand::random::<u32>() % width, 0);
        let (end, map_tiles) = generate_main_path(width, height, start);

        Self {
            width,
            height,
            start,
            end,
            map_tiles,
        }
    }

    pub fn add_path(&mut self, map_tiles: Vec<MapTile>) {
        self.map_tiles = map_tiles;
    }
}

pub fn generate_main_path(width: u32, height: u32, start: (u32, u32)) -> ((u32, u32), Vec<MapTile>) {
    let mut current_pos = start;

    let mut prev_dir = Direction::Down;
    let mut map_tiles = Vec::<MapTile>::new();
    let mut direction_map_tiles = Vec::<MapTile>::new();

    while current_pos.1 != height - 1 {
        let direction : Option<Direction> = match rand::random::<u32>() % 3 {
            0 => Some(Direction::Down),
            1 => {
                if current_pos.0 != width - 1 && prev_dir != Direction::Left {
                    current_pos.0 += 1;
                    Some(Direction::Right)
                } else {
                    None
                }
            }
            2 => {
                if current_pos.0 != 0 && prev_dir != Direction::Right {
                    current_pos.0 -= 1;
                    Some(Direction::Left)
                } else {
                    None
                }
            }
            _ => panic!("This should never happen"),
        }; 

        if let Some(dir) = direction{
            let map_tile = MapTile::Path { pos: current_pos, from: prev_dir, to: dir };
            direction_map_tiles.push(map_tile);

            prev_dir = dir;
        }
    }


    for i in 0..height {
        for j in 0..width {
           map_tiles.push(MapTile::Empty{pos: (j, i)});
        }
    }

    for dir_map_tile in direction_map_tiles.iter() {
        match dir_map_tile {
            MapTile::Path {pos, from, to} => {
                let (x, y) = pos;
                map_tiles[(y * width + x) as usize] = MapTile::Path {pos: (*x, *y), from: *from, to: *to};
            }
            _ => panic!("This should never happen"),
        }
    }

    ((current_pos.0, current_pos.1), direction_map_tiles)
}

#[test]
fn generate_map() {
    let map = Map::new(5, 7);
    println!("{:?}", map)
}
