use std::io;

use rand::Rng;

fn all_combinations() -> Vec<String> {
    let symbols = ['☻', '♣', '♠', '♥', '♦', '♪'];
    let mut combinations = Vec::new();
    for &a in &symbols {
        for &b in &symbols {
            for &c in &symbols {
                for &d in &symbols {
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

fn next_guess(remaining_combinations: &Vec<String>, all_combinations: &Vec<String>) -> String {
    if remaining_combinations.len() < 3 {
        return remaining_combinations[0].to_string();
    }

    let mut min = remaining_combinations.len();
    let mut best_guess = String::from("");

    for guess in all_combinations {
        let max = max_possible_outcomes(&remaining_combinations, guess, min);
        if max < min {
            min = max;
            best_guess.clear();
            best_guess.push_str(guess);
        }
    }
    
    best_guess
}

fn random_combination() -> String {
    let symbols = ['☻', '♣', '♠', '♥', '♦', '♪'];
    (0..4)
    .map(|_| symbols[rand::thread_rng().gen_range(0..symbols.len())])
    .collect()
}

fn await_key() {
    println!("[press any key to continue]");
    let mut wait = String::new();
    io::stdin().read_line(&mut wait).expect("");
}

fn run_algorithm() {
    print!("\x1B[2J\x1B[1;1H");
    let combination = random_combination();

    println!("Combination: {combination}\n");
    
    let mut remaining_combinations = all_combinations();
    let all_combinations = all_combinations();
    
    let mut guess: String = String::from("☻☻♣♣");

    loop {
        println!("Guess:       {guess}");
        let (right, wrong) = determine_hits(&combination, &guess);
        write_hits(right, wrong);
        if right == 4 {
            break;
        }

        remaining_combinations.retain(|x| determine_hits(x, &guess) == (right, wrong));

        let l = remaining_combinations.len();
        println!("Remaining possibilities: {l}");
        
        guess.clear();
        guess.push_str(&next_guess(&remaining_combinations, &all_combinations));
    }
    println!("Combination found!");
    await_key();
}

fn display_chosen_symbol(chosen_symbol: usize) -> String {
    let symbols = ['☻', '♣', '♠', '♥', '♦', '♪'];
    symbols.iter().enumerate()
    .map(|(i, &x)| if i == chosen_symbol {
        format!("[{}]", x)
    } else {
        format!(" {} ", x)
    }).collect()
}

fn play_game() {
    let symbols = ['☻', '♣', '♠', '♥', '♦', '♪'];
    print!("\x1B[2J\x1B[1;1H");
    let combination = random_combination();

    println!("You have 6 attempts to guess the combination.\n");
    let mut attempt: u8 = 0;
    let mut guesses = Vec::new();
    let mut game_won = false;

    'game_loop: loop {
        attempt+=1;
        let mut guess = String::new();
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

            println!("Guess {attempt}/6");
            println!("{guess}");

            let display_symbols = display_chosen_symbol(chosen_symbol);
            println!("{display_symbols}");
            println!("Choose symbol using A and D. Select by pressing enter. Press X to undo.");
            
            let mut option = String::new();
            io::stdin().read_line(&mut option).expect("");
            let option = option.trim().to_uppercase();

            match option.as_str() {
                "A" => chosen_symbol = chosen_symbol.saturating_sub(1),
                "D" => chosen_symbol = (chosen_symbol + 1).min(5),
                "X" => {
                    guess.pop();
                },
                "" => {
                    guess.push(symbols[chosen_symbol]);
                },
                _ => (),
            }

            if guess.chars().count() == 4 {
                break;
            }
        }
        let (right, wrong) = determine_hits(&combination, &guess);
        write_hits(right, wrong);
        guesses.push((guess.clone(), right, wrong));

        if right == 4 {
            game_won = true;
        }
    }
    
    if game_won {
        println!("Combination found!");
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
        println!("Q - Quit");
    
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("");
        let option = option.trim().to_uppercase();

        match option.as_str() {
            "1" => play_game(),
            "3" => run_algorithm(),
            "Q" => break,
            _ => continue,
        }
    }
}
