use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait Grid2D {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn valid_coordinate(&self, p: &Point) -> bool;
}

#[derive(Debug)]
struct AoCGrid<'a> {
    lines: Vec<&'a str>,
    width: usize,
    height: usize,
}

impl<'a> Grid2D for AoCGrid<'a> {
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn valid_coordinate(&self, p: &Point) -> bool {
        p.x < self.width && p.y < self.height
    }

}

#[derive(Debug)]
struct AoCGridAdjacentPoints {
    valid_coords: Vec<Point>,
    iter_number: usize
}

impl AoCGridAdjacentPoints {
    fn new<T: Grid2D>(grid: &T, p: &Point) -> Self {
        if !grid.valid_coordinate(p) {
            panic!("Cannot provide adjacency for invalid coordinate {}", p);
        }
        const POSSIBLE_ADJACENCY: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)
        ];
        let mut valid_coords: Vec<Point> = Vec::new();
        for (dx, dy) in POSSIBLE_ADJACENCY.iter() {
            if let Some(u) = p.x.checked_add_signed(*dx) {
                if let Some(v) = p.y.checked_add_signed(*dy) {
                    let candidate_point = Point{ x:u, y:v };
                    if grid.valid_coordinate(&candidate_point) {
                        valid_coords.push(candidate_point);
                    }
                }
            }
        }
        Self { valid_coords, iter_number: 0 }
    }
}

impl Iterator for AoCGridAdjacentPoints {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.valid_coords.get(self.iter_number);
        self.iter_number += 1;
        result.copied()
    }
}

#[derive(Debug)]
struct AoCGridAdjacenyIterator<'g> {
    grid: &'g AoCGrid<'g>,
    inner: AoCGridAdjacentPoints,
}

impl<'g> AoCGridAdjacenyIterator<'g> {
    fn new(grid: &'g AoCGrid, p: &Point) -> Self {
        let inner = AoCGridAdjacentPoints::new(grid, p);
        AoCGridAdjacenyIterator { grid, inner }
    }
}

impl<'g> Iterator for AoCGridAdjacenyIterator<'g> {
    type Item = &'g str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.inner.next() {
            self.grid.get(&p)
        }
        else {
            None
        }
    }
}

struct GridIterator {
    width: usize,
    height: usize,
    current: Option<Point>,
}

impl GridIterator {
    fn new<T: Grid2D>(grid: &T) -> Self {
        Self { width: grid.width(), height: grid.height(), current: None }
    }
}

impl Iterator for GridIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = Some(Point{x:0, y:0});
        }
        else if let Some(p) = self.current {
            if p.x == self.width-1 {
                if p.y == self.height-1 {
                    self.current = None;
                }
                else {
                    self.current = Some(Point{ x:0, y:p.y+1 })
                }
            }
            else {
                self.current = Some(Point{ x: p.x+1, y:p.y })
            }
            
        }
        self.current
    }
}

impl<'a> AoCGrid<'a> {
    fn new(input: &'a str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let mut peeky = lines.iter().peekable();
        let width: usize = peeky.peek().expect("Input should have at least one line").len();
        for line in peeky {
            if (**line).len() != width {
                panic!("Not all lines are the same length");
            }
        }
        let height: usize = lines.len();
        Self { lines, width, height }
    }

    fn get(&self, p: &Point) -> Option<&str> {
        if self.valid_coordinate(p) {
            self.lines.get(p.y).and_then(|l| l.get(p.x..p.x+1))
        }
        else {
            None
        }
    }

    fn get_str(&self, p: &Point, length: usize) -> Option<&str> {
        let end_point = Point { x: p.x+length, y: p.y };
        if self.valid_coordinate(p) {
            if self.valid_coordinate(&end_point) {
                self.lines.get(p.y).and_then(|l| l.get(p.x..end_point.x))
            }
            else {
                self.lines.get(p.y).and_then(|l| l.get(p.x..))
            }
        } else {
            None
        }
    }

