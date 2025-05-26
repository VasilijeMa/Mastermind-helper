use rand::Rng;
use once_cell::sync::Lazy;

pub static SYMBOLS: [char; 6] = ['☻', '♣', '♠', '♥', '♦', '♪'];
pub static ALL_COMBINATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    all_combinations()
});
pub static ALL_RESPONSES: Lazy<Vec<(usize, usize)>> = Lazy::new(|| {
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

pub fn random_combination() -> String {
    (0..4)
    .map(|_| SYMBOLS[rand::thread_rng().gen_range(0..SYMBOLS.len())])
    .collect()
}

pub fn are_similar(user_guess: &str, guess: &str) -> bool {
    let (right, wrong) = determine_hits(user_guess, guess);
    right > 2 || wrong > 2 || right + wrong == 4
}