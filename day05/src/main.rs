use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut line_iter = BufReader::new(file).lines();
    // part1(&mut line_iter);
    part2(&mut line_iter);
}

fn part1(line_iter: &mut Lines<BufReader<File>>) {
    let mut crates = get_crate_init(line_iter);
    let concised_moves = get_concise_moves(line_iter);
    for m in &concised_moves {
        for i in 0..m[0] {
            let c = crates[m[1] - 1].pop().unwrap();
            crates[m[2] - 1].push(c);
        }
    }
    let mut result: String = "".to_string();
    for mut cr in crates {
        result.push(cr.pop().unwrap());
    }
    println!("result part1: {}", result);
}

fn part2(line_iter: &mut Lines<BufReader<File>>) {
    let mut crates = get_crate_init(line_iter);
    let concised_moves = get_concise_moves(line_iter);
    for m in &concised_moves {
        let split_pos = crates[m[1] - 1].len() - m[0];
        let mut chars = crates[m[1] - 1].split_off(split_pos);
        crates[m[2] - 1].append(&mut chars);
    }
    let mut result: String = "".to_string();
    for mut cr in crates {
        result.push(cr.pop().unwrap());
    }
    println!("result part2: {:?}", result);
}

fn get_crate_init(line_iter: &mut Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut config: Vec<String> = vec![];
    for line in line_iter.map(|l| l.unwrap()) {
        if line.is_empty() {
            break;
        }
        config.push(line);
    }

    let num_crates: usize = config
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap() as usize;

    let mut crates: Vec<Vec<char>> = vec![vec![]; num_crates];

    config.reverse();
    config.remove(0);
    for line in config {
        for (i, c) in line.chars().enumerate() {
            if (i as i32 - 1) % 4 == 0 && c != ' ' {
                crates[(i + 1) / 4].push(c);
            }
        }
    }
    crates
}

fn get_concise_moves(line_iter: &mut Lines<BufReader<File>>) -> Vec<[usize; 3]> {
    let mut concised: Vec<[usize; 3]> = vec![];
    for line in line_iter.map(|l| l.unwrap()) {
        let split: Vec<&str> = line.split_whitespace().collect();
        let split = [
            split[1].parse::<usize>().unwrap(),
            split[3].parse::<usize>().unwrap(),
            split[5].parse::<usize>().unwrap(),
        ];
        concised.push(split);
    }
    concised
}
