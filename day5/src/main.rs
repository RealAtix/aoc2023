use std::{io::BufRead, time::Instant};

use itertools::Itertools;

type Range = (i64, usize);

#[derive(Debug)]
struct Map {
    _src_category: String,
    _dest_category: String,
    src_ranges: Vec<Range>,
    dest_ranges: Vec<Range>,
}

impl Map {
    fn new(src_category: String, dest_category: String) -> Self {
        Map {
            _src_category: src_category,
            _dest_category: dest_category,
            src_ranges: Vec::new(),
            dest_ranges: Vec::new(),
        }
    }

    fn add_range(&mut self, src_start: i64, dest_start: i64, range_length: usize) {
        self.src_ranges.push((src_start, range_length));
        self.dest_ranges.push((dest_start, range_length));
    }

    fn sort_ranges(&mut self) {
        // Sort ranges ascending based on source range start value
        let mut indices: Vec<usize> = (0..self.src_ranges.len()).collect();
        indices.sort_by_key(|&i| self.src_ranges[i].0);

        self.src_ranges = indices.iter().map(|&i| self.src_ranges[i]).collect();
        self.dest_ranges = indices.iter().map(|&i| self.dest_ranges[i]).collect();
    }
}

fn input() -> (Vec<i64>, Vec<Map>) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();

    let mut current_map: Option<Map> = None;
    let map_suffix = " map:";

    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| match line {
            s if s.starts_with("seeds: ") => {
                let (_, seeds_part) = s.split_once(':').unwrap();
                seeds.extend(
                    seeds_part
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap()),
                );
            }
            s if s.ends_with(map_suffix) => {
                let (src_category, _, dest_category) = s[..s.len() - map_suffix.len()]
                    .splitn(3, '-')
                    .collect_tuple()
                    .unwrap();
                current_map = Some(Map::new(src_category.to_owned(), dest_category.to_owned()));
            }
            s if s.is_empty() => {
                if current_map.is_some() {
                    let mut current_map = current_map.take().unwrap();
                    current_map.sort_ranges();
                    maps.push(current_map);
                }
            }
            s => {
                let (dest_start, src_start, range_length) = s
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                current_map.as_mut().unwrap().add_range(
                    src_start,
                    dest_start,
                    range_length as usize,
                )
            }
        });
    let mut current_map = current_map.unwrap();
    current_map.sort_ranges();
    maps.push(current_map);

    (seeds, maps)
}

fn find_lowest_location(seeds: &[i64], maps: &[Map]) -> Option<i64> {
    seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |src, map| {
                for (i, (src_start, range_length)) in map.src_ranges.iter().enumerate() {
                    if (*src_start..*src_start + *range_length as i64).contains(&src) {
                        return map.dest_ranges[i].0 + src - src_start;
                    }
                }
                src
            })
        })
        .min()
}

fn main() {
    let time = Instant::now();
    let (seeds, maps) = input();
    part1(&seeds, &maps);
    part2(&seeds, &maps);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(seeds: &[i64], maps: &[Map]) {
    let result = find_lowest_location(seeds, maps);
    println!("Part 1 answer: {:?}", result.unwrap());
}

fn part2(seeds: &[i64], maps: &[Map]) {
    let seeds = seeds
        .chunks(2)
        .flat_map(|range| (range[0]..range[0] + range[1]).collect_vec())
        .collect_vec();
    let result = find_lowest_location(seeds.as_ref(), maps);
    println!("Part 2 answer: {:?}", result.unwrap());
}
