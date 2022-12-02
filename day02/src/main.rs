use std::fs;
fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut total_points: i32 = 0;
    for line in input.lines() {
        let mut points: i32 = hand_number(line.chars().nth(2).unwrap()) + 1;
        let result = (hand_number(line.chars().nth(0).unwrap()) - points + 3) % 3;
        match result {
            0 => total_points += points + 3,
            1 => total_points += points,
            2 => total_points += points + 6,
            _ => (),
        }
    }
    println!("result part1: {}", total_points);
}

fn hand_number(hand: char) -> i32 {
    match hand {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("invalid input"),
    }
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut total_points: i32 = 0;
    for line in input.lines() {
        let hand_opp = hand_number(line.chars().nth(0).unwrap());
        let match_result = line.chars().nth(2).unwrap();

        total_points += match match_result {
            'X' => (hand_opp + 2) % 3 + 1,
            'Y' => hand_opp + 1 + 3,
            'Z' => (hand_opp + 1) % 3 + 1 + 6,
            _ => panic!("invalid input"),
        };
    }
    println!("result part2: {}", total_points);
}
