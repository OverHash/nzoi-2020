use std::io::{self, BufRead};

fn main() {
    let mut year_readings: [Vec<u16>; 3] = Default::default();

    // skip first integer
    // iterate every 2nd loop
    // three times
    for (i, line) in io::stdin()
        .lock()
        .lines()
        .skip(1)
        .step_by(2)
        .take(3)
        .enumerate()
    {
        let line = line.unwrap();
        year_readings[i] = line
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    }

    let all_readings = year_readings
        .iter()
        .fold(Vec::new(), |mut all_years, year| {
            for reading in year {
                all_years.push(*reading);
            }
            all_years
        });

    if usize::from(all_readings.iter().sum::<u16>()) >= 12 * all_readings.len() {
        // check for resow
        let recent_year_readings = &year_readings[2];

        let saturated_recent_year_readings: Vec<&u16> = recent_year_readings
            .iter()
            .filter(|reading| *reading >= &25)
            .collect();

        if saturated_recent_year_readings.len() * 2 >= recent_year_readings.len() {
            println!("resow");
        } else {
            println!("unhealthy");
        }
    } else {
        println!("healthy");
    }
}
