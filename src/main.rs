use rand::Rng;

fn determine_hits(combination: &str, guess: &str) -> (u8, u8) {
    let comb_chars: Vec<char> = combination.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    let mut right_place = vec![false; comb_chars.len()];
    let mut wrong_place = vec![false; comb_chars.len()];
    let mut right = 0;
    let mut wrong = 0;

    for i in 0..comb_chars.len() {
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

fn main() {
    let symbols = ['☻', '♥', '♦', '♣', '♠', '♪'];
    let _hit_types = ['•', '○'];
    // let combination: String = String::from("☻☻♥♠");
    let combination: String = (0..4)
    .map(|_| symbols[rand::thread_rng().gen_range(0..symbols.len())])
    .collect();
    println!("Combination: {combination}");
    // let guess: String = String::from("☻♥♠♦");
    let guess: String = (0..4)
    .map(|_| symbols[rand::thread_rng().gen_range(0..symbols.len())])
    .collect();
    println!("Guess:       {guess}");
    let (right, wrong) = determine_hits(&combination, &guess);
    println!("{right}, {wrong}");
}
