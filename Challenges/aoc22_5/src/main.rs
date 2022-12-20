use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::env;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}


fn remove_from_stack<'a>(l: &mut Vec<&'a str>) -> &'a str{
    
    if !l.is_empty() {
        return l.pop().unwrap();
    } else {
        return "";
    }
}
fn disp(l: &mut Vec<&str>) {
    print!("{}", l.last().unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || &args[1] != "--file" {
        println!("Specify path of input file:\n\t--file FILEPATH");
        return;
    }
    
    let mut l1 = vec!["Q", "M", "G", "C", "L"];
    let mut l2 = vec!["R", "D", "L", "C", "T", "F", "H", "G"];
    let mut l3 = vec!["V", "J", "F", "N", "M", "T", "W", "R"];
    let mut l4 = vec!["J", "F", "D", "V", "Q", "P"];
    let mut l5 = vec!["N", "F", "M", "S", "L", "B", "T"];
    let mut l6 = vec!["R", "N", "V", "H", "C", "D", "P"];
    let mut l7 = vec!["H", "C", "T"];
    let mut l8 = vec!["G", "S", "J", "V", "Z", "N", "H", "P"];
    let mut l9 = vec!["Z", "F", "H", "G"];
    
         if let Ok(lines) = read_lines(&args[2])
    {
        for line in lines {
            let buffer = match line {
                Ok(r) => String::from(r),
                Err(_e)=>String::new(),
            };
            
            let input: Vec<&str> = buffer.split(' ').collect();
            
            let count: i32 = input[1].parse::<i32>().unwrap();
            let from: i32 = input[3].parse::<i32>().unwrap();
            let to: i32 = input[5].parse::<i32>().unwrap();
            let mut temp = Vec::new();
            
            for _i in 1..(count+1) {
                let mut _x = "";

                match from {
                    1=> _x = remove_from_stack(&mut l1),
                    2=> _x = remove_from_stack(&mut l2),
                    3=> _x = remove_from_stack(&mut l3),
                    4=> _x = remove_from_stack(&mut l4),
                    5=> _x = remove_from_stack(&mut l5),
                    6=> _x = remove_from_stack(&mut l6),
                    7=> _x = remove_from_stack(&mut l7),
                    8=> _x = remove_from_stack(&mut l8),
                    9=> _x = remove_from_stack(&mut l9),
                    _=>break,
                }
                println!("move {} from {} to {}", _x, from, to);
                if _x.is_empty() {
                    continue;
                }
                temp.push(_x);
            }

            for _i in 1..(count+1) {
                match to {
                    1=> {
                        l1.push(temp.pop().unwrap());
                    },
                    2=>{
                        l2.push(temp.pop().unwrap());
                    },
                    3=>{
                        l3.push(temp.pop().unwrap());
                    },
                    4=>{
                        l4.push(temp.pop().unwrap());
                    },
                    5=>{
                        l5.push(temp.pop().unwrap());
                    },
                    6=>{
                        l6.push(temp.pop().unwrap());
                    },
                    7=>{
                        l7.push(temp.pop().unwrap());
                    },
                    8=>{
                        l8.push(temp.pop().unwrap());
                    },
                    9=>{
                        l9.push(temp.pop().unwrap());
                    },
                    _=>break,
                }
            }
        }
    }
    
    for i in 1..10 {
        match i {
                1=>disp(&mut l1),
                2=>disp(&mut l2),
                3=>disp(&mut l3),
                4=>disp(&mut l4),
                5=>disp(&mut l5),
                6=>disp(&mut l6),
                7=>disp(&mut l7),
                8=>disp(&mut l8),
                9=>disp(&mut l9),
                _=>println!(""),
        }
    }
}
