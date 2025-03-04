use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Left,
    Down
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Right
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new(world_width: usize, spawn_idx: usize) -> World {
        World {
            width: world_width,
            size: world_width * world_width,
            snake: Snake::new(spawn_idx)
        }
    }
     
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();

        match self.snake.direction {
            Direction::Right => {
                let next_col = (snake_idx + 1) % self.width;
                self.snake.body[0].0 = (snake_idx + 1) % self.size
            },
            Direction::Left => self.snake.body[0].0 = (snake_idx - 1) % self.size,
            Direction::Down => self.snake.body[0].0 = (snake_idx + 1) % self.size,
            Direction::Up => self.snake.body[0].0 = (snake_idx + 1) % self.size,

        }

    }
}

// wasm-pack build --target web
