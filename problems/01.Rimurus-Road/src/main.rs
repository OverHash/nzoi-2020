use std::io;

fn main() {
    let mut input_distances = String::new();
    io::stdin().read_line(&mut input_distances).unwrap();

    let distances = input_distances
		.trim_end()
        .split(' ')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut distance_from_start = String::new();
    io::stdin().read_line(&mut distance_from_start).unwrap();

    let distance_from_start = distance_from_start.trim_end().parse::<u32>().unwrap();

    let mut all_distances = distances.iter().fold(Vec::new(), |mut tmp, &distance| {
        let from_start = distance - (distance_from_start % distance);
        let to_end = from_start + distance;

        tmp.push(from_start);
        if from_start != to_end {
            tmp.push(to_end);
        }
        tmp
    });

    all_distances.sort_unstable();

    println!("{}", all_distances[0]);

    if all_distances[0] == all_distances[1] {
        println!("can't make up my mind");
    }
}
