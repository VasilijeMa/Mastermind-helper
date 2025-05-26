use crate::state::*;
use crate::util::{SYMBOLS, determine_hits};
use crate::view::view::View;

fn user_interaction(game_state: &mut GameState, view: &impl View) -> bool {
    use Display::*;
    match game_state.display {
        KeepGuessing => {
            match view.read_guessing_input(&game_state).as_str() {
                "A" => game_state.chosen_symbol = game_state.chosen_symbol.saturating_sub(1),
                "D" => game_state.chosen_symbol = (game_state.chosen_symbol + 1).min(5),
                "X" => { game_state.current_guess.pop(); },
                "" => game_state.current_guess.push(SYMBOLS[game_state.chosen_symbol]),
                "Q" => return true,
                _ => return false,
            }

            if game_state.current_guess.chars().count() == 4 {
                game_state.save_current_guess();
            }
        },
        OfferReview => {
            match view.read_review_offer_input().as_str() {
                "R" => game_state.process_past_guesses(),
                "" => game_state.display = Display::KeepGuessing,
                "Q" => return true,
                _ => return false
            }
        },
        ShowReviewOptions => {
            let option = view.read_review_type_input(&game_state);
            match option.as_str() {
                "" => game_state.display = Display::KeepGuessing,
                "Q" => return true,
                _ => {
                    if let Ok(num @ 1..=3) = option.parse::<u8>() {
                        game_state.display = Display::ShowReview(num);
                    } else {
                        return false;
                    }
                }
            }
        }
        ShowReview(_) => {
            view.show_review(&game_state);
            game_state.display = ShowReviewOptions;
        }
    }

    false
}

pub fn play_game(game_mode: GameMode, view: &impl View) {
    let mut game_state = GameState::new(game_mode);

    loop {
        view.show_past_guesses(&game_state.past_guesses);
        
        if game_state.game_won || game_state.attempt > 6 {
            break;
        }

        if user_interaction(&mut game_state, view) {
            return;
        }
    }

    view.show_end_game(&game_state);
}

pub fn run_algorithm(view: &impl View) {
    let mut game_state = GameState::new(GameMode::Simulated);

    view.show_combination(&game_state.combination);

    loop {
        let (right, wrong) = determine_hits(&game_state.combination, &game_state.current_guess);
        
        game_state.remaining_combinations.retain(|x| determine_hits(x, &game_state.current_guess) == (right, wrong));

        view.show_simulated_guess(&game_state, right, wrong);
        
        if right == 4 {
            break;
        }

        game_state.next_simulated_guess();
    }
    view.show_algorithm_end(game_state.attempt);
}