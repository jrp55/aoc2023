use std::str::FromStr;
use itertools::Itertools;
use std::fs::read_to_string;
use rayon::prelude::*;

#[derive(Debug, PartialEq)]
struct Range {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

#[derive(Debug, PartialEq)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u64> = s.split_ascii_whitespace().map(|s| s.parse().expect("parse range integer")).collect();
        if parts.len() != 3 {
            return Err(ParseRangeError);
        }
        Ok(Range { source_start: *parts.get(1).unwrap(), destination_start: *parts.first().unwrap(), length: *parts.get(2).unwrap() })
    }
}

trait Transformer {
    fn transform(&self, input: u64) -> u64;
}

#[derive(Debug)]
struct StageTransformer {
    ranges: Vec<Range>,
}

impl StageTransformer {
    fn new(mut ranges: Vec<Range>) -> Self {
        ranges.sort_by(|a, b| a.source_start.partial_cmp(&b.source_start).unwrap() );
        Self { ranges }
    }
}

impl Transformer for StageTransformer {
    fn transform(&self, input: u64) -> u64 {
        let pp = self.ranges.partition_point(|r| r.source_start <= input);
        match pp.checked_sub(1) {
            None => input,
            Some(idx) => {
                let candidate_range = self.ranges.get(idx).expect("Got a valid index from lookup");
                let diff = input - candidate_range.source_start;
                if diff <= candidate_range.length {
                    diff + candidate_range.destination_start
                } else {
                    input
                }
            }
        }
    }
}

#[derive(Debug)]
struct AlmanacTransformer {
    stage_transformers: Vec<StageTransformer>,
}

impl Transformer for AlmanacTransformer {
    fn transform(&self, input: u64) -> u64 {
        let mut result = input;
        for stage_transformer in &self.stage_transformers {
            result = stage_transformer.transform(result);
        }
        result
    }
}

#[derive(Debug)]
struct Almanac {
    transformer: AlmanacTransformer,
    seeds: Vec<u64>,
}

#[derive(Debug)]
struct ParseAlmanacError;

impl FromStr for Almanac {
    type Err = ParseAlmanacError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input_lines = s.lines();
        let seeds_input = input_lines.next().expect("Should have a seeds line");
        let mut seed_iter = seeds_input.split_ascii_whitespace();
        seed_iter.next();
        let seeds: Vec<u64> = seed_iter.map(|s| s.parse().expect("Parsing seed int")).collect();
        let mut stage_transformers: Vec<StageTransformer> = Vec::with_capacity(7);
        while let Some("") = input_lines.next() {
            // skip title
            input_lines.next();
            stage_transformers.push(StageTransformer::new(input_lines.take_while_ref(|l| !l.is_empty()).map(|s| s.parse().expect("Parsing a range")).collect()));
        }
        Ok(Almanac { transformer: AlmanacTransformer { stage_transformers }, seeds })
    }
}

impl Transformer for Almanac {
    fn transform(&self, input: u64) -> u64 {
        self.transformer.transform(input)
    }
}

fn solve_one(almanac: &Almanac) -> u64 {
    almanac.seeds.iter().map(|seed| almanac.transform(*seed)).min().expect("Expected an answer to part one")
}

fn solve_two_int<T>(chunks: &[&[u64]], transformer: &T) -> u64
    where T: Transformer + Sync
{
    chunks.par_iter().map(|x| -> u64 {
        let start = x.first().expect("A chunk to have a start");
        let len = x.get(1).expect("A chunk to have a length");
        ((*start)..start+len).map(|n| transformer.transform(n)).min().expect("A chunk to have a minimum location")
    }).min().expect("An answer to part two")
}

fn solve_two(almanac: &Almanac) -> u64 {
    let chunks: Vec<&[u64]> = almanac.seeds.chunks(2).collect();
    solve_two_int(chunks.as_slice(), almanac)
}

fn main() {
    let almanac: Almanac = read_to_string("input.txt").expect("Read input.txt").parse().expect("Input could be parsed into Almanac");
    println!("part one: {}", solve_one(&almanac));
    println!("part two: {}", solve_two(&almanac));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_transformer() {
        let st = StageTransformer {
            ranges: vec![
                Range {
                    source_start: 0,
                    destination_start: 42,
                    length: 7,
                },
                Range {
                    source_start: 7,
                    destination_start: 57,
                    length: 4,
                },
                Range {
                    source_start: 11,
                    destination_start: 0,
                    length: 42,
                },
                Range {
                    source_start: 53,
                    destination_start: 49,
                    length: 8,
                },
            ],
        };
        assert_eq!(49, st.transform(53));
    }

    #[test]
    fn almanac() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let alm = Almanac::from_str(input).expect("Yeah");
        assert_eq!(82, alm.transform(79));
        assert_eq!(43, alm.transform(14));
        assert_eq!(86, alm.transform(55));
        assert_eq!(35, alm.transform(13));
        assert_eq!(35, solve_one(&alm));
        assert_eq!(46, solve_two(&alm));
    }

}
