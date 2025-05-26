use crate::util::SYMBOLS;
use std::io;
use crate::view::view::View;
use crate::state::{GameState, Display};

pub struct CLI;

impl CLI {
    fn write_hits(&self, right: usize, wrong: usize) {
        let hits = match (right, wrong) {
            (0, 0) => "none".to_string(),
            _ => "•".repeat(right) + &"○".repeat(wrong),
        };
        println!("{hits}");
    }

    fn display_chosen_symbol(&self, chosen_symbol: usize) -> String {
        SYMBOLS.iter().enumerate()
        .map(|(i, &x)| if i == chosen_symbol {
            format!("[{}]", x)
        } else {
            format!(" {} ", x)
        }).collect()
    }

    fn write_was_victory_possible(&self, was_victory_possible: bool) {
        if was_victory_possible {
            println!("There were optimal moves that could result in the final combination.");
        } else {
            println!("No optimal moves could have resulted in the final combination.");
        }
    }

    fn read_option(&self) -> String {
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("");
        option.trim().to_uppercase()
    }

    fn await_key(&self) {
        println!("[press any key to continue]");
        let mut wait = String::new();
        io::stdin().read_line(&mut wait).expect("");
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}

impl View for CLI {
    fn new() -> Self {
        CLI
    }

    fn read_main_menu_input(&self) -> String {
        self.clear_screen();
        println!("Press the following keys to choose an option:");
        println!("1 - Play game");
        println!("2 - Play game with step-by-step aid");
        println!("3 - Run the algorithm for a random combination");
        println!("Q - Quit (use at any time)");
    
        self.read_option()
    }

    fn show_past_guesses(&self, guesses: &Vec<(String, usize, usize)>) {
        self.clear_screen();
        guesses.iter().enumerate()
            .for_each(|(i, (g, r, w))| {
                println!("Guess {}/6:", i + 1);
                println!("{g}");
                self.write_hits(*r, *w);
            });
    }

    fn read_guessing_input(&self, game_state: &GameState) -> String {
        println!("Guess {}/6:", game_state.attempt);
        println!("{}", game_state.current_guess);

        let display_symbols = self.display_chosen_symbol(game_state.chosen_symbol);
        println!("{display_symbols}");
        println!("Browse symbols using A and D. Select by pressing enter. Press X to undo.");
        
        self.read_option()
    }

    fn read_review_offer_input(&self) -> String {
        println!("If you wish to review your move, press R. Otherwise, press enter.");
        self.read_option()
    }

    fn read_review_type_input(&self, game_state: &GameState) -> String {
        if game_state.best_guesses.contains(&game_state.past_guesses.last().unwrap().0) {
            println!("Your move was optimal!")
        } else {
            println!("Your move was not optimal.");
        };
        self.write_was_victory_possible(game_state.was_victory_possible);
        
        println!("To view all optimal moves, press 1.");
        println!("To view optimal moves most similar to yours, press 2.");
        println!("To view all remaining combinations, press 3.");
        println!("To return to the game, press enter.");

        self.read_option()
    }

    fn show_review(&self, game_state: &GameState) {
        use Display::ShowReview;
        match game_state.display {
            ShowReview(1) => {
                println!("All optimal moves:");
                println!("{}", game_state.best_guesses.join(" "));
            },
            ShowReview(2) => {
                println!("Similar optimal moves:");
                println!("{}", game_state.similar_best_guesses().join(" "));
            },
            ShowReview(3) => {
                println!("There are {} remaining possible combinations:", game_state.remaining_combinations.len());
                println!("{}", game_state.remaining_combinations.join(" "));
            },
            _ => return
        }

        self.await_key();
    }

    fn show_end_game(&self, game_state: &GameState) {
        if game_state.game_won {
            println!("Combination found in {} attempts!", game_state.attempt - 1);
        } else {
            println!("Out of attempts! The combination was {}.", game_state.combination);
            println!("Better luck next time!");
        }
            
        self.await_key();
    }

    fn show_combination(&self, combination: &str) {
        self.clear_screen();
        println!("Combination: {}\n", combination);
    }

    fn show_simulated_guess(&self, game_state: &GameState, right: usize, wrong: usize) {
        println!("Guess {}:     {}", game_state.attempt, game_state.current_guess);
        self.write_was_victory_possible(game_state.was_victory_possible);
        self.write_hits(right, wrong);
        println!("Number of remaining possibilities: {}", game_state.remaining_combinations.len());
    }

    fn show_algorithm_end(&self, attempt: u8) {
        println!("Combination found in {} attempts!", attempt);
        self.await_key();
    }
}