    fn adjacent(&'a self, p: &Point) -> AoCGridAdjacenyIterator<'a> {
        AoCGridAdjacenyIterator::new(self, p)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct GridNumber {
    value: u64,
    start_coord: Point,
    coord_length: usize,
}

impl GridNumber {
    fn part_number(&self, engine_schematic: &EngineSchematic) -> Option<u64> {
        match GridNumberAdjacentData::new(self, engine_schematic).any(|d| d.parse::<GridDataType>().expect("Expected valid data") == GridDataType::Symbol) {
            true => Some(self.value),
            false => None,
        }
    }
}

#[derive(Debug)]
struct EngineSchematic<'a> {
    grid: &'a AoCGrid<'a>,
}

impl<'a> Grid2D for EngineSchematic<'a> {
    fn width(&self) -> usize { self.grid.width() }
    fn height(&self) -> usize { self.grid.height() }
    fn valid_coordinate(&self, p: &Point) -> bool {
        self.grid.valid_coordinate(p)
    }

}

#[derive(Debug)]
struct GridNumberIterator<'a> {
    engine_schematic: &'a EngineSchematic<'a>,
    point: Option<Point>,
}

#[derive(Debug, PartialEq)]
enum GridDataType {
    Digit(u64),
    Symbol,
    Space,
}

#[derive(Debug)]
struct ParseGridDataTypeError;

impl FromStr for GridDataType {
    type Err = ParseGridDataTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 { return Err(ParseGridDataTypeError); }

        match s {
            "." => Ok(Self::Space),
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Ok(Self::Digit(s.parse().unwrap())),
            _ => Ok(Self::Symbol),
        }
    }
}

impl<'a> GridNumberIterator<'a> {
    fn new(engine_schematic: &'a EngineSchematic<'a>) -> Self {
        Self { engine_schematic, point: Some(Point{x:0,y:0}) }
    }

    fn next_point(&self) -> Option<Point> {
        match &self.point {
            Some(p) => {
                match ((p.x == (self.engine_schematic.width() - 1)), (p.y == (self.engine_schematic.height() - 1))) {
                    (true, true) => None,
                    (true, false) => Some(Point{x:0, y:p.y+1}),
                    (false, _) => Some(Point{x:p.x+1, y:p.y})
                }
            },
            None => None,
        }
    }
}

fn get_coord_length(start: &Point, end: &Point, grid_width: usize) -> usize {
    if end.x < start.x && end.y > start.y {
        // Crossed line
        grid_width - start.x
    } else {
        end.x - start.x
    }
}

impl<'a> Iterator for GridNumberIterator<'a> {
    type Item = GridNumber;
    fn next(&mut self) -> Option<Self::Item> {
        let mut start_of_next_number: Option<Point> = None;
        let mut result: Option<Self::Item> = None;
        while self.point.is_some() {
            let this_point = self.point.unwrap();
            match self.engine_schematic.grid.get(&this_point).unwrap().parse::<GridDataType>().expect("Parse correct griddatatype") {
                GridDataType::Digit(_) => {
                    if start_of_next_number.is_none() {
                        start_of_next_number = Some(this_point);
                    }
                },
                _ => {
                    if let Some(start_coord) = start_of_next_number {
                        let coord_length = get_coord_length(&start_coord, &this_point, self.engine_schematic.width());
                        let value = self.engine_schematic.grid.get_str(&start_coord, coord_length).expect("get_str").parse().expect("parse u64 from digits");
                        result = Some(GridNumber { value, start_coord, coord_length });
                        start_of_next_number = None;
                    }
                },
            }
            let next_point = self.next_point();
            if let (Some(start_coord), Some(p), Some(np)) = (start_of_next_number, self.point, next_point) {
                if np.y > p.y {
                    let coord_length = self.engine_schematic.width() - start_coord.x;
                    let value = self.engine_schematic.grid.get_str(&start_coord, coord_length).expect("get_str").parse().expect("parse u64 from digits");
                    result = Some(GridNumber { value, start_coord, coord_length });
                    start_of_next_number = None;
                }
            }
            self.point = next_point;
            if result.is_some() {
                return result;
            }
        }
        if let Some(start_coord) = start_of_next_number {
            let coord_length = self.engine_schematic.width() - start_coord.x;
            let value = self.engine_schematic.grid.get_str(&start_coord, coord_length).expect("get_str").parse().expect("parse u64 from digits");
            result = Some(GridNumber { value, start_coord, coord_length });
        }
        result
    }
}

