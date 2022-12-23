use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut _max: i32 = 71780;
    let mut _max2: i32 = 0;
    let mut _max3: i32 = 0;
    let mut _curr: i32 = 0;
    let mut _i: i32 = 1;
    let mut _i_max: i32 = 1;

    if let Ok(lines) = read_lines("/Users/daniel/Documents/GitHub/AOC2022/Challenges/aoc22_1/src/puzzle.txt") {
        for line in lines {
            let str_line = match line {
                Ok(r) => String::from(r),
                Err(_err) => String::new()
            };
            if str_line.is_empty() {
                if _curr > _max2 && _curr < _max {
                    _max3 = _max2;
                    _max2 = _curr;
                }
                _curr = 0;
                _i += 1;
            } else {
                let int_line: i32 = str_line.parse::<i32>().unwrap();
                _curr += int_line;
            }
        }
    }

    println!("MaxTot = {}", _max + _max2 + _max3);
}


