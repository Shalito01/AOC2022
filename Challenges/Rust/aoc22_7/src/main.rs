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
    let mut pwd: String = String::new();
    let mut cwd: String = String::new();
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let input = File::open(config.file_path).unwrap();
    let file_content = BufReader::new(input);

    for line in file_content.lines() {
        println!("{}",line.unwrap());
    }

}
