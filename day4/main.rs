use grid::Grid;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day4/input.txt") {
        let ls: Vec<String> = lines.filter_map(|line| line.ok()).collect();
        let drawing: Vec<u32> = ls
            .first()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        let grid_rows: Vec<_> = ls
            .iter()
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(|row| {
                row.split_whitespace()
                    .map(|cell| cell.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let grids: Vec<Grid<u32>> = grid_rows
            .iter()
            .tuples::<(_, _, _, _, _)>()
            .map(|(a, b, c, d, e)| {
                let mut g = Grid::new(1, 1);
                g.clear();
                g.push_row(a.to_vec());
                g.push_row(b.to_vec());
                g.push_row(c.to_vec());
                g.push_row(d.to_vec());
                g.push_row(e.to_vec());
                assert_eq!(g.rows(), 5);
                assert_eq!(g.cols(), 5);
                g
            })
            .collect();

        let mut previous_winners: BTreeSet<usize> = BTreeSet::new();

        for i in 0..drawing.len() {
            let so_far = drawing[0..i].to_vec();
            let winners: BTreeSet<usize> = grids
                .iter()
                .enumerate()
                .filter(|(_i, g)| is_winner(&g, &so_far))
                .map(|(i, _g)| i)
                .collect();
            let new_winners: BTreeSet<usize> =
                winners.difference(&previous_winners).map(|i| *i).collect();

            if previous_winners.union(&new_winners).count() == grids.len() {
                // This is the final winner!
                let last = grids.get(*new_winners.iter().next().unwrap()).unwrap();
                println!("last grid: {:?}", last);

                let sum: u32 = last.iter().filter(|c| !so_far.contains(c)).sum();
                let last_draw = so_far.last().unwrap();
                println!(
                    "sum: {} last_draw: {} score: {}",
                    sum,
                    last_draw,
                    sum * last_draw
                );
                return;
            }

            new_winners.iter().for_each(|i| {
                previous_winners.insert(*i);
            });
        }
    }
}

fn is_winner(grid: &Grid<u32>, drawing: &Vec<u32>) -> bool {
    // Rows
    for i in 0..grid.rows() {
        if grid.iter_row(i).all(|cell| drawing.contains(&cell)) {
            return true;
        }
    }

    // Columns
    for i in 0..grid.cols() {
        if grid.iter_col(i).all(|cell| drawing.contains(&cell)) {
            return true;
        }
    }

    false
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
