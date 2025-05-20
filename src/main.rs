use std::io;

use itertools::Itertools;

mod algorithm;
use algorithm::*;

struct GameState {
    remaining_combinations: Vec::<String>,
    current_guess: String,
    game_mode: GameMode,
    combination: String,
    attempt: u8,
    past_guesses: Vec<(String, usize, usize)>,
    game_won: bool,
    num_processed_guesses: usize,
    best_guesses: Vec<String>,
    was_victory_possible: bool,
    chosen_symbol: usize,
    display: Display
}

enum Display {
    KeepGuessing,
    OfferReview,
    ShowReviewOptions,
    ShowReview(u8)
}

enum GameMode {
    Normal,
    WithHelper,
    Simulated
}

impl GameState {
    fn new(game_mode: GameMode) -> Self {
        Self {
            remaining_combinations: if let GameMode::Normal = game_mode { Vec::new() } else { all_combinations() },
            current_guess: if let GameMode::Simulated = game_mode { String::from("☻☻♣♣") } else { String::new() },
            game_mode,
            combination: random_combination(),
            attempt: 1,
            past_guesses: Vec::new(),
            game_won: false,
            num_processed_guesses: 0,
            best_guesses: Vec::new(),
            was_victory_possible: true,
            chosen_symbol: 0,
            display: Display::KeepGuessing
        }
    }

    fn is_finished(&self) -> bool {
        self.game_won || self.attempt > 6
    }

    fn save_current_guess(&mut self) {
        let (right, wrong) = determine_hits(&self.combination, &self.current_guess);
        write_hits(right, wrong);
        self.past_guesses.push((self.current_guess.clone(), right, wrong));

        if right == 4 {
            self.game_won = true;
        }
        self.attempt+=1;
        self.current_guess.clear();
        self.chosen_symbol = 0;
    }
}

fn write_hits(right: usize, wrong: usize) {
    let hits = match (right, wrong) {
        (0, 0) => "none".to_string(),
        _ => "•".repeat(right) + &"○".repeat(wrong),
    };
    println!("{hits}");
}

fn display_chosen_symbol(chosen_symbol: usize) -> String {
    SYMBOLS.iter().enumerate()
    .map(|(i, &x)| if i == chosen_symbol {
        format!("[{}]", x)
    } else {
        format!(" {} ", x)
    }).collect()
}

fn show_past_guesses(guesses: &Vec<(String, usize, usize)>) {
    guesses.iter().enumerate()
        .for_each(|(i, (g, r, w))| {
            let j = i + 1;
            println!("Guess {j}/6:");
            println!("{g}");
            write_hits(*r, *w);
        });
}

fn write_is_possible_guess(was_victory_possible: bool) {
    if was_victory_possible {
        println!("There were optimal moves that could result in the final combination.");
    } else {
        println!("No optimal moves could have resulted in the final combination.");
    }
}

fn read_option() -> String {
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("");
    option.trim().to_uppercase()
}

