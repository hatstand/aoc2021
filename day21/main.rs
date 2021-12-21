use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let re = Regex::new(r"\d+$").unwrap();

    if let Ok(lines) = read_lines("./day21/input.txt") {
        let starting_positions: Vec<u32> = lines
            .filter_map(|l| l.ok())
            .filter_map(|l| {
                let caps = re.captures(&l)?;
                let pos = caps.get(0);
                pos.map(|m| m.as_str().parse::<u32>().ok()).flatten()
            })
            .collect();

        println!("{:?}", starting_positions);

        let mut player_one = starting_positions[0] - 1;
        let mut player_two = starting_positions[1] - 1;
        let mut player_one_score = 0;
        let mut player_two_score = 0;

        let mut die_rolls = 0;

        fn die() -> impl std::iter::Iterator<Item = u32> {
            let mut num: u32 = 99;
            std::iter::from_fn(move || {
                num += 1;
                Some((num % 100) + 1)
            })
        }

        let mut roll = die();

        loop {
            let sum_one: u32 = roll.by_ref().take(3).sum();
            die_rolls += 3;
            player_one = ((player_one + sum_one) % 10);
            player_one_score += player_one + 1;
            if player_one_score >= 1000 {
                println!("{}", player_two_score * die_rolls);
                break;
            }

            let sum_two: u32 = roll.by_ref().take(3).sum();
            die_rolls += 3;
            player_two = (player_two + sum_two) % 10;
            player_two_score += player_two + 1;
            if player_two_score >= 1000 {
                println!("{}", player_one_score * die_rolls);
                break;
            }

            println!("{} {}", player_one_score, player_two_score);
        }
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
