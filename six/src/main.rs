use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Document {
    times: Vec<u64>,
    distances: Vec<u64>,
}

#[derive(Debug)]
struct WellKernedDocument {
    time: u64,
    distance: u64,
}

#[derive(Debug)]
struct ParseDocumentError;

impl FromStr for Document {
    type Err = ParseDocumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().expect("Document has a first line for times");
        let second_line = lines.next().expect("Document has a second line for times");

        let times = first_line.split_ascii_whitespace().skip(1).map(|s| s.parse().expect("Can parse a time")).collect();
        let distances = second_line.split_ascii_whitespace().skip(1).map(|s| s.parse().expect("Can parse a distance")).collect();
        Ok(Self { times, distances })
    }
}

fn concat_string_vec(strings: Vec<String>) -> String {
    let mut result: String = String::new();
    strings.into_iter().for_each(|s| result.push_str(&s));
    result
}

impl FromStr for WellKernedDocument {
    type Err = ParseDocumentError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().expect("Document has a first line for times");
        let second_line = lines.next().expect("Document has a second line for times");

        let times: Vec<String> = first_line.split_ascii_whitespace().skip(1).map(|s| (*s).to_owned()).collect();
        let time = concat_string_vec(times);
        let distances: Vec<String> = second_line.split_ascii_whitespace().skip(1).map(|s| (*s).to_owned()).collect();
        let distance = concat_string_vec(distances);
        let time: u64 = time.parse().expect("To parse a time");
        let distance: u64 = distance.parse().expect("To parse a distance");
        Ok(Self { time, distance })
    }
}

fn num_combos_that_beat(time: u64, distance: u64) -> u64 {
    let mut count = 0;
    let mid_floor = time.checked_div(2).expect("Valid time division to work");
    let mut hold = mid_floor;
    let mut go_time = time - hold;
    let mut candidate_distance = hold * go_time;
    while candidate_distance > distance && hold > 0 && go_time < time {
        count += 1;
        hold -= 1;
        go_time += 1;
        candidate_distance = hold * go_time;
    }

    hold = mid_floor + 1;
    go_time = time - hold;
    candidate_distance = hold * go_time;
    while candidate_distance > distance && go_time > 0 && hold < time {
        count += 1;
        hold += 1;
        go_time -= 1;
        candidate_distance = hold * go_time;
    }
    
    println!("{}", count);
    count
}

fn solve_one(doc: &Document) -> u64 {
    doc.times.iter().zip(doc.distances.iter()).map(|(t, d)| num_combos_that_beat(*t, *d)).product()
}

fn main() {
    let input = read_to_string("input.txt").expect("Can read input.txt");
    let doc: Document = input.parse().expect("Can parse valid document");
    let wkd: WellKernedDocument = input.parse().expect("Can parse valid wkd");
    println!("part one: {}", solve_one(&doc));
    println!("part two: {}", num_combos_that_beat(wkd.time, wkd.distance));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_one() {
        let doc: Document = INPUT.parse().expect("Can parse valid document");
        assert_eq!(vec![7, 15, 30], doc.times);
        assert_eq!(vec![9, 40, 200], doc.distances);
        assert_eq!(288, solve_one(&doc));
    }

    #[test]
    fn part_two() {
        let doc: WellKernedDocument = INPUT.parse().expect("Can parse valid document");
        assert_eq!(71530, doc.time);
        assert_eq!(940200, doc.distance);
        assert_eq!(71503, num_combos_that_beat(doc.time, doc.distance));
    }
}
