use std::{io::BufRead, time::Instant};

const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

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
    part2(&input);
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

fn part2(input: &[String]) {
    let result: usize = input
        .iter()
        .map(|line| {
            let bytes = line.as_bytes();
            let first_digit = (0..bytes.len()).find_map(|i| find_num(bytes, i)).unwrap();
            let last_digit = (0..bytes.len())
                .rev()
                .find_map(|i| find_num(bytes, i))
                .unwrap();
            first_digit * 10 + last_digit
        })
        .sum();
    println!("Part 2 answer: {:?}", result);
}

fn find_num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}
