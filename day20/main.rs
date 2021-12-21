use array2d::Array2D;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day20/input.txt") {
        let ls: Vec<_> = lines.filter_map(|l| l.ok()).collect();
        let algo_chars: Vec<char> = ls
            .iter()
            .map_while(|l| {
                if l.is_empty() {
                    None
                } else {
                    let c: Vec<_> = l.chars().collect();
                    Some(c)
                }
            })
            .flatten()
            .collect();

        let image_lines: Vec<_> = ls
            .iter()
            .rev()
            .map_while(|l| {
                if l.is_empty() {
                    None
                } else {
                    let c: Vec<_> = l.chars().collect();
                    Some(c)
                }
            })
            .collect();

        let image_chars: Vec<char> = image_lines.iter().rev().flatten().map(|c| *c).collect();
        let s = (image_chars.len() as f64).sqrt().floor() as usize;
        let img = Array2D::from_row_major(&image_chars, s, s);
        println!("original {}x{}", img.num_columns(), img.num_rows());
        print_img(&img);
        println!("");

        let default_for_index = |i: usize| -> bool {
            if algo_chars[0] == '.' {
                false
            } else {
                i % 2 != 0
            }
        };

        let enhanced = enhance_img(&img, &algo_chars, default_for_index(0));
        println!(
            "enhance x1 {}x{}",
            enhanced.num_columns(),
            enhanced.num_rows()
        );
        print_img(&enhanced);
        println!("");

        let enhanced_2 = enhance_img(&enhanced, &algo_chars, default_for_index(1));
        println!(
            "enhance x2 {}x{}",
            enhanced_2.num_columns(),
            enhanced_2.num_rows()
        );
        print_img(&enhanced_2);

        let light_pixels = enhanced_2
            .elements_row_major_iter()
            .filter(|&c| *c == '#')
            .count();
        println!("light pixels: {}", light_pixels);
    }
}

fn enhance_img(img: &Array2D<char>, algo_chars: &Vec<char>, default: bool) -> Array2D<char> {
    assert_eq!(img.num_columns(), img.num_rows());
    let get_pixel = |x: i32, y: i32| -> bool {
        if x < 0 || y < 0 {
            default
        } else {
            img.get(y as usize, x as usize)
                .map_or(default, |c| *c == '#')
        }
    };
    let parse_pixels = |x: i32, y: i32| -> u16 {
        let mut out = 0;
        let mut index = 9;

        for j in y - 1..=y + 1 {
            for i in x - 1..=x + 1 {
                let n = get_pixel(i, j);
                index -= 1;
                out |= (n as u16) << index;
            }
        }
        assert_eq!(index, 0);
        out
    };

    let mut out_img = Array2D::filled_with('.', img.num_rows() + 2, img.num_columns() + 2);

    for j in -1..((img.num_rows() + 1) as i32) {
        for i in -1..((img.num_columns() + 1) as i32) {
            let p = parse_pixels(i, j);
            assert!((p as usize) < algo_chars.len());

            let out_c = algo_chars[p as usize];
            println!("({},{}) -> {:09b} ({}) -> {}", i, j, p, p, out_c);

            out_img
                .set((j + 1) as usize, (i + 1) as usize, out_c)
                .unwrap();
        }
    }
    out_img
}

fn print_img(img: &Array2D<char>) {
    for i in 0..img.num_rows() {
        img.row_iter(i).for_each(|c| print!("{}", c));
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