struct GridNumberAdjacentData<'a> {
    adjacent_points: Vec<&'a str>,
    iter_number: usize,
}

impl<'a> GridNumberAdjacentData<'a> {
    fn new(grid_number: &'a GridNumber, engine_schematic: &'a EngineSchematic<'a>) -> Self {
        let mut adjacent_points: Vec<&'a str> = Vec::with_capacity(2*grid_number.coord_length + 6);
        for i in 0..grid_number.coord_length {
            let this_point = Point { x: grid_number.start_coord.x + i, y: grid_number.start_coord.y };
            for adj in engine_schematic.grid.adjacent(&this_point) {
                adjacent_points.push(adj);
            }
        }
        Self { adjacent_points, iter_number: 0 }
    }
}

impl<'a> Iterator for GridNumberAdjacentData<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_number >= self.adjacent_points.len() {
            None
        } else {
            self.iter_number += 1;
            self.adjacent_points.get(self.iter_number-1).copied()
        }
    }
}

struct GearIterator {
    gears: Vec<Gear>,
    iter_number: usize,
}

impl GearIterator {
    fn new(engine_schematic: &EngineSchematic) -> Self {
        let mut gears = Vec::new();
        let lookup = part_number_lookup(engine_schematic);
        for point in GridIterator::new(engine_schematic) {
            if "*" == engine_schematic.grid.get(&point).expect("valid data for valid coordinate") {
                let mut adjacent_part_numbers = HashSet::new();
                for adj in AoCGridAdjacentPoints::new(engine_schematic, &point) {
                    if let Some(grid_number) = lookup.get(&adj) {
                       adjacent_part_numbers.insert(grid_number);
                    }
                }

                if adjacent_part_numbers.len() == 2 {
                    gears.push(Gear { ratio: adjacent_part_numbers.into_iter().map(|g|g.value).product() })
                }
            }
        }
        Self { gears, iter_number: 0 }
    }
}

impl Iterator for GearIterator {
    type Item = Gear;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.gears.get(self.iter_number);
        self.iter_number += 1;
        result.copied()
    }
}

#[derive(Clone, Copy)]
struct Gear {
    ratio: u64,
}

impl<'a> EngineSchematic<'a> {
    fn new(grid: &'a AoCGrid) -> Self {
        Self { grid }
    }

    fn grid_numbers(&self) -> GridNumberIterator {
        GridNumberIterator::new(self)
    }

    fn gears(&self) -> GearIterator {
        GearIterator::new(self)
    }
}

fn part_number_lookup(engine_schematic: &EngineSchematic) -> HashMap<Point, GridNumber> {
    let mut result = HashMap::new();
    for grid_number in engine_schematic.grid_numbers() {
        if grid_number.part_number(engine_schematic).is_some() {
            let start_coord = grid_number.start_coord;
            for dx in 0..grid_number.coord_length {
                let point = Point{x:start_coord.x+dx, y:start_coord.y};
                result.insert(point, grid_number);
            }
        }
    }
    result
}


fn solve_one(engine_schematic: &EngineSchematic) -> u64 {
    engine_schematic.grid_numbers().map(|n| n.part_number(engine_schematic).unwrap_or(0)).sum()
}

