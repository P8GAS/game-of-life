#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        matches!(self, Cell::Alive)
    }

    pub fn is_dead(&self) -> bool {
        matches!(self, Cell::Dead)
    }

    pub fn next_state(&self, neighbors: u8) -> Cell {
        if self.is_alive() {
            match neighbors {
                2 | 3 => Cell::Alive,
                _ => Cell::Dead,
            }
        } else {
            match neighbors {
                3 => Cell::Alive,
                _ => Cell::Dead,
            }
        }
    }
}
