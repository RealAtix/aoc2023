use std::{io::BufRead, time::Instant};

fn input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn solve(input: Vec<Vec<i64>>) -> i64 {
    input[0]
        .iter()
        .zip(input[1].iter())
        .map(|(&time, &record)| {
            let mut left = 1;
            let mut right = time - 1;
            while left <= right {
                let mut found = false;
                if (time - left) * left <= record {
                    left += 1;
                    found = true;
                }
                if (time - right) * right <= record {
                    right -= 1;
                    found = true;
                }
                if !found {
                    break;
                }
            }
            right - left + 1
        })
        .product()
}

fn main() {
    let time = Instant::now();
    let input = input();
    part1(&input);
    part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &[String]) {
    let input: Vec<Vec<i64>> = input
        .iter()
        .map(|line| -> Vec<i64> {
            let (_, numbers) = line.split_once(':').unwrap();
            numbers
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    println!("Part 1 answer: {:?}", solve(input));
}

fn part2(input: &[String]) {
    let input: Vec<Vec<i64>> = input
        .iter()
        .map(|line| {
            line.split_once(':')
                .and_then(|(_, numbers)| {
                    numbers
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse()
                        .ok()
                })
                .map(|num| vec![num])
                .unwrap_or_else(Vec::new)
        })
        .collect();
    println!("Part 2 answer: {:?}", solve(input));
}
