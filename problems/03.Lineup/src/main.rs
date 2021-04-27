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

    // read their lineup
    lineup_buff = String::new();
    io::stdin().lock().read_line(&mut lineup_buff).unwrap();

    let their_lineup: Vec<u32> = lineup_buff
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    // find our optimal layout
    let mut optimal_layout: Vec<u32> = Vec::with_capacity(our_lineup.len());

    for their_player in &their_lineup {
        let mut closest_rank = u32::MAX;
        let mut closest_rank_index: Option<usize> = None;

        for (i, our_player) in our_lineup.iter().enumerate() {
            if our_player < &closest_rank && our_player > their_player {
                closest_rank = *our_player;
                closest_rank_index = Some(i);
            }
        }

        let best_rank_index = match closest_rank_index {
            Some(v) => v,
            None => {
                let mut lowest_num = our_lineup[0];
                let mut lowest_num_index: usize = 0;

                for (i, player) in our_lineup.iter().skip(1).enumerate() {
                    if player < &lowest_num {
                        lowest_num = *player;
                        lowest_num_index = i;
                    }
                }

                lowest_num_index
            }
        };

        optimal_layout.push(our_lineup[best_rank_index]);
        our_lineup.swap_remove(best_rank_index);
    }

    let matches_won = optimal_layout
        .iter()
        .enumerate()
        .fold(0, |acc: u32, (index, our_player)| {
            if our_player > &their_lineup[index] {
                return acc + 1;
            }

            acc
        });

    println!("{}", matches_won);
}
