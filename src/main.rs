use std::io;

use rand::Rng;
use once_cell::sync::Lazy;
use itertools::Itertools;

static SYMBOLS: [char; 6] = ['☻', '♣', '♠', '♥', '♦', '♪'];
static ALL_COMBINATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    all_combinations()
});

fn all_combinations() -> Vec<String> {
    let mut combinations = Vec::new();
    for &a in &SYMBOLS {
        for &b in &SYMBOLS {
            for &c in &SYMBOLS {
                for &d in &SYMBOLS {
                    combinations.push(format!("{}{}{}{}", a, b, c, d));
                }
            }
        }
    }
    combinations
}

fn all_responses() -> Vec<(u8, u8)> {
    let mut responses = Vec::new();
    for j in 0..=4 {
        let i = 4 - j;
        for k in (0..=i).rev() {
            if (j, k) == (3, 1) {
                continue;
            }
            responses.push((j, k));
        }
    }
    responses
}

fn determine_hits(combination: &str, guess: &str) -> (usize, usize) {
    let comb_chars: Vec<char> = combination.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    let mut right = [false; 4];
    let mut wrong = [false; 4];

    for i in 0..4 {
        right[i] = comb_chars[i] == guess_chars[i];
    }

    for (i, &c) in guess_chars.iter().enumerate() {
        if right[i] {
            continue;
        }
        if let Some(j) = comb_chars.iter().enumerate()
            .find(|(j, &x)| x == c && !right[*j] && !wrong[*j])
            .map(|(j, _)| j)
        {
            wrong[j] = true;
        }
    }

    (right.iter().filter(|&&x| x).count(), wrong.iter().filter(|&&x| x).count())
}

fn write_hits(right: usize, wrong: usize) {
    let hits = match (right, wrong) {
        (0, 0) => "none".to_string(),
        _ => "•".repeat(right) + &"○".repeat(wrong),
    };
    println!("{hits}");
}

fn max_possible_outcomes(remaining_combinations: &Vec<String>, guess: &str, min: usize) -> usize {
    let mut max = 0;
    let responses = all_responses();
    for (right, wrong) in &responses {
        let num_outcomes = remaining_combinations.iter()
        .filter(|&x| determine_hits(x, guess) == (*right as usize, *wrong as usize)).count();
        if num_outcomes > max {
            if num_outcomes > min {
                return num_outcomes;
            }
            max = num_outcomes;
        }
    }
    max
}

fn next_best_guesses(remaining_combinations: &Vec<String>) -> (Vec<String>, bool) {
    let mut best_guesses:Vec<String> = Vec::new();
    if remaining_combinations.len() == 1 {
        return (vec![remaining_combinations[0].to_string()], true);
    }

    let mut min = remaining_combinations.len();

    for guess in ALL_COMBINATIONS.iter() {
        let max = max_possible_outcomes(&remaining_combinations, &guess, min);
        if max < min {
            min = max;
            best_guesses.clear();
            best_guesses.push(String::from(guess));
        } else if max == min {
            best_guesses.push(String::from(guess));
        }
    }

    if let Some(_) = best_guesses.iter().find(|&x| remaining_combinations.contains(x)) {
        best_guesses.retain(|x| remaining_combinations.contains(x));
        return (best_guesses, true);
    }
    
    (best_guesses, false)
}

fn random_combination() -> String {
    (0..4)
    .map(|_| SYMBOLS[rand::thread_rng().gen_range(0..SYMBOLS.len())])
    .collect()
}

fn await_key() {
    println!("[press any key to continue]");
    let mut wait = String::new();
    io::stdin().read_line(&mut wait).expect("");
}

fn show_algorithm() {
    print!("\x1B[2J\x1B[1;1H");
    let combination = random_combination();

    println!("Combination: {combination}\n");
    
    let mut remaining_combinations = all_combinations();
    
    let mut guess: String = String::from("☻☻♣♣");

    let mut best_guesses = Vec::new();
    let mut is_possible_guess = true;

    loop {
        println!("Guess:       {guess}");
        write_is_possible_guess(is_possible_guess);
        let (right, wrong) = determine_hits(&combination, &guess);
        write_hits(right, wrong);
        if right == 4 {
            break;
        }
        
        remaining_combinations.retain(|x| determine_hits(x, &guess) == (right, wrong));

        let l = remaining_combinations.len();
        println!("Remaining possibilities: {l}");
        
        best_guesses.clear();
        (best_guesses, is_possible_guess) = next_best_guesses(&remaining_combinations);

        guess.clear();
        guess.push_str(&best_guesses[0]);
    }
    println!("Combination found!");
    await_key();
}

fn display_chosen_symbol(chosen_symbol: usize) -> String {
    SYMBOLS.iter().enumerate()
    .map(|(i, &x)| if i == chosen_symbol {
        format!("[{}]", x)
    } else {
        format!(" {} ", x)
    }).collect()
}

fn read_option() -> String {
    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("");
    option.trim().to_uppercase()
}

fn are_similar(user_guess: &str, guess: &str) -> bool {
    let (right, wrong) = determine_hits(user_guess, guess);
    right > 2 || wrong > 2 || right + wrong == 4
}

fn write_is_possible_guess(is_possible_guess: bool) {
    if is_possible_guess {
        println!("There were optimal moves that could've resulted in the final combination.");
    } else {
        println!("No optimal moves could've resulted in the final combination.");
    }
}

