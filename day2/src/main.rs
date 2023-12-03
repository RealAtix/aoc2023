use std::{collections::HashMap, io::BufRead, time::Instant};

use nom::bytes::complete::tag;
use nom::{
    branch::alt,
    character::complete::{digit1, space1},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Cube {
    count: i32,
    color: Color,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    rounds: Vec<Vec<Cube>>,
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_game_id(input: &str) -> IResult<&str, i32> {
    delimited(tag("Game "), parse_number, tag(": "))(input)
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    map(
        separated_pair(
            parse_number,
            space1,
            alt((
                value(Color::Red, tag("red")),
                value(Color::Green, tag("green")),
                value(Color::Blue, tag("blue")),
            )),
        ),
        |(count, color)| Cube { count, color },
    )(input)
}

fn parse_round(input: &str) -> IResult<&str, Vec<Cube>> {
    separated_list1(tag(", "), parse_cube)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = parse_game_id(input).unwrap();
    let (input, rounds) = separated_list1(tag("; "), parse_round)(input).unwrap();
    Ok((input, Game { id, rounds }))
}

fn is_game_possible(game: &Game, available_cubes: &HashMap<Color, i32>) -> bool {
    game.rounds.iter().all(|round| {
        round.iter().all(|cube| {
            available_cubes
                .get(&cube.color)
                .map_or(false, |&max_count| cube.count <= max_count)
        })
    })
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
    part1(&input);
    part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &[String]) {
    let available_cubes = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let result: i32 = input
        .iter()
        .filter_map(|line| match parse_game(line) {
            Ok((_, game)) => Some(game),
            Err(_) => None,
        })
        .filter(|game| is_game_possible(game, &available_cubes))
        .map(|game| game.id)
        .sum();

    println!("Part 1 answer: {:?}", result);
}

fn part2(input: &[String]) {
    let result: i32 = input
        .iter()
        .filter_map(|line| match parse_game(line) {
            Ok((_, game)) => Some(game),
            Err(_) => None,
        })
        .map(|game| {
            let mut max_cubes = HashMap::new();

            for round in game.rounds {
                for cube in round {
                    max_cubes
                        .entry(cube.color)
                        .and_modify(|e| *e = i32::max(*e, cube.count))
                        .or_insert(cube.count);
                }
            }

            max_cubes.values().product::<i32>()
        })
        .sum();

    println!("Part 2 answer: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_id_parsing() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let (input, id) = parse_game_id(input).unwrap();
        assert_eq!("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", input);
        assert_eq!(1, id);
    }

    #[test]
    fn cube_parsing() {
        let input = "3 blue";

        let (input, cube) = parse_cube(input).unwrap();
        assert_eq!("", input);
        assert_eq!(
            Cube {
                count: 3,
                color: Color::Blue
            },
            cube
        );
    }

    #[test]
    fn round_parsing() {
        let input = "3 blue, 4 red";

        let (input, round) = parse_round(input).unwrap();
        assert_eq!("", input);
        assert_eq!(
            vec![
                Cube {
                    count: 3,
                    color: Color::Blue
                },
                Cube {
                    count: 4,
                    color: Color::Red
                }
            ],
            round
        );
    }

    #[test]
    fn game_parsing() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let (input, game) = parse_game(input).unwrap();
        assert_eq!("", input);
        assert_eq!(
            Game {
                id: 1,
                rounds: vec![
                    vec![
                        Cube {
                            count: 3,
                            color: Color::Blue
                        },
                        Cube {
                            count: 4,
                            color: Color::Red
                        }
                    ],
                    vec![
                        Cube {
                            count: 1,
                            color: Color::Red
                        },
                        Cube {
                            count: 2,
                            color: Color::Green
                        },
                        Cube {
                            count: 6,
                            color: Color::Blue
                        },
                    ],
                    vec![Cube {
                        count: 2,
                        color: Color::Green
                    }]
                ]
            },
            game
        )
    }
}
