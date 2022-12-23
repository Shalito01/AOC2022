use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;
use std::fs::File;
use std::collections::VecDeque;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>> 
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut q: VecDeque<char> = VecDeque::new();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || &args[1] != "--file" {
        println!("Specify path of input file:\n\t--file FILEPATH");
        return;
    }
    
    if let Ok(lines) = read_lines(&args[2])
    {
        for line in lines
        {
            let buffer: String = line.unwrap();
            let mut diff = true;
            let mut counter: i32 = 1;

            for ch in buffer.chars()
            {
                
                if q.len() == 14 {   
                    q.pop_front();
                }
                
                
                diff = true;
                q.push_back(ch);

                for x in q.iter() {
                    let mut i = 0;
                    for y in q.iter() {
                        if *x == *y {
                            i += 1;
                        }
                    }

                    if i > 1 {
                        diff = false;
                        break;
                    }
                }
                
                // Part One = 4 Part two = 14
                if diff == true && q.len() == 14 {
                    println!("Found after {} chars", counter);
                    return;
                }
                counter += 1;
            }

                        println!("Found after {} chars", counter);
        }
    }

}
