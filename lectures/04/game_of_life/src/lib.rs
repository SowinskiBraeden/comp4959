use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = if *self == Cell::Alive {
            Cell::Dead
        } else {
            Cell::Alive
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let (width, height) = (64, 64);
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn count_live_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for i in [self.height - 1, 0, 1] {
            for j in [self.width - 1, 0, 1] {
                if i == 0 && j == 0 {
                    continue;
                }
                let idx = self.get_index((row + i) % self.height, (col + j) % self.width);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut cells = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                cells[idx] = match (self.cells[idx], self.count_live_neighbours(row, col)) {
                    (_, 3) | (Cell::Alive, 2) => Cell::Alive,
                    _ => Cell::Dead,
                }
            }
        }
        self.cells = cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn set_alive(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = Cell::Alive;
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::Dead;
        }
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let glyph = if self.cells[idx] == Cell::Alive {
                    '⬛'
                } else {
                    '🔳'
                };
                write!(f, "{glyph}");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
