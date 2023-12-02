from io import TextIOBase
import re
import sys
from typing import List

FORWARDS = r"(?:one|two|three|four|five|six|seven|eight|nine|\d)"
BACKWARDS = r"(?:eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)"

TEST_INPUT=r"""two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""

def transform_match(match: str) -> int:
    match match:
        case "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0": return int(match)
        case "one" | "eno": return 1
        case "two" | "owt": return 2
        case "three" | "eerht": return 3
        case "four" | "ruof": return 4
        case "five" | "evif": return 5
        case "six" | "xis": return 6
        case "seven" | "neves": return 7
        case "eight" | "thgie": return 8
        case "nine" | "enin": return 9
        case _: raise RuntimeError(f"Unexpected match {match}")

def process_line(line: str) -> int:
    first = re.search(FORWARDS, line)
    if first is None: return 0
    first = 10 * transform_match(line[first.start():first.end()])

    r = ''.join(reversed(line))
    last = re.search(BACKWARDS, r)
    if last is None: return 0
    last = transform_match(r[last.start():last.end()])
    return first + last
    
def process_lines(lines: List[str]) -> int:
    return sum((process_line(l.strip()) for l in lines))

def process_textiobase(tiob: TextIOBase) -> int:
    return sum((process_line(l.strip()) for l in tiob))

def test():
    assert 281==process_lines(TEST_INPUT.splitlines())

def main():
    test()
    filename = sys.argv[1] if len(sys.argv)==2 else "input.txt"
    with open(filename, "r", encoding="utf-8") as f:
        print(process_textiobase(f))

if __name__ == "__main__":
    main()
