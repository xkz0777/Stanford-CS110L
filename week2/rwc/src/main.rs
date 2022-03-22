use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn count(filename: &String) -> Result<(usize, usize, usize), io::Error> {
    let file = File::open(filename)?;
    let mut words: usize = 0;
    let mut lines: usize = 0;
    let mut chars: usize = 0;

    for line in io::BufReader::new(file).lines().map(|line| line.unwrap()) {
        lines += 1;
        chars += line.len() + 1;
        let words_in_line: Vec<&str> = line.trim().split(' ').collect();
        words += words_in_line.len();
    }

    Ok((words, lines, chars))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let (num_words, num_lines, num_chars) = count(filename).expect("Invalid filename");
    println!(
        "There are {} words, {} lines, {} characters in {}",
        num_words, num_lines, num_chars, filename
    );
}
