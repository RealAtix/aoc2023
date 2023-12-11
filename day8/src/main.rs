use std::{
    cmp::{max, min},
    collections::HashMap,
    io::BufRead,
    time::Instant,
};

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
    part2(&instructions, &maps);
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

// From Rosetta Code
fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm_steps(steps: &[usize]) -> usize {
    steps.iter().copied().reduce(|acc, s| lcm(s, acc)).unwrap()
}

fn part2(instructions: &Vec<char>, maps: &Maps) {
    let nodes: Vec<String> = maps.keys().filter(|&k| k.ends_with('A')).cloned().collect();
    let mut steps: Vec<usize> = vec![0; nodes.len()];
    for (i, mut node) in nodes.iter().enumerate() {
        while let Some((left, right)) = maps.get(node) {
            let instruction = instructions[steps[i] % instructions.len()];
            steps[i] += 1;
            match instruction {
                'L' => node = left,
                'R' => node = right,
                _ => unreachable!(),
            };
            if node.ends_with('Z') {
                break;
            }
        }
    }
    let result = lcm_steps(&steps);
    println!("Part 2 answer: {:?}", result);
}
