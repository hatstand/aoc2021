use grid::Grid;
use itertools::Itertools;
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

        for i in 0..drawing.len() {
            let so_far = drawing[0..i].to_vec();
            let winners: Vec<_> = grids.iter().filter(|g| is_winner(&g, &so_far)).collect();
            if !winners.is_empty() {
                println!("Winner: {:?}", winners);
                println!("drawn to: {:?}", so_far);

                let winner = winners.first().unwrap();
                let sum: u32 = winner.iter().filter(|c| !so_far.contains(c)).sum();
                let last_draw = so_far.last().unwrap();
                println!(
                    "sum: {} last_draw: {} score: {}",
                    sum,
                    last_draw,
                    sum * last_draw
                );
                return;
            }
        }
    }
}

fn is_winner(grid: &Grid<u32>, drawing: &Vec<u32>) -> bool {
    // Rows
    for i in 0..grid.rows() {
        if grid.iter_row(i).all(|cell| drawing.contains(&cell)) {
            println!("Row winner: {:?}", grid.iter_row(i));
            return true;
        }
    }

    // Columns
    for i in 0..grid.cols() {
        if grid.iter_col(i).all(|cell| drawing.contains(&cell)) {
            println!("Col winner: {:?}", grid.iter_col(i));
            return true;
        }
    }

    // // Diagonal 1
    // if (0..grid.rows())
    //     .map(|j| grid.get(j, j).unwrap())
    //     .all(|cell| drawing.contains(&cell))
    // {
    //     println!("Diagonal 1 winner");
    //     return true;
    // }

    // // Diagonal 2
    // if (0..grid.rows())
    //     .map(|j| grid.get(j, grid.rows() - j - 1).unwrap())
    //     .all(|cell| drawing.contains(&cell))
    // {
    //     println!("Diagonal 2 winner");
    //     return true;
    // }

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
