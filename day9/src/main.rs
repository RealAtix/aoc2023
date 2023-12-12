use std::{io::Read, time::Instant};

fn input() -> Vec<Vec<i64>> {
    let mut buf = vec![];
    std::io::stdin().lock().read_to_end(&mut buf).unwrap();
    buf.split(|&b| b == b'\n')
        .map(|line| {
            line.split(|&b| b == b' ')
                .filter(|&b| !b.is_empty())
                .map(|b| atoi::atoi(b).unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|v| !v.is_empty())
        .collect()
}

fn solve(input: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;
    for seq in input {
        let mut current_seq = seq.clone();
        let mut last: Vec<i64> = Vec::new();

        while current_seq.iter().any(|&n| n != 0) {
            last.push(*current_seq.last().unwrap());
            current_seq = current_seq
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<i64>>();
        }
        result += last.iter().rev().sum::<i64>();
    }
    result
}

fn main() {
    let time = Instant::now();
    let mut input = input();
    part1(solve(&input));
    input.iter_mut().for_each(|h| h.reverse());
    part2(solve(&input));

    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(result: i64) {
    println!("Part 1 answer: {:?}", result);
}

fn part2(result: i64) {
    println!("Part 2 answer: {:?}", result);
}
