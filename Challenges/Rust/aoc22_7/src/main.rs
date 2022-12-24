use std::process;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::env;
/*
fn cmd(c: &str, obj: &str, cwd: String, pwd: String) {
    if c == "ls" {
        count_size();
    }
}
*/
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn main() {
    let mut cwd: Vec<String> = Vec::new();
    let mut dict: Vec<(String, i32)> = Vec::new();
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let input = File::open(config.file_path).unwrap();
    let file_content = BufReader::new(input);

    for line in file_content.lines() {
        //println!("{}",line.unwrap());
        let buffer = line.unwrap();
        let arg: Vec<&str> = buffer.split(' ').collect();
        
        if arg[0] == "$" {
            if arg[1] == "cd" {
                if arg[2] == ".." {
                    cwd.pop();
                } else {
                    cwd.push(String::from(arg[2]));

                    let mut flag = true;
                    for x in dict.iter() {
                        if x.0 == arg[2] {
                            flag = false;
                            break;
                        }
                    }

                    if flag {
                        dict.push((String::from(arg[2]), 0));
                    }
                }
                //dict.push((String::from(arg[2]), 0));
            }
        }
    }

    for x in dict {
        println!("{} {}", x.0, x.1);
    }
}
