use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day6/input.txt") {
        let line = lines.last().unwrap().unwrap();

        let fish: Vec<u8> = line.split(",").map(|x| x.parse().unwrap()).collect();

        // Simulate each _type_ of fish and build a lookup table.
        let types = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let n = 80;
        let m: HashMap<_, _> = types
            .iter()
            .map(|t| {
                let start = vec![*t];

                let end_result = (0..n).fold(start, |previous, _| {
                    let next: Vec<_> = previous
                        .iter()
                        .flat_map(|f| {
                            let next: Vec<_> = match f {
                                0 => vec![6, 8],
                                1 => vec![0],
                                2 => vec![1],
                                3 => vec![2],
                                4 => vec![3],
                                5 => vec![4],
                                6 => vec![5],
                                7 => vec![6],
                                8 => vec![7],
                                _ => panic!("weird fish"),
                            };
                            next
                        })
                        .collect();
                    next
                });

                (t, end_result.len())
            })
            .collect();
        println!("{:?}", m);

        let q: usize = fish.iter().map(|f| m.get(f).unwrap()).sum();

        println!("{}", q);
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
