use std::{io::BufRead, time::Instant};

fn input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    let time = Instant::now();
    let input = input();
    part1(&input);
    // part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &[String]) {
    let result: i32 = input
        .iter()
        .map(|line| {
            let bytes = line.as_bytes();
            let first_digit = bytes.iter().find(|&&b| b.is_ascii_digit()).unwrap();
            let last_digit = bytes.iter().rev().find(|&&b| b.is_ascii_digit()).unwrap();

            ((first_digit - b'0') * 10 + (last_digit - b'0')) as i32
        })
        .sum();

    println!("Part 1 answer: {:?}", result);
}

// fn part2(input: &Vec<usize>) {
//     println!("Part 2 answer: {:?}", result);
// }
