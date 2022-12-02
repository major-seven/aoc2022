use std::fs;
fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut most_calories = 0;
    let mut current_calories = 0;
    for line in input.lines() {
        if line.len() != 0 {
            let cal: u32 = line.parse().unwrap();
            current_calories += cal;
            continue;
        }
        if current_calories > most_calories {
            most_calories = current_calories;
        }
        current_calories = 0;
    }
    if current_calories > most_calories {
        most_calories = current_calories;
    }
    println!("result part1: {}", most_calories);
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut top_three: [u32; 3] = [0; 3];
    let mut current_calories = 0;
    for line in input.lines() {
        if line.len() != 0 {
            let cal: u32 = line.parse().unwrap();
            current_calories += cal;
            continue;
        }
        for i in 0..3 {
            if top_three[i] < current_calories {
                let temp = top_three[i];
                top_three[i] = current_calories;
                current_calories = temp;
            }
        }
        current_calories = 0;
    }
    for i in 0..3 {
        if top_three[i] < current_calories {
            let temp = top_three[i];
            top_three[i] = current_calories;
            current_calories = temp;
        }
    }
    let sum: u32 = top_three.iter().sum();
    println!("result part2 {}", sum);
}
