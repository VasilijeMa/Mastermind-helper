mod util;
mod state;
mod view {
    pub mod view;
    pub mod cli;
}
mod algorithm;
mod game;

use state::GameMode;
use view::view::View;

fn main() {
    let view = view::cli::CLI::new();
    loop {
        match view.read_main_menu_input().as_str() {
            "1" => game::play_game(GameMode::Normal, &view),
            "2" => game::play_game(GameMode::WithHelper, &view),
            "3" => game::run_algorithm(&view),
            "Q" => return,
            _ => continue,
        }
    }
}