use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input.txt").unwrap();
    let line_iter = BufReader::new(file).lines();
    // part1(line_iter);
    part2(line_iter);
}

fn part1(line_iter: Lines<BufReader<File>>) {
    let mut result: i32 = 0;

    for line in line_iter.map(|x| x.unwrap()) {
        let mut h_set: HashSet<char> = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            if i < line.len() / 2 {
                h_set.extend([c]);
            } else if h_set.contains(&c) {
                result += get_priority(c);
                break;
            }
        }
    }
    println!("result part1: {}", result);
}

fn part2(line_iter: Lines<BufReader<File>>) {
    let mut group_pos = 0;
    let mut result = 0;
    let mut h_sets: Vec<HashSet<char>> = vec![HashSet::new(), HashSet::new(), HashSet::new()];
    for line in line_iter.map(|l| l.unwrap()) {
        for c in line.chars() {
            h_sets[group_pos].extend([c]);
        }
        if group_pos == 2 {
            let temp = h_sets[0].intersection(&h_sets[1]);
            for c in temp.into_iter() {
                if h_sets[2].contains(c) {
                    result += get_priority(c.to_owned());
                    break;
                }
            }
            h_sets = vec![HashSet::new(), HashSet::new(), HashSet::new()];
        }
        group_pos = (group_pos + 1) % 3;
    }
    println!("result part2: {}", result);
}

fn get_priority(c: char) -> i32 {
    let mut d = c as i32;
    if d > 96 {
        d -= 96;
    } else {
        d -= 38;
    }
    d
}
