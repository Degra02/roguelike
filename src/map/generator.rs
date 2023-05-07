#[derive(Debug, Clone, Copy)]
pub enum Directions {
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub start: (u32, u32),
    pub main_path: Vec<Directions>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let start = (rand::random::<u32>() % width, 0);

        Self {
            width,
            height,
            start,
            main_path: generate_main_path(width, height, start),
        }
    }
}

pub fn generate_main_path(width: u32, height: u32, start: (u32, u32)) -> Vec<Directions> {
    let mut path = Vec::<Directions>::new();

    let mut current_height = start.1;
    let mut current_width = start.0;

    while current_height != height - 1 {
        let direction = match rand::random::<u32>() % 3 {
            0 => Directions::Down,
            1 => {
                if current_width != width - 1 {
                    current_width += 1;
                    Directions::Right
                } else {
                    current_width -= 1;
                    Directions::Left
                }
            }
            2 => {
                if current_width != 0 {
                    current_width -= 1;
                    Directions::Left
                } else {
                    current_width += 1;
                    Directions::Right
                }
            }
            _ => panic!("This should never happen"),
        };

        path.push(direction);

        match direction {
            Directions::Down => current_height += 1,
            _ => (),
        }
    }

    path
}
