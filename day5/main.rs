use grid::Grid;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    if let Ok(lines) = read_lines("./day5/input.txt") {
        let ls: Vec<String> = lines.filter_map(|line| line.ok()).collect();

        let coords: Vec<_> = ls
            .iter()
            .map(|l| {
                let caps = re.captures(l).unwrap();
                let x1: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
                let y1: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
                let x2: u64 = caps.get(3).unwrap().as_str().parse().unwrap();
                let y2: u64 = caps.get(4).unwrap().as_str().parse().unwrap();
                ((x1, y1), (x2, y2))
            })
            .collect();
        println!("{:?}", coords);

        let grid_size = coords
            .iter()
            .fold((0, 0), |(max_x, max_y), ((x1, y1), (x2, y2))| {
                (max(max(max_x, *x1), *x2), max(max(max_y, *y1), *y2))
            });

        println!("{:?}", grid_size);

        let mut grid: Grid<u64> = Grid::new((grid_size.1 + 1) as usize, (grid_size.0 + 1) as usize);

        for ((x1, y1), (x2, y2)) in coords {
            // Only horizontal & vertical for now.
            if x1 == x2 {
                let start = min(y1, y2);
                let end = max(y1, y2);
                for y in start..=end {
                    *grid.get_mut(y as usize, x1 as usize).unwrap() += 1;
                }
            } else if y1 == y2 {
                let start = min(x1, x2);
                let end = max(x1, x2);
                for x in start..=end {
                    *grid.get_mut(y1 as usize, x as usize).unwrap() += 1;
                }
            }
        }

        print_grid(&grid);

        let scary = grid.iter().filter(|c| *c > &1).count();
        println!("{}", scary);
    }
}

fn print_grid(grid: &Grid<u64>) {
    for row in 0..grid.rows() {
        grid.iter_row(row).for_each(|c| print!("{}", c));
        println!();
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
