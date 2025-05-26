use crate::state::GameState;

pub trait View {
    fn new() -> Self;
    fn read_main_menu_input(&self) -> String;
    fn show_past_guesses(&self, guesses: &Vec<(String, usize, usize)>);
    fn read_guessing_input(&self, game_state: &GameState) -> String;
    fn read_review_offer_input(&self) -> String;
    fn read_review_type_input(&self, game_state: &GameState) -> String;
    fn show_review(&self, game_state: &GameState);
    fn show_end_game(&self, game_state: &GameState);
    fn show_combination(&self, combination: &str);
    fn show_simulated_guess(&self, game_state: &GameState, right: usize, wrong: usize);
    fn show_algorithm_end(&self, attempt: u8);
}