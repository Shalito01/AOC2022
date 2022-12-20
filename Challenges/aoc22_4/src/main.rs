use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut tot = 0;
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 || &args[1] != "--file" {
        println!("You must enter file path:\n\t-f FILEPATH");
        return;
    }
    
    
    let f_path = &args[2];


    if let Ok(lines) =
        read_lines(f_path)
    {
        for line in lines {
            let ln = match line {
                Ok(r) => String::from(r),
                Err(_e) => String::new(),
            };
            let sections: Vec<&str> = ln.split(',').collect();
            let first_rg: Vec<&str> = sections[0].split('-').collect();
            let second_rg: Vec<&str> = sections[1].split('-').collect();

            let i1: i32 = first_rg[0].parse::<i32>().unwrap();
            let f1: i32 = first_rg[1].parse::<i32>().unwrap();
            let i2: i32 = second_rg[0].parse::<i32>().unwrap();
            let f2: i32 = second_rg[1].parse::<i32>().unwrap();

            // Part One
            if i1 >= i2 && f1 <= f2 {
                tot += 1;
            } else if i1 <= i2 && f1 >= f2 {
                tot += 1;
            }

            // Part Two
            if i1 > i2 && i1 < f2 {
                tot += 1;
            } else if f1 > i2 && f1 < f2 {
                tot += 1;
            } else if i2 > i1 && i2 < f1 {
                tot += 1;
            } else if f2 > i1 && f2 < f1 {
                tot += 1;
            } else if i1 == i2 || i1 == f2 || f1 == i2 || f1 == f2 {
                tot += 1;
            }
        }
    }
    println!("{}", tot);
}
