use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day6/input.txt") {
        let line = lines.last().unwrap().unwrap();

        let fish: Vec<u8> = line.split(",").map(|x| x.parse().unwrap()).collect();

        let mut fishies: Vec<u64> = vec![0; 9];
        fish.iter().for_each(|x| {
            fishies[*x as usize] += 1;
        });

        // Simulate each _type_ of fish and build a lookup table.
        let n = 256;

        for _ in 0..n {
            let old_fish = fishies.clone();

            fishies[8] = old_fish[0];
            fishies[7] = old_fish[8];
            fishies[6] = old_fish[7] + old_fish[0];
            fishies[5] = old_fish[6];
            fishies[4] = old_fish[5];
            fishies[3] = old_fish[4];
            fishies[2] = old_fish[3];
            fishies[1] = old_fish[2];
            fishies[0] = old_fish[1];
        }

        println!("{:?}", fishies);
        println!("{}", fishies.iter().sum::<u64>());
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