fn play_game(with_helper: bool) {
    print!("\x1B[2J\x1B[1;1H");
    let combination = random_combination();

    println!("You have 6 attempts to guess the combination.\n");
    let mut attempt: u8 = 0;
    let mut guesses: Vec<(String, usize, usize)> = Vec::new();
    let mut game_won = false;
    let mut remaining_combinations = Vec::new();
    if with_helper {
        remaining_combinations = all_combinations();
    }
    let mut attempt_over = false;
    let mut review_move = false;
    let mut guess = String::new();
    let mut processed_guesses: usize = 0;
    let mut review_display: u8 = 4;
    let mut best_guesses: Vec<String> = Vec::new();
    let mut last_guess = String::new();
    let mut is_possible_guess = true;

    'game_loop: loop {
        attempt+=1;
        guess.clear();
        let mut chosen_symbol = 0;
        loop {
            print!("\x1B[2J\x1B[1;1H");
            
            guesses.iter().enumerate()
                .for_each(|(i, (g, r, w))| {
                    let j = i + 1;
                    println!("Guess {j}/6:");
                    println!("{g}");
                    write_hits(*r, *w);
                });

            if attempt > 6 || game_won {
                break 'game_loop;
            }

            if (!with_helper || !attempt_over) && !review_move {
                println!("Guess {attempt}/6");
                println!("{guess}");
    
                let display_symbols = display_chosen_symbol(chosen_symbol);
                println!("{display_symbols}");
                println!("Browse symbols using A and D. Select by pressing enter. Press X to undo.");
                
                let option = read_option();
    
                match option.as_str() {
                    "A" => chosen_symbol = chosen_symbol.saturating_sub(1),
                    "D" => chosen_symbol = (chosen_symbol + 1).min(5),
                    "X" => {
                        guess.pop();
                    },
                    "" => guess.push(SYMBOLS[chosen_symbol]),
                    "Q" => return,
                    _ => continue,
                }
            } else if attempt_over {
                println!("If you wish to review your move, press R. Otherwise, press enter.");
                
                let option = read_option();
    
                match option.as_str() {
                    "R" => review_move = true,
                    "" =>  (),
                    "Q" => return,
                    _ => continue
                }
            } else {
                if review_display == 4 {
                    guesses.iter()
                    .skip(processed_guesses)
                    .take(guesses.len() - processed_guesses - 1)
                    .for_each(|(g, r, w)|{
                        remaining_combinations.retain(|x| determine_hits(x, g) == (*r, *w));
                    });
                    processed_guesses += guesses.len() - processed_guesses - 1;

                    last_guess.clear();
                    last_guess.push_str(&guesses.last().unwrap().0);

                    best_guesses.clear();
                    (best_guesses, is_possible_guess) = next_best_guesses(&remaining_combinations);

                    review_display = 0;
                }
                
                if review_display == 0 {
                    if best_guesses.contains(&last_guess) {
                        println!("Your move was optimal!")
                    } else {
                        println!("Your move was not optimal.");
                    };
                    write_is_possible_guess(is_possible_guess);
                    
                    println!("To view all optimal moves, press 1.");
                    println!("To view optimal moves most similar to yours, press 2.");
                    println!("To view all remaining combinations, press 3.");
                    println!("To return to the game, press enter.");
    
                    let option = read_option();
    
                    match option.as_str() {
                        "" => {
                            review_move = false;
                            review_display = 4;
                        },
                        "Q" => return,
                        _ => {
                            if let Ok(num @ 1..=3) = option.parse::<u8>() {
                                review_display = num;
                            } else {
                                continue;
                            }
                        }
                    }
                } else {
                    match review_display {
                        1 => {
                            println!("All optimal moves:");
                            println!("{}", best_guesses.join(" "));
                        },
                        2 => {
                            println!("Similar optimal moves:");
                            println!("{}", best_guesses.iter().filter(|x| are_similar(&last_guess, x)).join(" "));
                        },
                        3 => {
                            println!("There are {} remaining possible combinations:", remaining_combinations.len());
                            println!("{}", remaining_combinations.join(" "));
                        },
                        _ => continue
                    }
                    review_display = 0;
                    await_key();
                }
                
            }

            attempt_over = false;
            if guess.chars().count() == 4 {
                break;
            }
        }
        let (right, wrong) = determine_hits(&combination, &guess);
        write_hits(right, wrong);
        guesses.push((guess.clone(), right, wrong));
        attempt_over = true;

        if right == 4 {
            game_won = true;
        }
    }
    
    if game_won {
        attempt -= 1;
        println!("Combination found in {attempt} attempts!");
    } else {
        println!("Out of attempts! The combination was {combination}.");
        println!("Better luck next time!");
    }
    
    await_key();
}

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Press the following keys to choose an option:");
        println!("1 - Play game");
        println!("2 - Play game with step-by-step aid");
        println!("3 - Run the algorithm for a random combination");
        println!("Q - Quit (use at any time)");
    
        let option = read_option();

        match option.as_str() {
            "1" => play_game(false),
            "2" => play_game(true),
            "3" => show_algorithm(),
            "Q" => {
                print!("\x1B[2J\x1B[1;1H");
                return;
            },
            _ => continue,
        }
    }
}