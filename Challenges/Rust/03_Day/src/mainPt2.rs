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
    let mut i = 1;
    let mut rk: [String; 3] = ["".to_string(), "".to_string(), "".to_string()];
    
    if let Ok(lines) = read_lines("/Users/daniel/Documents/GitHub/AOC2022/Challenges/aoc22_3/src/input.txt") {
        for line in lines {
            let backpack = match line {
                Ok(r)=> String::from(r),
                Err(_e)=> String::new(),
            };
            
            
            if i <= 3 {
                rk[i-1] = backpack.clone();
            }
                
            if i == 3 {
                for ch in rk[0].chars() {
                    if rk[1].contains(ch) && rk[2].contains(ch) {
                        let mut x = ch as i32;
                        if ch.is_lowercase() {
                            x -= 96;
                        } else {
                            x -= 38;
                        }
                        tot += x;
                        println!("{}, {}, {}, {}, {}", rk[0], rk[1], rk[2], ch, x);
                        break;
                    }
                }
            }
            if i == 3 {
                i = 1;
            } else {
                i += 1;
            }
        }
    }

    println!("{}", tot);
}
