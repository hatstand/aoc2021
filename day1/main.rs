use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("./day1/input.txt") {
        let depths: Vec<u64> = lines
            .filter_map(|l| l.ok())
            .filter_map(|l| l.parse().ok())
            .collect();

        println!("valid lines: {}", depths.len());

        let sums: Vec<_> = depths.iter().tuple_windows::<(_, _, _)>().map(|(a, b, c)| a+b+c).collect();

        let count = sums.iter().tuple_windows::<(_, _)>().fold(0, |acc, (a, b)| {
            if b > a { acc + 1 } else { acc }
        });

        println!("{}", count);
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
