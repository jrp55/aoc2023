use std::str::FromStr;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Drawing {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDrawingError;

impl FromStr for Drawing {
    type Err = ParseDrawingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;

        for elem in s.split(", ") {
             match elem.split_once(' ') {
                Some((num, col)) => {
                    let n: u64 = num.parse().unwrap();
                    match col {
                        "red" => { red = n; },
                        "green" => { green = n; },
                        "blue" => { blue = n; },
                        _ => panic!("Unexpected colour {}", col),
                    }
                },
                None => { eprintln!("{}", elem) }
             }
        }

        Ok(Self { red, green, blue})
    }
}

fn part_one_criterion(drawing: &Drawing) -> bool {
    drawing.red <= 12 && drawing.green <= 13 && drawing.blue <= 14
}

impl Drawing {
    fn is_possible(&self, criterion: &dyn Fn(&Drawing)->bool) -> bool {
        criterion(self)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u64,
    drawings: Vec<Drawing>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((game_id, drawings)) = s.split_once(": ") {
            if let Some((_, id)) = game_id.split_once(' ') {
                let id: u64 = id.parse().expect("Parsing game ID");
                let drawings: Vec<Drawing> = drawings.split("; ").map(|s| Drawing::from_str(s).expect("Parsing drawing")).collect();
                Ok(Game{id, drawings})
            } else {
                Err(ParseGameError)
            }
        }
        else {
            Err(ParseGameError)
        }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.drawings.iter().all(|d| d.is_possible(&part_one_criterion))
    }

    fn power(&self) -> u64 {
        let mut max_red: u64 = 0;
        let mut max_green: u64 = 0;
        let mut max_blue: u64 = 0;

        for drawing in self.drawings.iter() {
            if drawing.red > max_red { max_red = drawing.red; }
            if drawing.green > max_green { max_green = drawing.green; }
            if drawing.blue > max_blue { max_blue = drawing.blue; }
        }

        max_red * max_green * max_blue
    }
}

fn solve_one(games: &[Game]) -> u64 {
    games.iter().filter(|g| (*g).is_possible()).map(|g|g.id).sum()
}

fn solve_two(games: &[Game]) -> u64 {
    games.iter().map(|g| g.power()).sum()
}

fn parse_games<T: AsRef<str>>(input: T) -> Vec<Game> {
    input.as_ref().lines().map(|s| Game::from_str(s).expect("Parse error for game")).collect()
}

fn main() {
    let input = read_to_string("input.txt").expect("Read input.txt");
    let games = parse_games(input);
    println!("{}", solve_one(&games));
    println!("{}", solve_two(&games));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_drawing() {
        const TEST_INPUT: &str = r"3 blue, 4 red";
        let drawing = Drawing::from_str(TEST_INPUT);
        assert_eq!(Ok(Drawing{red: 4, blue: 3, green: 0}), drawing)
    }

    #[test]
    fn parse_drawings() {
        const TEST_INPUT: &str = r"1 red, 2 green, 6 blue; 2 green";
        let drawings: Vec<Drawing> = TEST_INPUT.split("; ").map(|s| Drawing::from_str(s).expect("parsing drawing")).collect();
        assert_eq!(vec![Drawing{red: 1, green: 2, blue: 6 }, Drawing{red: 0, green: 2, blue: 0}], drawings);
    }

    #[test]
    fn test_parse_games() {
        const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(vec![
            Game{ id: 1, drawings: vec![Drawing{ blue:3, red: 4, green:0}, Drawing{red:1, green:2, blue: 6}, Drawing{green:2, red:0, blue: 0}]},
            Game{ id: 2, drawings: vec![Drawing{ blue:1, green:2, red:0 }, Drawing{green:3, blue:4, red:1}, Drawing{green: 1, blue:1, red:0}]},
        ], parse_games(TEST_INPUT));
    }

    #[test]
    fn part_one() {
        const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse_games(TEST_INPUT);
        assert_eq!(8, solve_one(&games))
    }

    #[test]
    fn part_two() {
        const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse_games(TEST_INPUT);
        assert_eq!(2286, solve_two(&games))
    }

}
