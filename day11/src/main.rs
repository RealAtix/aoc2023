use std::{collections::HashSet, io::BufRead, time::Instant};

type GalaxyPairs = Vec<((usize, usize), (usize, usize))>;

fn input() -> (GalaxyPairs, HashSet<usize>, HashSet<usize>) {
    let input: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    // Track universe expansion and galaxies
    let mut expand_rows = HashSet::new();
    let mut expand_cols = HashSet::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (y, row) in input.iter().enumerate() {
        if !row.contains('#') {
            expand_rows.insert(y);
        }
        for (x, ch) in row.chars().enumerate() {
            if ch == '.' && !input.iter().any(|row| row.chars().nth(x) == Some('#')) {
                expand_cols.insert(x);
            } else if ch == '#' {
                galaxies.push((y, x));
            }
        }
    }

    let pairs: GalaxyPairs = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, loc1)| galaxies.iter().skip(i + 1).map(|loc2| (*loc1, *loc2)))
        .collect();

    (pairs, expand_rows, expand_cols)
}

// fn get_expanded_point(
//     original: usize,
//     expansions: &HashSet<usize>,
//     expansion_factor: usize,
// ) -> usize {
//     original + (expansions.iter().filter(|&&e| e < original).count() * expansion_factor)
// }

fn get_shortest_galaxy_paths(
    galaxy_pairs: &GalaxyPairs,
    expand_rows: &HashSet<usize>,
    expand_cols: &HashSet<usize>,
    expansion_factor: usize,
) -> isize {
    // Factor in the expansion of the universe
    let get_expanded_point =
        |original: usize, expansions: &HashSet<usize>, expansion_factor: usize| -> usize {
            original + (expansions.iter().filter(|&&e| e < original).count() * expansion_factor)
        };

    // Sum the shortest path
    // https://en.wikipedia.org/wiki/Taxicab_geometry
    galaxy_pairs
        .iter()
        .map(|(loc1, loc2)| {
            let loc1_expanded = (
                get_expanded_point(loc1.0, expand_rows, expansion_factor),
                get_expanded_point(loc1.1, expand_cols, expansion_factor),
            );
            let loc2_expanded = (
                get_expanded_point(loc2.0, expand_rows, expansion_factor),
                get_expanded_point(loc2.1, expand_cols, expansion_factor),
            );
            (loc1_expanded.1 as isize - loc2_expanded.1 as isize).abs()
                + (loc1_expanded.0 as isize - loc2_expanded.0 as isize).abs()
        })
        .sum()
}

fn main() {
    let time = Instant::now();
    let (galaxy_pairs, expand_rows, expand_cols) = input();
    part1(&galaxy_pairs, &expand_rows, &expand_cols);
    part2(&galaxy_pairs, &expand_rows, &expand_cols);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(galaxy_pairs: &GalaxyPairs, expand_rows: &HashSet<usize>, expand_cols: &HashSet<usize>) {
    const EXPANSION_FACTOR: usize = 1;
    println!(
        "Part 1 answer: {:?}",
        get_shortest_galaxy_paths(galaxy_pairs, expand_rows, expand_cols, EXPANSION_FACTOR)
    );
}

fn part2(galaxy_pairs: &GalaxyPairs, expand_rows: &HashSet<usize>, expand_cols: &HashSet<usize>) {
    const EXPANSION_FACTOR: usize = 999_999;
    println!(
        "Part 2 answer: {:?}",
        get_shortest_galaxy_paths(galaxy_pairs, expand_rows, expand_cols, EXPANSION_FACTOR)
    );
}
