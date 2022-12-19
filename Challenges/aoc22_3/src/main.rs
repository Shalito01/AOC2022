use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut tot = 0;

    if let Ok(lines) = read_lines("/Users/daniel/Documents/GitHub/AOC2022/Challenges/aoc22_3/src/input.txt") {
        for line in lines {
            let backpack = match line {
                Ok(r)=> String::from(r),
                Err(_e)=> String::new(),
            };
            let (cmpt_1, cmpt_2) = backpack.split_at(backpack.chars().count() / 2);
            for ch in cmpt_1.chars() {
                if cmpt_2.contains(ch) {
                    let mut x = ch as i32;
                    if ch.is_lowercase() {
                        x -= 96;
                    } else {
                        x -= 38;
                    }
                    tot += x;
                    println!("{}, {}, {}, {}", cmpt_1, cmpt_2, ch, x);
                    break;
                }
            }
        }
    }

    println!("{}", tot);
}
