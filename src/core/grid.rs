use crate::core::cell::Cell;

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::Dead; width * height];

        Self {
            height,
            width,
            cells,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        assert!(
            x < self.width && y < self.height,
            "Coordinates out of bounds"
        );

        self.cells[y * self.width + x]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        assert!(
            x < self.width && y < self.height,
            "Coordinates out of bounds"
        );

        self.cells[y * self.width + x] = cell;
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut alive_neighbors = 0;

        for dx in [self.width - 1, 0, 1] {
            for dy in [self.height - 1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x + dx) % self.width;
                let ny = (y + dy) % self.height;

                if self.get_cell(nx, ny).is_alive() {
                    alive_neighbors += 1;
                }
            }
        }
        alive_neighbors
    }
}
