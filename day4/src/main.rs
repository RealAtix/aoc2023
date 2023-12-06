use std::{io::BufRead, time::Instant};

use itertools::Itertools;
#[derive(Debug)]
struct Card {
    winning: Vec<i32>,
    actual: Vec<i32>,
}

fn input() -> Vec<Card> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (_, parts) = line.split_once(':').unwrap();
            let (winning, actual) = parts
                .trim()
                .split('|')
                .map(|batch| {
                    batch
                        .split_whitespace()
                        .filter_map(|n| n.parse().ok())
                        .collect_vec()
                })
                .collect_tuple::<(_, _)>()
                .unwrap();

            Card { winning, actual }
        })
        .collect()
}

fn get_match_count(card: &Card) -> usize {
    card.actual
        .iter()
        .filter(|v| card.winning.contains(v))
        .count()
}

fn main() {
    let time = Instant::now();
    let input = input();
    part1(&input);
    part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &[Card]) {
    let result: i32 = input
        .iter()
        .map(|card| {
            let exp = get_match_count(card) - 1;
            i32::pow(2, exp as u32)
        })
        .sum();
    println!("Part 1 answer: {:?}", result);
}

fn part2(input: &[Card]) {
    let mut card_count: Vec<usize> = vec![1; input.len()];
    input.iter().enumerate().for_each(|(i, card)| {
        let match_count = get_match_count(card);
        let current_copies = card_count[i];
        for _ in 0..current_copies {
            for j in 0..match_count {
                card_count[i + j + 1] += 1;
            }
        }
    });
    let result: usize = card_count.iter().sum();
    println!("Part 2 answer: {:?}", result);
}