fn await_key() {
    println!("[press any key to continue]");
    let mut wait = String::new();
    io::stdin().read_line(&mut wait).expect("");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn show_algorithm() {
    clear_screen();
    let mut game_state = GameState::new(GameMode::Simulated);

    println!("Combination: {}\n", game_state.combination);

    loop {
        println!("Guess {}:     {}", game_state.attempt, game_state.current_guess);
        write_is_possible_guess(game_state.was_victory_possible);
        let (right, wrong) = determine_hits(&game_state.combination, &game_state.current_guess);
        write_hits(right, wrong);
        
        if right == 4 {
            break;
        }
        
        game_state.remaining_combinations.retain(|x| determine_hits(x, &game_state.current_guess) == (right, wrong));

        println!("Remaining possibilities: {}", game_state.remaining_combinations.len());
        
        game_state.best_guesses.clear();
        (game_state.best_guesses, game_state.was_victory_possible) = next_best_guesses(&game_state.remaining_combinations);

        game_state.current_guess.clear();
        game_state.current_guess.push_str(&game_state.best_guesses[0]);
        game_state.attempt+= 1;
    }
    println!("Combination found in {} attempts!", game_state.attempt);
    await_key();
}

fn keep_guessing(game_state: &mut GameState) -> bool {
    println!("Guess {}/6:", game_state.attempt);
    println!("{}", game_state.current_guess);

    let display_symbols = display_chosen_symbol(game_state.chosen_symbol);
    println!("{display_symbols}");
    println!("Browse symbols using A and D. Select by pressing enter. Press X to undo.");
    
    let option = read_option();

    match option.as_str() {
        "A" => game_state.chosen_symbol = game_state.chosen_symbol.saturating_sub(1),
        "D" => game_state.chosen_symbol = (game_state.chosen_symbol + 1).min(5),
        "X" => {
            game_state.current_guess.pop();
        },
        "" => game_state.current_guess.push(SYMBOLS[game_state.chosen_symbol]),
        "Q" => return true,
        _ => return false,
    }

    if game_state.current_guess.chars().count() == 4 {
        game_state.save_current_guess();
        if let GameMode::WithHelper = game_state.game_mode {
            game_state.display = Display::OfferReview;
        }
    }

    return false;
}

fn offer_review(game_state: &mut GameState) -> bool {
    println!("If you wish to review your move, press R. Otherwise, press enter.");
                
    let option = read_option();

    match option.as_str() {
        "R" => {
            game_state.display = Display::ShowReviewOptions;

            game_state.past_guesses.iter()
            .skip(game_state.num_processed_guesses)
            .take(game_state.past_guesses.len() - game_state.num_processed_guesses - 1)
            .for_each(|(g, r, w)|{
                game_state.remaining_combinations.retain(|x| determine_hits(x, g) == (*r, *w));
            });

            let (_, right, wrong) = game_state.past_guesses.last().unwrap();

            game_state.best_guesses.clear();
            (game_state.best_guesses, game_state.was_victory_possible) = next_best_guesses(&game_state.remaining_combinations);

            game_state.remaining_combinations.retain(|x| determine_hits(x, &game_state.past_guesses.last().unwrap().0) == (*right, *wrong));
            game_state.num_processed_guesses += game_state.past_guesses.len() - game_state.num_processed_guesses;
        },
        "" =>  {
            game_state.display = Display::KeepGuessing;
        },
        "Q" => return true,
        _ => return false
    }
    return false;
}

fn show_review_options(game_state: &mut GameState) -> bool {
    if game_state.best_guesses.contains(&game_state.past_guesses.last().unwrap().0) {
        println!("Your move was optimal!")
    } else {
        println!("Your move was not optimal.");
    };
    write_is_possible_guess(game_state.was_victory_possible);
    
    println!("To view all optimal moves, press 1.");
    println!("To view optimal moves most similar to yours, press 2.");
    println!("To view all remaining combinations, press 3.");
    println!("To return to the game, press enter.");

    let option = read_option();

    match option.as_str() {
        "" => {
            game_state.display = Display::KeepGuessing;
        },
        "Q" => return true,
        _ => {
            if let Ok(num @ 1..=3) = option.parse::<u8>() {
                game_state.display = Display::ShowReview(num);
            } else {
                return false;
            }
        }
    }
    return false;
}

fn show_review(game_state: &mut GameState) -> bool {
    use Display::*;
    match game_state.display {
        ShowReview(1) => {
            println!("All optimal moves:");
            println!("{}", game_state.best_guesses.join(" "));
        },
        ShowReview(2) => {
            println!("Similar optimal moves:");
            println!("{}", game_state.best_guesses.iter().filter(|x| are_similar(&game_state.past_guesses.last().unwrap().0, x)).join(" "));
        },
        ShowReview(3) => {
            println!("There are {} remaining possible combinations:", game_state.remaining_combinations.len());
            println!("{}", game_state.remaining_combinations.join(" "));
        },
        _ => return false
    }

    game_state.display = ShowReviewOptions;
    await_key();

    return false;
}

fn end_game(game_state: &GameState) {
    if game_state.game_won {
        println!("Combination found in {} attempts!", game_state.attempt - 1);
    } else {
        println!("Out of attempts! The combination was {}.", game_state.combination);
        println!("Better luck next time!");
    }
        
    await_key();
}

fn play_game(game_mode: GameMode) {
    let mut game_state = GameState::new(game_mode);

    loop {
        clear_screen();
        show_past_guesses(&game_state.past_guesses);
        
        if game_state.is_finished() {
            break;
        }

        use Display::*;
        match game_state.display {
            KeepGuessing => {
                if keep_guessing(&mut game_state) {
                    return;
                }
            },
            OfferReview => {
                if offer_review(&mut game_state) {
                    return;
                }
            },
            ShowReviewOptions => {
                if show_review_options(&mut game_state) {
                    return;
                }
            }
            ShowReview(_) => {
                show_review(&mut game_state);
            }
        }
    }

    end_game(&game_state);
}

fn main() {
    loop {
        clear_screen();
        println!("Press the following keys to choose an option:");
        println!("1 - Play game");
        println!("2 - Play game with step-by-step aid");
        println!("3 - Run the algorithm for a random combination");
        println!("Q - Quit (use at any time)");
    
        let option = read_option();

        match option.as_str() {
            "1" => play_game(GameMode::Normal),
            "2" => play_game(GameMode::WithHelper),
            "3" => show_algorithm(),
            "Q" => return,
            _ => continue,
        }
    }
}