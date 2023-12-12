use std::{collections::HashMap, io::BufRead, time::Instant};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn build_direction_map() -> HashMap<(Direction, char), Direction> {
    use Direction::*;
    [
        ((Up, '|'), Up),
        ((Up, 'F'), Right),
        ((Up, '7'), Left),
        ((Down, '|'), Down),
        ((Down, 'L'), Right),
        ((Down, 'J'), Left),
        ((Left, '-'), Left),
        ((Left, 'L'), Up),
        ((Left, 'F'), Down),
        ((Right, '-'), Right),
        ((Right, 'J'), Up),
        ((Right, '7'), Down),
    ]
    .into_iter()
    .collect()
}

fn next_point(point: (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (point.0 - 1, point.1),
        Direction::Down => (point.0 + 1, point.1),
        Direction::Left => (point.0, point.1 - 1),
        Direction::Right => (point.0, point.1 + 1),
    }
}

fn find_loop_length(
    lines: &[String],
    mut point: (usize, usize),
    mut direction: Direction,
    map: &HashMap<(Direction, char), Direction>,
) -> Option<usize> {
    let mut len = 0;
    loop {
        len += 1;
        point = next_point(point, direction);
        let y = point.0;
        let x = point.1;
        if let Some(line) = lines.get(y) {
            if let Some(&new_direction) = map.get(&(direction, line.chars().nth(x)?)) {
                direction = new_direction;
            } else if line.chars().nth(x)? == 'S' {
                return Some(len);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

fn input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn solve(input: &[String]) -> usize {
    // Find the starting position in the grid
    let start = input
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.chars().position(|c| c == 'S').map(|j| (i, j)));
    let map = build_direction_map();

    if let Some(start) = start {
        // We check all four directions (BFS), we know that only only starting directions will complete
        // the loop
        let base = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        let loop_length = base
            .iter()
            .filter_map(|&dir| find_loop_length(input, start, dir, &map))
            .find(|&len| len != 0)
            .unwrap_or(0);
        loop_length / 2
    } else {
        panic!("Start not found")
    }
}

fn main() {
    let time = Instant::now();
    let input = input();
    part1(solve(&input));
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(result: usize) {
    println!("Part 1 answer: {:?}", result);
}
