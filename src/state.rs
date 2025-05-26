use crate::util::*;
use crate::algorithm::next_best_guesses;

pub enum Display {
    KeepGuessing,
    OfferReview,
    ShowReviewOptions,
    ShowReview(u8)
}

pub enum GameMode {
    Normal,
    WithHelper,
    Simulated
}

pub struct GameState {
    pub remaining_combinations: Vec::<String>,
    pub current_guess: String,
    game_mode: GameMode,
    pub combination: String,
    pub attempt: u8,
    pub past_guesses: Vec<(String, usize, usize)>,
    pub game_won: bool,
    num_processed_guesses: usize,
    pub best_guesses: Vec<String>,
    pub was_victory_possible: bool,
    pub chosen_symbol: usize,
    pub display: Display
}

impl GameState {
    pub fn new(game_mode: GameMode) -> Self {
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

    pub fn save_current_guess(&mut self) {
        let (right, wrong) = determine_hits(&self.combination, &self.current_guess);
        // write_hits(right, wrong);
        self.past_guesses.push((self.current_guess.clone(), right, wrong));

        if right == 4 {
            self.game_won = true;
        }
        self.attempt+=1;
        self.current_guess.clear();
        self.chosen_symbol = 0;

        if let GameMode::WithHelper = self.game_mode {
            self.display = Display::OfferReview;
        }
    }

    pub fn similar_best_guesses(&self) -> Vec<String> {
        self.best_guesses.iter()
            .filter(|&x| are_similar(&self.past_guesses.last().unwrap().0, x))
            .cloned()
            .collect()
    }

    pub fn process_past_guesses(&mut self) {
        self.display = Display::ShowReviewOptions;

        self.past_guesses.iter()
        .skip(self.num_processed_guesses)
        .take(self.past_guesses.len() - self.num_processed_guesses - 1)
        .for_each(|(g, r, w)|{
            self.remaining_combinations.retain(|x| determine_hits(x, g) == (*r, *w));
        });

        self.best_guesses.clear();
        (self.best_guesses, self.was_victory_possible) = next_best_guesses(&self.remaining_combinations);
        
        let (_, right, wrong) = self.past_guesses.last().unwrap();
        self.remaining_combinations.retain(|x| determine_hits(x, &self.past_guesses.last().unwrap().0) == (*right, *wrong));

        self.num_processed_guesses += self.past_guesses.len() - self.num_processed_guesses;
    }

    pub fn next_simulated_guess(&mut self) {
        self.best_guesses.clear();
        (self.best_guesses, self.was_victory_possible) = next_best_guesses(&self.remaining_combinations);

        self.current_guess.clear();
        self.current_guess.push_str(&self.best_guesses[0]);
        self.attempt+=1;
    }
}