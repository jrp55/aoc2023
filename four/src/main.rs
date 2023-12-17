use std::collections::HashSet;
use std::str::FromStr;
use std::fs::read_to_string;

#[derive(Debug)]
struct Card {
    id: u64,
    winning_nums: HashSet<u64>,
    chosen_nums: Vec<u64>,
}

#[derive(Debug)]
struct CardParseError;

impl FromStr for Card {
    type Err = CardParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((id, nums_spec)) = s.split_once(": ") {
            if let Some((winning_nums, chosen_nums)) = nums_spec.split_once(" | ") {
                let winning_nums: HashSet<u64> = winning_nums.split_ascii_whitespace().map(|s| s.parse().expect("parse winning num")).collect();
                let chosen_nums: Vec<u64> = chosen_nums.split_ascii_whitespace().map(|s| s.parse().expect("parse chosen num")).collect();
                let id: u64 = id.split_ascii_whitespace().nth(1).expect("card_num").parse().expect("parse card num");
                Ok(Self {id, winning_nums, chosen_nums})
            } else {
                Err(CardParseError)
            }
        } else {
            Err(CardParseError)
        }
    }
}

impl Card {
    fn value(&self) -> u64 {
        match self.matches_count() {
            0 => 0,
            n => 2u64.pow((n-1).try_into().unwrap())
        }
    }

    fn matches_count(&self) -> usize {
        self.chosen_nums.iter().filter(|n| self.winning_nums.contains(*n)).count()
    }
}

fn solve_one(cards: &[Card]) -> u64 {
    cards.iter().map(|c| c.value()).sum()
}

fn solve_two(cards: Vec<Card>) -> u64 {
    let mut counts = vec![1; cards.len()];
    for card in cards.iter() {
        let v = card.matches_count();
        if v > 0 {
            let m = *counts.get((card.id-1) as usize).expect("every card should have a count");
            for id in card.id..card.id+(v as u64) {
                *counts.get_mut(id as usize).expect("Cards will never make you copy a card past the end of the table") += m;
            }
        }
    }
    counts.iter().sum()
}

fn parse_cards(input: &str) -> Vec<Card> {
    input.lines().map(|l| l.parse::<Card>().unwrap()).collect()
}

fn main(){
    let cards = parse_cards(&read_to_string("input.txt").expect("reading input.txt"));
    println!("part 1 : {}", solve_one(&cards));
    println!("part 2 : {}", solve_two(cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test() {
        let cards = parse_cards(TEST_DATA);
        assert_eq!(13, solve_one(&cards));
        assert_eq!(30, solve_two(cards));
    }
}