use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day7/input.txt") {
        let line = lines.last().unwrap().unwrap();
        let mut crabs: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();

        // Find the median and then move all crabs towards it?
        crabs.sort();
        println!("{:?}", crabs);
        // let median = crabs[crabs.len() / 2];
        // println!("Aligning to {}", median);

        // Lol brute force?
        let min = *crabs.iter().min().unwrap();
        let max = *crabs.iter().max().unwrap();

        let v: Vec<_> = (min..max)
            .map(|cand| {
                crabs.iter().fold(0, |acc, x| {
                    // Triangular sequence. (n * (n + 1)) / 2
                    let dist = (x - cand).abs();
                    let cost = (dist * (dist + 1)) / 2;
                    cost + acc
                })
            })
            .collect();

        let res = v.iter().min().unwrap();

        // let res: i32 = crabs.iter().map(|x| (x - median).abs()).sum();
        println!("{}", res);
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
