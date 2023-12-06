use std::{collections::HashMap, io::BufRead, time::Instant};

type Key = (usize, usize);
type Map = HashMap<Key, EnginePart>;

#[derive(Debug)]
enum EnginePart {
    Number(i32),
    Symbol,
    Gear,
}

fn insert_number(key: Key, number: i32, map: &mut Map) {
    for x in 0..number.checked_ilog10().unwrap_or(0) + 1 {
        map.insert((key.0, key.1 - x as usize), EnginePart::Number(number));
    }
}

fn look_around(key: &Key, map: &Map) -> Option<Vec<i32>> {
    let (row, col) = *key;
    let mut result = vec![];

    // Search left and right first
    for x in [0, 2] {
        if let Some(EnginePart::Number(num)) = map.get(&(row, col + x - 1)) {
            result.push(*num);
        }
    }

    // Avoid duplications by then searching top, top-left, top-right, bottom, bottom-left and bottom-right
    'row: for y in [0, 2] {
        for x in [1, 0, 2] {
            if let Some(EnginePart::Number(num)) = map.get(&(row + y - 1, col + x - 1)) {
                result.push(*num);
                if x == 1 {
                    // break, due to corners being the same number (if any)
                    continue 'row;
                }
            }
        }
    }

    if result.is_empty() {
        return None;
    }
    Some(result)
}

fn update_number(number: &mut i32, digit: char) {
    *number *= 10;
    *number += digit.to_digit(10).unwrap() as i32;
}

fn parse_engine_schematic(data: Vec<String>) -> (i32, i32) {
    let mut map = Map::new();

    // Insert symbols, if digit we collect and insert full number after a break (or we reach bounds)
    for (y, row) in data.iter().enumerate() {
        // let mut num = String::new();
        let mut number = 0;
        let x_bounds = row.len() - 1;
        for (x, val) in row.char_indices() {
            match val {
                '.' => {
                    if number != 0 {
                        // end of number
                        insert_number((y, x - 1), number, &mut map);
                        number = 0
                    }
                }
                n if n.is_ascii_digit() => {
                    update_number(&mut number, n);
                    if x == x_bounds {
                        // digit is the last char in the row
                        insert_number((y, x), number, &mut map);
                        number = 0;
                    }
                }
                c => {
                    if number != 0 {
                        insert_number((y, x - 1), number, &mut map);
                        number = 0;
                    }

                    // insert symbol
                    let symbol = match c {
                        '*' => EnginePart::Gear,
                        _ => EnginePart::Symbol,
                    };
                    map.insert((y, x), symbol);
                }
            }
        }
    }

    // Search map for symbols, look for adjacent numbers
    let mut sum = 0;
    let mut gears = 0;
    for (coord, engine_part) in map.iter() {
        match engine_part {
            EnginePart::Number(_) => (),
            EnginePart::Gear => {
                if let Some(numbers) = look_around(coord, &map) {
                    sum += numbers.iter().sum::<i32>();
                    if numbers.len() == 2 {
                        gears += numbers[0] * numbers[1]
                    }
                }
            }
            EnginePart::Symbol => {
                if let Some(numbers) = look_around(coord, &map) {
                    sum += numbers.iter().sum::<i32>();
                }
            }
        }
    }
    (sum, gears)
}

fn input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

fn main() {
    let time = Instant::now();
    let input = input();
    let (part_total, gear_ratios) = parse_engine_schematic(input);
    part1(&part_total);
    part2(&gear_ratios);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(result: &i32) {
    println!("Part 1 answer: {:?}", result);
}

fn part2(result: &i32) {
    println!("Part 2 answer: {:?}", result);
}
