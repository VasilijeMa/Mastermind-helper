use rand::Rng;
use once_cell::sync::Lazy;

pub static SYMBOLS: [char; 6] = ['☻', '♣', '♠', '♥', '♦', '♪'];
static ALL_COMBINATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    all_combinations()
});
static ALL_RESPONSES: Lazy<Vec<(usize, usize)>> = Lazy::new(|| {
    all_responses()
});

pub fn all_combinations() -> Vec<String> {
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

fn all_responses() -> Vec<(usize, usize)> {
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

pub fn determine_hits(combination: &str, guess: &str) -> (usize, usize) {
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

fn max_possible_outcomes(remaining_combinations: &Vec<String>, guess: &str, min: usize) -> usize {
    let mut max = 0;
    for (right, wrong) in ALL_RESPONSES.iter() {
        let num_outcomes = remaining_combinations.iter()
        .filter(|&x| determine_hits(x, guess) == (*right, *wrong)).count();
        if num_outcomes > max {
            if num_outcomes > min {
                return num_outcomes;
            }
            max = num_outcomes;
        }
    }
    max
}

pub fn next_best_guesses(remaining_combinations: &Vec<String>) -> (Vec<String>, bool) {
    let mut best_guesses:Vec<String> = Vec::new();
    if remaining_combinations.len() == 1 {
        return (vec![remaining_combinations[0].to_string()], true);
    }

    let mut min = remaining_combinations.len();

    ALL_COMBINATIONS.iter().for_each(|guess| {
        let max = max_possible_outcomes(&remaining_combinations, guess, min);
        if max < min {
            min = max;
            best_guesses.clear();
            best_guesses.push(String::from(guess));
        } else if max == min {
            best_guesses.push(String::from(guess));
        }
    });

    if let Some(_) = best_guesses.iter().find(|&x| remaining_combinations.contains(x)) {
        best_guesses.retain(|x| remaining_combinations.contains(x));
        return (best_guesses, true);
    }
    
    (best_guesses, false)
}

pub fn random_combination() -> String {
    (0..4)
    .map(|_| SYMBOLS[rand::thread_rng().gen_range(0..SYMBOLS.len())])
    .collect()
}

pub fn are_similar(user_guess: &str, guess: &str) -> bool {
    let (right, wrong) = determine_hits(user_guess, guess);
    right > 2 || wrong > 2 || right + wrong == 4
}