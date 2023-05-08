use std::fmt::{Display, Formatter};

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

impl Display for  MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapTile::Entrance { pos , to} => {
                write!(f, "[Entrance: {:?}; to: {:?}]", pos, to)
            }
            MapTile::Exit { pos, from } => {
                write!(f, "[Exit: {:?}; from: {:?}]", pos, from)
            }
            MapTile::Path { pos, from, to } => {
                write!(f, "[Path: {:?}; from: {:?}, to: {:?}]", pos, from, to)
            }
            MapTile::Empty { pos } => {
                write!(f, "[Empty: {:?}]", pos)
            }
        }
    }
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
    let mut to_save_pos = start;

    let mut prev_dir = Direction::Down;
    let mut map_tiles = Vec::<MapTile>::new();
    let mut direction_map_tiles = Vec::<MapTile>::new();

    let mut first = true;

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
            let map_tile: MapTile;
            if to_save_pos.1 == 0  && first {
                map_tile = MapTile::Entrance { pos: start, to: dir }; 
                first = false;
            } else if to_save_pos.1 == height - 1 {
                map_tile = MapTile::Exit { pos: to_save_pos, from: prev_dir };
            } else {
                map_tile = MapTile::Path { pos: to_save_pos, from: prev_dir, to: dir };
            }
            direction_map_tiles.push(map_tile);
            if dir == Direction::Down {
                current_pos.1 += 1;
            }

            prev_dir = dir;
            to_save_pos = current_pos;
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
            MapTile::Entrance { pos, to } => {
                let (x, y) = pos;
                map_tiles[(y * width + x) as usize] = MapTile::Entrance {pos: (*x, *y), to: *to};
            }
            MapTile::Exit { pos, from } => {
                let (x, y) = pos;
                map_tiles[(y * width + x) as usize] = MapTile::Exit {pos: (*x, *y), from: *from};
            }
            _ => {panic!("No Empty tiles should be here");}
        }
        }

    ((current_pos.0, current_pos.1), map_tiles)
}

#[test]
fn generate_map() {
    let map = Map::new(5, 7);
    for i in 0..map.height {
        for j in 0..map.width {
            print!(" {} ", map.map_tiles[(i * map.width + j) as usize]);
        }
        println!();
    } 
}
