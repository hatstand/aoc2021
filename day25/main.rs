use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use wrapping_coords2d::WrappingCoords2d;

fn main() {
    if let Ok(lines) = read_lines("./day25/example.txt") {
        let input: Vec<char> = lines.map(|l| l.unwrap().chars().collect())
            .fold(vec![], |mut acc, cs: Vec<char>| {
                cs.iter().for_each(|c| acc.push(*c));
                acc
            });
        let width: usize = 10;

        print(&input, width);
        println!();

        let mut count = 1;
        let mut last = input;
        loop {
            let next = iteration(&last, width);
            println!("Iteration: {}", count);
            print(&next, width);
            println!();

            if eq(&next, &last) {
                break;
            }

            count += 1;

            last = next;
        }

        println!("iterations: {}", count);
    }
}

fn eq(a: &Vec<char>, b: &Vec<char>) -> bool {
    assert_eq!(a.len(), b.len());

    a.iter().partial_cmp(b.iter()).unwrap() == std::cmp::Ordering::Equal
}

fn print(input: &Vec<char>, width: usize) {
    for j in 0..input.len() / width {
        for i in 0..width {
            print!("{}", input[j*width+i])
        }
        println!();
    }
}

fn iteration(input: &Vec<char>, width: usize) -> Vec<char> {
    let w2d = WrappingCoords2d::new(width as i32, (input.len() / width) as i32).unwrap();

    let mut out = vec!['.'; input.len()];
    for i in 0..w2d.width() {
        for j in 0..w2d.height() {
            let cell = input[w2d.index(i, j)];
            if cell == '>' {
                let adjacent_cell = input[w2d.index(i+1, j)];
                if adjacent_cell == '.' {
                    out[w2d.index(i+1, j)] = '>';
                } else {
                    out[w2d.index(i, j)] = '>';
                }
            }
        }
    }

    for i in 0..w2d.width() {
        for j in 0..w2d.height() {
            let index = w2d.index(i, j);
            let cell = input[index];
            if cell == 'v' {
                let adjacent_coord = w2d.index(i, j+1);
                let adjacent_vert = input[adjacent_coord];
                let adjacent_hori = out[adjacent_coord];

                if adjacent_vert == 'v' || adjacent_hori == '>' {
                    out[index] = 'v';
                } else {
                    out[adjacent_coord] = 'v';
                }
            }
        }
    }

    out
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
