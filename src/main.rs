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

    let mut right_place = vec![false; 4];
    let mut wrong_place = vec![false; 4];
    let mut right = 0;
    let mut wrong = 0;

    for i in 0..4 {
        if comb_chars[i] == guess_chars[i] {
            right_place[i] = true;
            right += 1;
        }
    }

    for (i, &c) in guess_chars.iter().enumerate() {
        if right_place[i] {
            continue;
        }
        if let Some(j) = comb_chars.iter().enumerate()
            .find(|(j, &x)| x == c && !right_place[*j] && !wrong_place[*j])
            .map(|(j, _)| j)
        {
            wrong_place[j] = true;
            wrong += 1;
        }
    }

    (right, wrong)
}

fn write_hits(right: usize, wrong: usize) {
    let hits = match (right, wrong) {
        (0, 0) => {
            "none".to_string()
        },
        _ => {
            "•".repeat(right) + &"○".repeat(wrong)
        }
    };
    println!("Hits: {hits}");
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
    if remaining_combinations.len() == 1 {
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

fn main() {
    let symbols = ['☻', '♣', '♠', '♥', '♦', '♪'];
    let combination: String = (0..4)
    .map(|_| symbols[rand::thread_rng().gen_range(0..symbols.len())])
    .collect();

    println!("Combination: {combination}");
    
    let mut remaining_combinations = all_combinations();
    let all_combinations = all_combinations();
    
    let mut guess: String = String::from("☻☻♣♣");

    while !remaining_combinations.is_empty() {
        println!("Guess:       {guess}");
        let (right, wrong) = determine_hits(&combination, &guess);
        write_hits(right, wrong);

        remaining_combinations.retain(|x| determine_hits(x, &guess) == (right, wrong) && right != 4);
        let l = remaining_combinations.len();
        println!("Remaining possibilities: {l}");
        
        guess.clear();
        guess.push_str(&next_guess(&remaining_combinations, &all_combinations));
    }
    println!("Combination found!")
}
