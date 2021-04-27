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
    let matches_won: u32 = our_lineup.iter().fold(0, |wins, &our_player| {
        if our_player > their_lineup[wins as usize] {
            return wins + 1;
        }

        wins
    });

    println!("{}", matches_won);
}
