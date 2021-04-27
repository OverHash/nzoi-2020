use std::io::{self, BufRead};

fn main() {
    // skip first line (tells us how many elements)
    std::io::stdin().read_line(&mut String::new()).unwrap();

    // read our lineup
    let mut lineup_buff = String::new();
    io::stdin().lock().read_line(&mut lineup_buff).unwrap();

    let mut our_lineup: Vec<u32> = lineup_buff
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    our_lineup.sort_unstable();

    // read their lineup
    lineup_buff = String::new();
    io::stdin().lock().read_line(&mut lineup_buff).unwrap();

    let mut their_lineup: Vec<u32> = lineup_buff
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    their_lineup.sort_unstable();

    // calculate wins
    let mut matches_won: u32 = 0;

    for our_player in &our_lineup {
        if our_player > &their_lineup[matches_won as usize] {
            matches_won += 1;
        }
    }

    println!("{}", matches_won);
}
