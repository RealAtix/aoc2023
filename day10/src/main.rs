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

/// Calculate the length at each step for the first loop completion
fn calculate_lengths(
    lines: &[String],
    start: (usize, usize),
    map: &HashMap<(Direction, char), Direction>,
) -> HashMap<(usize, usize), usize> {
    let base = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let mut length_store = HashMap::new();

    'base: for dir in base {
        let mut point = start;
        let mut len = 0;
        let mut dirx = dir;

        loop {
            len += 1;
            point = next_point(point, dirx);
            let y = point.0;
            let x = point.1;

            if let Some(line) = lines.get(y) {
                if let Some(&new_direction) = map.get(&(dirx, line.chars().nth(x).unwrap())) {
                    dirx = new_direction;
                    length_store.insert(point, len);
                }
                if line.chars().nth(x).unwrap_or('\0') == 'S' {
                    length_store.insert(point, len);
                    break 'base;
                }
            } else {
                break;
            }
        }
    }

    length_store
}

fn count_points_inside(
    lines: &[String],
    lengths: &HashMap<(usize, usize), usize>,
    start: (usize, usize),
) -> usize {
    let mut total_count = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut scan = false;
        let mut count = 0;

        for (col, _) in line.chars().enumerate() {
            // If scan is true and current tile is not part of the loop, this must mean this tile
            // is contained within the loop grid
            if scan && lengths.get(&(row, col)).is_none() {
                count += 1;
            }

            // Check if the current point and the point below it are on the loop path
            if let (Some(&a), Some(&b)) = (lengths.get(&(row, col)), lengths.get(&(row + 1, col))) {
                // Calculate relative position on the loop using modulo arithmetic to handle the
                // cyclic nature
                let loop_crossing_check: isize = (((a as isize - b as isize)
                    % lengths[&start] as isize)
                    + lengths[&start] as isize)
                    % lengths[&start] as isize;
                // If loop_crossing_check is 1 or loop length - 1, the loop boundary has been
                // crossed
                if [1, lengths[&start] - 1].contains(&(loop_crossing_check as usize)) {
                    scan = !scan;
                }
            }
        }

        total_count += count;
    }

    total_count
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
        // We check all four directions (DFS), we know that only starting two directions will complete
        // the same loop
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
    part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(result: usize) {
    println!("Part 1 answer: {:?}", result);
}

fn part2(input: &[String]) {
    // Find the starting position in the grid
    let start = input
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.chars().position(|c| c == 'S').map(|j| (i, j)));
    let map = build_direction_map();

    let result = if let Some(start) = start {
        let lengths = calculate_lengths(input, start, &map);
        count_points_inside(input, &lengths, start)
    } else {
        panic!("Start not found")
    };

    println!("Part 2 answer: {:?}", result);
}
