use std::{collections::HashMap, io::BufRead, time::Instant};

type Maps = HashMap<String, (String, String)>;

fn input() -> (Vec<char>, Maps) {
    let mut map = HashMap::new();
    let mut it = std::io::stdin().lock().lines().map(|line| line.unwrap());
    let directions = it.next().unwrap().chars().collect();
    it.next();
    for line in it {
        let (node, elements) = line.split_once('=').unwrap();
        let node = node.trim();
        let (left, right) = elements.split_once(", ").unwrap();
        map.insert(
            node.to_owned(),
            (left[2..].to_owned(), right[..right.len() - 1].to_owned()),
        );
    }

    (directions, map)
}

fn main() {
    let time = Instant::now();
    let (instructions, maps) = input();
    part1(&instructions, &maps);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(instructions: &Vec<char>, maps: &Maps) {
    let mut node = "AAA";
    let mut steps = 0;
    while let Some((left, right)) = maps.get(node) {
        let instruction = instructions[steps % instructions.len()];
        steps += 1;
        match instruction {
            'L' => node = left,
            'R' => node = right,
            _ => unreachable!(),
        };
        if node == "ZZZ" {
            break;
        }
    }
    println!("Part 1 answer: {:?}", steps);
}
