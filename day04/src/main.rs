use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};
fn main() {
    let file = File::open("input.txt").unwrap();
    let line_iter = BufReader::new(file).lines();
    // part1(line_iter);
    part2(line_iter);
}

fn part1(line_iter: Lines<BufReader<File>>) {
    let mut result = 0;
    for line in line_iter.map(|l| l.unwrap()) {
        let split: Vec<&str> = line.split(',').collect();
        let r: Vec<i32> = split
            .iter()
            .map(|sp| sp.split('-').collect::<Vec<&str>>())
            .flatten()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if r[0] <= r[2] && r[1] >= r[3] || r[0] >= r[2] && r[1] <= r[3] {
            result += 1;
        }
    }
    println!("result part1: {}", result);
}

fn part2(line_iter: Lines<BufReader<File>>) {
    let mut result = 0;
    for line in line_iter.map(|l| l.unwrap()) {
        let split: Vec<&str> = line.split(',').collect();
        let r: Vec<i32> = split
            .iter()
            .map(|sp| sp.split('-').collect::<Vec<&str>>())
            .flatten()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if !(r[0] < r[2] && r[1] < r[2] || r[0] > r[3] && r[1] > r[3]) {
            result += 1;
        }
    }
    println!("result part2: {}", result);
}
