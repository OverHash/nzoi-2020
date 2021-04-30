use std::{
    cmp::min,
    convert::TryInto,
    io::{self, BufRead},
};

fn main() {
    // skip first line (tells us how many elements)
    std::io::stdin().read_line(&mut String::new()).unwrap();

    // read our trails
    let mut trails_buff = String::new();
    io::stdin().lock().read_line(&mut trails_buff).unwrap();

    let trails: Vec<u32> = trails_buff
        .split_whitespace()
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let clockwise_cost = solve(&trails);

    let anticlockwise_cost = {
        let mut trails = trails;
        trails.reverse();

        solve(&trails)
    };

    println!("{}", min(clockwise_cost, anticlockwise_cost));
}

fn calculate_first_difficulty(trail_ratings: &[u32]) -> u64 {
    trail_ratings.iter().enumerate().fold(0, |acc, (i, hut)| {
        acc + (hut * (trail_ratings.len() - i) as u32) as u64
    })
}

fn solve(trails: &[u32]) -> u64 {
    let trail_len = trails.len() as u64;

    let sum_ratings: u64 = trails.iter().sum::<u32>().try_into().unwrap();

    let mut current_cost = calculate_first_difficulty(trails);
    let mut minimum_cost = current_cost;

    for hut_index in 1..trails.len() {
        current_cost = current_cost + sum_ratings - trail_len * trails[hut_index - 1] as u64;
        minimum_cost = min(current_cost, minimum_cost);
    }

    minimum_cost
}
