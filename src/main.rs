use std::time::Duration;

use macroquad::{prelude::*, rand};

const CELL_SIZE: f32 = 40.;

struct Game {
    cells: Vec<Vec<Cell>>,
}

impl Game {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        Self { cells }
    }

    fn get_neighbors(&self, i: usize, j: usize) -> usize {
        let mut neighbor_count: usize = 0;
        let directions = [
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];

        for (dx, dy) in directions.iter() {
            if let Some(row_of_cells) = self.cells.get((i as isize + dx) as usize) {
                if let Some(cell) = row_of_cells.get((j as isize + dy) as usize) {
                    if matches!(cell.cell_status, Status::Alive) {
                        neighbor_count += 1;
                    }
                }
            }
        }

        neighbor_count
    }

    fn apply_rules(&mut self) {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                // check neighbors
                let n = self.get_neighbors(i, j);

                if n < 2 {
                    if matches!(self.cells[i][j].cell_status, Status::Alive) {
                        self.cells[i][j].cell_status = Status::Dead;
                        continue;
                    }
                }

                if n == 2 || n == 3 {
                    if matches!(self.cells[i][j].cell_status, Status::Alive) {
                        //lives
                        continue;
                    }
                }

                if n > 3 {
                    if matches!(self.cells[i][j].cell_status, Status::Alive) {
                        self.cells[i][j].cell_status = Status::Dead;
                        continue;
                    }
                }

                if n == 3 {
                    if matches!(self.cells[i][j].cell_status, Status::Dead) {
                        self.cells[i][j].cell_status = Status::Alive;
                        continue;
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        for r in &mut self.cells {
            for c in r {
                c.update_cell();
            }
        }
    }

    fn draw(&self) {
        for r in &self.cells {
            for c in r {
                c.draw_cell();
            }
        }
    }
}

#[derive(Debug)]
enum Status {
    Alive,
    Dead,
}

impl Status {
    fn random_value() -> Self {
        let number = rand::gen_range(0, 2);

        match number {
            0 => Status::Alive,
            1 => Status::Dead,
            _ => panic!("this is not supposed to happen"),
        }
    }
}

#[derive(Debug)]
struct Cell {
    x_pos: f32,
    y_pos: f32,
    cell_status: Status,
    color: Color,
}

impl Cell {
    fn new(x_pos: f32, y_pos: f32) -> Self {
        Self {
            x_pos,
            y_pos,
            cell_status: Status::random_value(),
            color: WHITE,
        }
    }

    fn update_cell(&mut self) {
        match self.cell_status {
            Status::Alive => self.color = BLACK,
            Status::Dead => self.color = WHITE,
        }
    }

    fn draw_cell(&self) {
        draw_rectangle(self.x_pos, self.y_pos, CELL_SIZE, CELL_SIZE, self.color)
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn intialize_cells() -> Vec<Vec<Cell>> {
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;
    let mut cells = vec![];

    while y_pos < screen_height() {
        let mut row = vec![];
        while x_pos < screen_width() {
            row.push(Cell::new(x_pos, y_pos));
            x_pos += 40.0;
        }
        y_pos += 40.;
        x_pos = 0.0;
        cells.push(row);
    }

    cells
}

#[macroquad::main(window_config)]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    rand::srand(macroquad::miniquad::date::now() as _);
    let cells = intialize_cells();
    let mut g = Game::new(cells);
    loop {
        clear_background(WHITE);
        g.apply_rules();
        g.update();
        g.draw();
        std::thread::sleep(Duration::from_millis(60));
        next_frame().await;
    }
}
