use std::fs;

fn main() {
    part1_and_2(4, 1);
    part1_and_2(14, 2);
}

fn part1_and_2(window_size: usize, part: u8) {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let char_iter = &mut buffer.chars();
    let mut window: Vec<char> = vec![];
    char_iter.take(window_size).for_each(|c| window.push(c));

    let mut result = window_size;
    while !is_unique(&mut window) {
        window.remove(0);
        window.push(char_iter.next().unwrap());
        result += 1;
    }
    println!("part{} result: {}", part, result);
}

fn is_unique(window: &mut Vec<char>) -> bool {
    for i in 0..window.len() - 1 {
        for j in i + 1..window.len() {
            if window.get(i).unwrap() == window.get(j).unwrap() {
                return false;
            }
        }
    }
    true
}
