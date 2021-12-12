use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\w+)\s+(\d+)").unwrap();

    if let Ok(lines) = read_lines("./day2/input.txt") {
        let instructions: Vec<(_, _)> = lines
            .filter_map(|l| l.ok())
            .filter_map(|l| {
                let caps = re.captures(&l);
                caps.map(|c| (c[1].to_string(), c[2].parse::<u64>().unwrap()))
            })
            .collect();

        println!("{} instructions", instructions.len());

        let loc = instructions.iter().fold((0, 0, 0), |(horizontal, depth, aim), (instruction, amount)| {
            println!("processing: {:?}", (instruction, amount));

            match instruction.as_str() {
                "forward" => (horizontal + amount, depth + aim*amount, aim),
                "down" => (horizontal, depth, aim + amount),
                "up" => (horizontal, depth, aim - amount),
                _ => (horizontal, depth, aim),
            }
        });

        println!("{:?} ({})", loc, loc.0 * loc.1);
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}