use crate::util::*;

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