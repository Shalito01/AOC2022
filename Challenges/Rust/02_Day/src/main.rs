use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut usr;
    let mut foe;
    let mut total_score = 0;
    let mut ch_score = 0;
    let mut round_score;

    if let Ok(lines) = read_lines("/Users/daniel/Documents/GitHub/AOC2022/Challenges/aoc22_2/src/input.txt") {
        for line in lines {
            let str_line = match line {
                Ok(r) => String::from(r),
                Err(_err) => String::new()
            };
            if !str_line.is_empty() {
                let words: Vec<&str> = str_line.split(' ').collect();
                foe = words[0];
                usr = words[1];

                match usr {
                    "X"=>usr = "A",
                    "Y"=>usr = "B",
                    "Z"=>usr = "C",
                    &_=> print!(""),
                }
                
                match usr {
                    "A"=>ch_score = 1,
                    "B"=>ch_score = 2,
                    "C"=>ch_score = 3,
                    &_=> print!(""),
                }


                if foe == usr {
                    round_score = 3;
                } else if foe == "A" && usr == "B" || foe == "B" && usr == "C" || foe == "C" && usr == "A" {
                    round_score = 6;
                } else {
                    round_score = 0;
                }

                total_score = total_score + ch_score + round_score;
            }
        }
    }

    println!("{}", total_score);
}
