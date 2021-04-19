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

    if calculate_year_mean(all_readings) >= 12 {
        // check for resow
        let recent_year_readings = &year_readings[2];

        let saturated_recent_year_readings: Vec<&u16> = recent_year_readings
            .iter()
            .filter(|reading| *reading >= &25)
            .collect();

        if saturated_recent_year_readings.len() >= recent_year_readings.len() / 2 {
            println!("resow");
        } else {
            println!("unhealthy");
        }
    } else {
        println!("healthy");
    }
}

fn calculate_year_mean(readings: Vec<u16>) -> u16 {
    let sum: u32 = readings.iter().fold(0_u32, |mut acc, reading| {
        acc += *reading as u32;
        acc
    });

    (sum / readings.len() as u32) as u16
}
