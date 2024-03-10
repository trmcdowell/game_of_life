use std::fmt;

use rand::Rng;

#[derive(Debug)]
pub struct App {
    pub universe: Universe,
}

impl App {
    pub fn new() -> App {
        App {
            universe: Universe::default(),
        }
    }
}

#[derive(Debug)]
pub struct Universe {
    cells: Vec<Cell>,
    height: u32,
    width: u32,
}

impl Universe {
    fn get_cell_idx(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // Count a cell's live neighbors. Uses self.height, self.width, and some
    // clever modulo addition to handle out of bounds errors
    fn count_neighbors(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_cell_idx(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next_gen = self.cells.clone();
        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_cell_idx(row, column);
                let cell = self.cells[idx];
                let neighbor_count = self.count_neighbors(row, column);

                let updated_cell = match cell {
                    Cell::Alive => {
                        if !(2..=3).contains(&neighbor_count) {
                            Cell::Dead
                        } else {
                            cell
                        }
                    }
                    Cell::Dead => {
                        if neighbor_count == 3 {
                            Cell::Alive
                        } else {
                            cell
                        }
                    }
                };

                next_gen[idx] = updated_cell;
            }
        }
        self.cells = next_gen;
    }
}

impl Default for Universe {
    fn default() -> Universe {
        // These dimensions are just based off of my monitor
        let height: u32 = 100;
        let width: u32 = 187;
        let mut cells: Vec<Cell> = Vec::new();
        let num_cells = height * width;
        let mut rng = rand::thread_rng();
        for _ in 0..num_cells {
            let alive: u8 = rng.gen::<u8>() % 2;
            if alive == 0 {
                cells.push(Cell::Alive);
            } else {
                cells.push(Cell::Dead);
            }
        }

        Universe {
            height,
            width,
            cells,
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells.chunks(self.width as usize) {
            for cell in row {
                let cell_str = match cell {
                    Cell::Alive => "◼",
                    Cell::Dead => " ",
                };
                write!(f, "{}", cell_str)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}
