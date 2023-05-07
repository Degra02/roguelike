#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapTile {
    entrance: Direction,
    exit: Direction
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub start: (u32, u32),
    pub end: (u32, u32),
    pub main_path: Vec<Direction>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let start = (rand::random::<u32>() % width, 0);
        let (end, main_path) = generate_main_path(width, height, start);

        Self {
            width,
            height,
            start,
            end,
            main_path,
        }
    }

    pub fn add_path(&mut self, main_path: Vec<Direction>) {
        self.main_path = main_path;
    }
}

pub fn generate_main_path(width: u32, height: u32, start: (u32, u32)) -> ((u32, u32), Vec<Direction>) {
    let mut path = Vec::<Direction>::new();

    let mut current_height = start.1;
    let mut current_width = start.0;

    let mut prev_dir = Direction::Down;

    while current_height != height - 1 {
        let direction : Option<Direction> = match rand::random::<u32>() % 3 {
            0 => Some(Direction::Down),
            1 => {
                if current_width != width - 1 && prev_dir != Direction::Left {
                    current_width += 1;
                    Some(Direction::Right)
                } else {
                    None
                }
            }
            2 => {
                if current_width != 0 && prev_dir != Direction::Right {
                    current_width -= 1;
                    Some(Direction::Left)
                } else {
                    None
                }
            }
            _ => panic!("This should never happen"),
        };

        if direction.is_some() {
            prev_dir = direction.unwrap();
            path.push(direction.unwrap());
        }

        match direction {
            Some(Direction::Down) => current_height += 1,
            _ => (),
        }
    }

    ((current_width, current_height), path)
}

#[test]
fn generate_map() {
    let mut map = Map::new(5, 7);
    println!("{:?}", map)
}
