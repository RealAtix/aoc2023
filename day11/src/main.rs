use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    time::Instant,
};

fn input() -> HashMap<(usize, usize), char> {
    let mut expand_rows = HashSet::new();
    let mut expand_cols = HashSet::new();
    // let mut universe = HashMap::new();

    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    // Track universe expansion
    for (y, row) in input.iter().enumerate() {
        if !row.contains('#') {
            expand_rows.insert(y);
        }
        for (x, ch) in row.chars().enumerate() {
            // universe.insert((y, x), ch);
            if ch == '.' && !input.iter().any(|row| row.chars().nth(x) == Some('#')) {
                expand_cols.insert(x);
            }
        }
    }

    // Return expanded universe
    let mut universe = HashMap::new();
    let col_count = input[0].len() + expand_cols.len(); // Assuming all rows are the same length
    for (y, row) in input.iter().enumerate() {
        let expanded_y = y + expand_rows.iter().filter(|&&ry| ry < y).count();
        for (x, ch) in row.chars().enumerate() {
            let expanded_x = x + expand_cols.iter().filter(|&&cx| cx < x).count();
            universe.insert((expanded_y, expanded_x), ch);
            if expand_cols.contains(&x) {
                universe.insert((expanded_y, expanded_x + 1), ch);
            }
        }
        if expand_rows.contains(&y) {
            for x in 0..col_count {
                universe.insert((expanded_y + 1, x), '.');
            }
        }
    }

    universe
}

fn main() {
    let time = Instant::now();
    let universe = input();
    part1(&universe);
    // part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(universe: &HashMap<(usize, usize), char>) {
    // Get pairs
    let galaxies: Vec<(usize, usize)> = universe
        .iter()
        .filter(|(_, &kind)| kind == '#')
        .map(|(&coord, _)| coord)
        .collect();
    let pairs: Vec<((usize, usize), (usize, usize))> = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, loc1)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|loc2| (loc1.clone(), loc2.clone()))
        })
        .collect();

    // Sum the shortest path
    // https://en.wikipedia.org/wiki/Taxicab_geometry
    let result: isize = pairs
        .iter()
        .map(|(loc1, loc2)| {
            (loc1.1 as isize - loc2.1 as isize).abs() + (loc1.0 as isize - loc2.0 as isize).abs()
        })
        .sum();

    println!("Part 1 answer: {:?}", result);
}

// fn part2(input: &[String]) {
//     println!("Part 2 answer: {:?}", result);
// }
