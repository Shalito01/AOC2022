use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn compute_res<'a>(f: &'a str, u: &'a str) -> &'a str {
    if u == "Y" {
        // Draw
        return f;
    } else if u == "X" {
        // Lose
        match f {
            "A"=> return "C",
            "B"=> return "A",
            "C"=> return "B",
            &_=> "",
        }
    } else  {
        // Win
        match f {
            "A"=> return "B",
            "B"=> return "C",
            "C"=> return "A",
            &_=> "",
        }
    }
}

fn main() {
    let mut usr;
    let mut foe;
    let mut total_score = 0;
    let mut ch_score = 0;
    let mut round_score = 0;

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
                    "X"=>round_score = 0,
                    "Y"=>round_score = 3,
                    "Z"=>round_score = 6,
                    &_=> print!(""),
                }

                usr = compute_res(foe, usr);
                
                match usr {
                    "A"=>ch_score = 1,
                    "B"=>ch_score = 2,
                    "C"=>ch_score = 3,
                    &_=> print!(""),
                }

                total_score = total_score + ch_score + round_score;
            }
        }
    }

    println!("{}", total_score);
}
