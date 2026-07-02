use crate::core::Grid;
use eframe::egui;

const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const CELL_SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Pause,
    Play,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    grid: Grid,
    state: State,
}

impl Game {
    pub fn new() -> Self {
        let grid = Grid::new(WIDTH, HEIGHT);
        let state = State::Pause;
        Self { grid, state }
    }
}

impl eframe::App for Game {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::hover());
        
    }
}