use std::{io::BufRead, time::Instant};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err("Invalid card value"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: i32,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<[Card; 5]> for HandType {
    type Error = &'static str;

    fn try_from(cards: [Card; 5]) -> Result<Self, Self::Error> {
        let mut counts: [usize; 15] = [0; 15];
        for card in cards {
            counts[card as usize] += 1;
        }
        let (uniques, max) = counts.iter().fold((0, 0), |(uniques, max), value| {
            (uniques + usize::from(value > &0), max.max(*value))
        });
        let hand_type = match max {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if uniques == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if uniques == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        };
        Ok(hand_type)
    }
}

fn input() -> Vec<Hand> {
    std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let (cards, bid) = line.split_once(' ')?;
            let bid: i32 = bid.parse().ok()?;
            let cards: [Card; 5] = cards
                .chars()
                .filter_map(|c| Card::try_from(c).ok())
                .collect::<Vec<Card>>()
                .try_into()
                .ok()?;
            let hand_type: HandType = cards.try_into().ok()?;

            Some(Hand {
                cards,
                bid,
                hand_type,
            })
        })
        .collect()
}

fn main() {
    let time = Instant::now();
    let mut input = input();
    input.sort_unstable();
    part1(&input);
    // part2(&input);
    println!("Time elapsed is {:?}", time.elapsed())
}

fn part1(input: &[Hand]) {
    let result: i32 = input
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as i32 * hand.bid)
        .sum();
    println!("Part 1 answer: {:?}", result);
}

// fn part2(input: &[String]) {
//     // println!("Part 2 answer: {:?}", solve(input));
// }
