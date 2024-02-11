use macroquad::prelude::*;

const CELL_SIZE: f32 = 40.;

struct Game {
    cells: Vec<Vec<Cell>>,
}

impl Game {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        Self { cells }
    }

    fn draw(&self) {
        for r in &self.cells {
            for c in r {
                c.draw_cell();
            }
        }
    }
}

enum Status {
    Alive,
    Dead,
}

struct Cell {
    x_pos: f32,
    y_pos: f32,
}

impl Cell {
    fn new(x_pos: f32, y_pos: f32) -> Self {
        Self { x_pos, y_pos }
    }

    fn draw_cell(&self) {
        draw_rectangle(self.x_pos, self.y_pos, CELL_SIZE, CELL_SIZE, BLACK)
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

fn grid_setup() {
    let mut x_pos = 0.0;
    let mut y_pos = 0.0;

    while y_pos < screen_height() {
        println!("y off {}", y_pos);
        while x_pos < screen_width() {
            println!("{y_pos}");
            draw_rectangle(x_pos, y_pos, CELL_SIZE, CELL_SIZE, BLACK);
            x_pos += 40.0;
        }
        y_pos += 40.;
        x_pos = 0.0;
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let cells = intialize_cells();
    let g = Game::new(cells);
    loop {
        clear_background(WHITE);
        g.draw();
        next_frame().await;
    }
}
