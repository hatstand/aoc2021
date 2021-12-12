use itertools::Itertools;
use itertools::multizip;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("./day3/input.txt") {
        let diagnostics: Vec<String> = lines
            .filter_map(|l| l.ok())
            .collect();

        let v2: Vec<&str> = diagnostics.iter().map(|s| &**s).collect();

        println!("valid lines: {}", diagnostics.len());

        let g = gamma(&diagnostics);
        let e = epsilon(&diagnostics);

        println!("{}/{} => {}", g, e, g*e);
    }
}

fn gamma(input: &Vec<String>) -> u64 {
    let mut gamma: u64 = 0;
    for i in 0..12 {
        let (ones, zeroes) = input.iter()
            .map(|s| s.chars().nth(i).unwrap())
            .fold((0, 0), |(ones, zeros), c| {
                match c {
                    '0' => (ones, zeros + 1),
                    '1' => (ones + 1, zeros),
                    _ => panic!("invalid input"),
                }
            });
        let bit = if ones > zeroes { 1 } else { 0 };

        gamma = gamma | (bit << 11-i);
    }

    gamma
}

fn epsilon(input: &Vec<String>) -> u64 {
    let mut epsilon: u64 = 0;
    for i in 0..12 {
        let (ones, zeroes) = input.iter()
            .map(|s| s.chars().nth(i).unwrap())
            .fold((0, 0), |(ones, zeros), c| {
                match c {
                    '0' => (ones, zeros + 1),
                    '1' => (ones + 1, zeros),
                    _ => panic!("invalid input"),
                }
            });
        let bit = if ones > zeroes { 0 } else { 1 };

        epsilon = epsilon | (bit << 11-i);
    }

    epsilon
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
