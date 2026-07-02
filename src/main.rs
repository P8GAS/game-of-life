mod app;
mod core;

use app::Game;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Ok(Box::new(Game::new()))),
    )
}   