use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Rocket {
    cost: u16,
    fuel: u16,
}

fn main() {
    // read meta-info
    let mut input_buff = String::new();
    stdin().lock().read_line(&mut input_buff).unwrap();

    let input = input_buff
        .split_whitespace()
        .map(str::parse::<u16>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let rocket_count = input.get(0).unwrap();
    let player_count = input.get(1).unwrap();
    let tile_count = input.get(2).unwrap();

    // read rockets
    let mut rockets: Vec<Rocket> = Vec::with_capacity(*rocket_count as usize);
    for _ in 0..*rocket_count {
        let mut rocket_info_buff = String::new();
        stdin().lock().read_line(&mut rocket_info_buff).unwrap();

        let rocket_info = rocket_info_buff
            .split_whitespace()
            .map(str::parse::<u16>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        rockets.push(Rocket {
            cost: rocket_info[0],
            fuel: rocket_info[1],
        });
    }

    // read players
    let mut players: Vec<u16> = Vec::with_capacity(*player_count as usize);
    for _ in 0..*player_count {
        let mut player_info_buff = String::new();
        stdin().lock().read_line(&mut player_info_buff).unwrap();

        let player_position = player_info_buff.trim_end().parse::<u16>().unwrap();
        players.push(player_position);
    }

    // solve for each player
    for player in &players {
        println!("{}", player);
    }

    println!("{:?}, {:?}", rockets, players);
}