fn solve_two(engine_schematic: &EngineSchematic) -> u64 {
   engine_schematic.gears().map(|g| g.ratio).sum()
}

fn main() {
    let input_data = read_to_string("input.txt").expect("Read input.txt");
    let grid = AoCGrid::new(&input_data);
    let engine_schematic = EngineSchematic::new(&grid);
    println!("One: {}", solve_one(&engine_schematic));
    println!("Two: {}", solve_two(&engine_schematic));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = 
r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn grid() {
        let grid = AoCGrid::new(TEST_INPUT);
        assert_eq!(10, grid.width);
        assert_eq!(10, grid.height);
        assert_eq!(Some("4"), grid.get(&Point{x:0,y:0}));
        assert_eq!(Some("*"), grid.get(&Point{x:3,y:1}));
        assert_eq!(None, grid.get(&Point{x:11,y:0}));
        assert_eq!(None, grid.get(&Point{x:0,y:11}));
        assert_eq!(vec!["4", "6", "7", ".", ".", ".", ".", "3"], grid.adjacent(&Point{x:1,y:1}).collect::<Vec<_>>())
    }

    #[test]
    fn test_grid_iterator() {
        let grid = AoCGrid::new(TEST_INPUT);
        let es = EngineSchematic::new(&grid);
        for p in GridIterator::new(&es) {
            assert!(grid.valid_coordinate(&p));
        }
    }

    #[test]
    fn engine_schematic() {
        let grid = AoCGrid::new(TEST_INPUT);
        let es = EngineSchematic::new(&grid);
        assert_eq!(vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598], es.grid_numbers().map(|g| g.value).collect::<Vec<u64>>());
    }

    #[test]
    fn wtf() {
        let input = r"...123.
.......";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        let v: Vec<u64> = es.grid_numbers().map(|g| g.value).collect();
        assert_eq!(vec![123], v);
    }

    #[test]
    fn gridnumberiterator() {
        let input: &str = r"12.34
56...
7..89";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        let mut iter = GridNumberIterator::new(&es);
        assert_eq!(Some(GridNumber{value: 12, start_coord: Point{x:0,y:0}, coord_length:2}), iter.next());
        assert_eq!(Some(GridNumber{value: 34, start_coord: Point{x:3,y:0}, coord_length:2}), iter.next());
        assert_eq!(Some(GridNumber{value: 56, start_coord: Point{x:0,y:1}, coord_length:2}), iter.next());
        assert_eq!(Some(GridNumber{value: 7, start_coord: Point{x:0,y:2}, coord_length:1}), iter.next());
        assert_eq!(Some(GridNumber{value: 89, start_coord: Point{x:3,y:2}, coord_length:2}), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn part_one() {
        let grid = AoCGrid::new(TEST_INPUT);
        let es = EngineSchematic::new(&grid);
        assert_eq!(4361, solve_one(&es));
    }

    #[test]
    fn part_two() {
        let grid = AoCGrid::new(TEST_INPUT);
        let es = EngineSchematic::new(&grid);
        assert_eq!(467835, solve_two(&es));
    }

    #[test]
    fn part_one_bigger() {
        let input: &str = 
r".........232.633.......................803..........................361................192............539.................973.221...340.....
.............*..............#.....256.#.........329....................*313............*.......766.......*..........472..-...........+..249.
670-..@.......181......814..865.........968......@.......605....128.............%......798.638...+....776...........*......%...........*....";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(vec![232,633,803,361,192,539,973,221,340,256,329,313,766,472,249,670,181,814,865,968,605,128,798,638,776], es.grid_numbers().map(|g| g.value).collect::<Vec<u64>>());
    }

    #[test]
    fn start_end() {
        let input: &str = r"12.34
56...
7..89";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(vec![12,34,56,7,89], es.grid_numbers().map(|g| g.value).collect::<Vec<u64>>());
    }

    #[test]
    fn edge_adjacency() {
        let input = r"123
456
789";
        let grid = AoCGrid::new(input);
        let adj: Vec<&str> = grid.adjacent(&Point{x:2,y:1}).collect();
        assert_eq!(vec!["2", "3", "5", "8", "9"], adj);
    }

    #[test]
    fn datatype() {
        assert_eq!(GridDataType::Symbol, "$".parse().unwrap());
    }

    #[test]
    fn reddit() {
        let input: &str =
r"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(925, solve_one(&es));
    }

    #[test]
    fn reddit2() {
        let input: &str = r"........
.24..4..
......*.";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(2, es.grid_numbers().count());
    }

    #[test]
    fn reddit3() {
        let input: &str = r"....................
..-52..52-..52..52..
..................-.";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(4, es.grid_numbers().count());
        assert_eq!(156, solve_one(&es))
    }

    #[test]
    fn reddit4() {
        let input: &str = r".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(62, solve_one(&es));
    }

    #[test]
    fn detects_all_numbers_from_large_input() {
        let input = read_to_string("input.txt").expect("Read input data");
        let grid = AoCGrid::new(&input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(vec![232,633,803,361,192,539,973,221,340,256,329,313,766,472,249,670,181,814,865,968,605,128,798,638,776,563,741,815,921,428,219,993,584,990,431,466,971,815,634,197,887,114,521,796,713,546,941,837,903,910,988,61,946,240,697,563,707,895,223,160,618,61,603,495,633,697,910,70,497,568,832,551,863,324,837,701,740,72,98,245,145,832,580,432,315,4,174,971,76,472,66,260,348,179,908,108,726,654,422,501,644,279,528,913,639,131,5,228,900,148,665,220,561,237,576,381,771,416,996,799,441,355,570,481,422,798,924,462,420,659,404,233,955,265,86,43,1,398,624,896,53,855,301,688,614,103,486,672,725,508,993,906,124,92,208,626,298,810,428,461,619,590,636,683,128,524,507,636,991,52,61,993,627,796,841,105,313,555,715,625,963,194,505,841,442,108,58,450,343,138,560,561,53,327,766,234,276,370,913,16,825,111,561,446,372,8,136,758,349,666,340,639,214,364,31,440,644,577,382,175,84,931,692,860,400,235,797,863,683,778,367,79,192,897,320,634,32,556,783,475,781,875,891,867,322,8,938,318,462,620,293,330,26,668,205,32,975,750,25,521,388,116,33,2,519,717,859,881,109,828,927,240,66,27,482,227,968,479,91,598,102,615,184,456,385,476,13,391,526,90,500,14,206,57,53,134,784,775,692,88,873,115,9,937,242,729,342,344,568,140,521,185,462,331,337,90,829,262,376,787,352,227,413,518,796,698,346,277,918,902,327,120,320,902,488,150,688,822,721,3,445,132,71,880,770,150,674,924,746,403,929,771,110,63,847,423,651,729,927,867,577,763,55,320,674,962,421,707,222,301,702,943,431,59,600,756,593,352,579,965,607,669,406,704,720,333,839,449,210,219,84,842,582,350,831,394,835,184,676,755,22,710,86,889,86,625,195,547,549,945,601,975,285,743,433,619,675,204,161,493,896,576,328,902,819,362,373,854,272,812,933,447,950,124,990,172,139,530,27,844,486,810,826,880,359,242,432,206,519,805,66,859,943,742,116,421,984,559,566,790,372,307,180,532,135,88,417,576,138,314,776,670,893,565,985,833,369,372,842,868,221,168,128,500,962,31,143,897,727,42,24,382,593,414,165,179,42,468,362,39,802,339,240,257,386,262,556,852,670,872,480,945,983,604,997,182,916,800,165,927,55,521,394,142,672,967,107,785,208,614,386,975,923,273,146,70,177,550,606,430,35,707,157,334,675,719,762,960,343,498,291,654,592,54,500,772,252,689,357,778,273,455,381,117,388,386,258,948,914,514,476,975,274,119,56,94,390,250,484,723,415,451,2,115,818,859,401,240,205,228,757,102,954,863,523,613,844,832,35,989,381,827,702,592,456,385,78,233,27,49,574,230,311,326,617,585,798,699,20,687,662,246,735,61,361,171,380,786,378,624,836,742,322,195,634,422,893,106,960,121,969,738,919,78,685,773,654,414,297,44,718,718,841,446,881,825,870,104,341,663,292,165,466,892,296,948,748,99,707,339,483,896,241,494,227,821,761,143,329,24,54,202,588,481,203,37,90,390,171,80,857,689,930,794,233,503,62,14,438,149,492,842,721,301,958,265,628,9,813,671,518,903,974,120,269,560,907,214,961,22,740,825,612,740,307,467,350,535,665,138,831,487,432,348,32,529,395,318,984,492,835,735,333,136,779,848,486,736,507,329,189,745,223,552,888,53,415,930,845,3,99,599,731,986,582,669,367,969,132,13,420,442,793,997,908,148,961,397,9,144,736,942,346,970,72,794,608,13,993,462,539,560,637,208,896,5,856,178,727,787,593,736,191,609,774,62,325,372,994,513,853,907,43,511,850,49,696,856,651,314,213,718,70,659,453,123,921,11,32,312,795,467,689,760,997,545,597,808,565,115,522,179,606,269,885,733,857,857,252,510,842,518,678,420,426,92,447,995,761,158,128,178,495,748,927,411,519,430,480,667,266,846,625,807,561,687,268,55,60,556,56,67,698,593,485,166,174,944,591,808,698,376,891,951,538,563,472,584,460,492,716,238,297,23,90,668,798,351,720,513,476,312,745,35,550,885,343,937,435,882,556,417,691,671,609,424,675,650,9,900,867,975,897,905,122,555,796,530,229,585,22,456,495,107,800,683,876,181,954,774,643,437,310,494,265,320,13,258,632,677,227,922,778,384,908,156,533,192,420,861,771,553,860,370,309,483,500,773,900,249,93,171,248,315,470,169,198,482,771,267,835,6,459,252,496,334,493,991,191,521,661,514,832,296,650,363,442,942,58,390,865,16,586,993,255,55,337,334,82,490,5,381,16,867,35,427,877,768,110,413,104,90,623,433,462,33,685,228,288,513,721,717,344,970,953,546,162,637,37,740,331,564,556,843,195,2,937,255,349,837,342,411,537,337,791,641,424,24,261,667,551,324,330,841,465,486,996,227,33,681,354,455,304,542,64,624,716,245,166,331,738,249,126,833,913,690,943,284,938,224,429,195,231,857,975,252,71,599,279,828,967,285,798,370,640,898,746,134,329,768,279,840,979,374,192,370,964,970,436,410,306,727,139,689,819,498,982,131,566,390,505,84,973,830,394,401,562,907,405,321,455,284,722,124,921,303,652,286,775,274,74,774,986,96,469,335,526,344,31,942,31,846,72,582,380,570,201,648,838,253,101,606,744,792,396,990,609,938,896,125,842,485,510,801,329,983,963,761,927,45,981,675,676,156,30,998,697,14,366,960,874,497,278],
        es.grid_numbers().map(|g| g.value).collect::<Vec<u64>>());
    }

    #[test]
    fn weird() {
        let input = r"................713.546......*........941......*..*..837............903...............910.........988....61..........&..946..240......697...";
        let grid = AoCGrid::new(input);
        let es = EngineSchematic::new(&grid);
        assert_eq!(vec![713,546,941,837,903,910,988,61,946,240,697], es.grid_numbers().map(|g| g.value).collect::<Vec<u64>>());

    }


}
