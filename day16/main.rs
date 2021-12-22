use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::bits::{bits, complete::tag, streaming::take};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::IResult;

type Packet = (u8, Literal);
type Literal = (u8, Integer);
type Integer = i32;

fn parse_packet(input: &[u8]) -> IResult<&[u8], Packet> {
    let version = tag(0x06, 3usize);

    let literal_id = tag(0x04, 3usize);

    let byte_with_next = map(tuple((tag(0x01, 1usize), take(4usize))), |(_, x)| x);
    let terminal_byte = map(tuple((tag(0x00, 1usize), take(4usize))), |(_, b)| b);

    // let integer = tuple((many_m_n(0, 3, byte_with_next), terminal_byte));
    let integer = map(
        tuple((many_m_n(0, 3, byte_with_next), terminal_byte)),
        |(parts, terminator): (Vec<i32>, _)| {
            let mut out = terminator;
            parts
                .iter()
                .rev()
                .enumerate()
                .for_each(|(i, x)| out |= x << ((i + 1) * 4));
            out
        },
    );

    let literal = tuple((literal_id, integer));

    // let operator_id = take(3usize);
    // let operator = tuple((operator_id,));

    // let packet = alt((literal, operator));

    // let a = take(5usize);
    // let b = take(5usize);
    // let c = take(5usize);

    let packet = tuple((version, literal));

    bits::<_, _, Error<(&[u8], usize)>, _, _>(packet)(input)
}

fn main() {
    if let Ok(lines) = read_lines("./day16/example.txt") {
        let input = &[0xd2, 0xfe, 0x28];
        let (_, (version, l)) = parse_packet(input).unwrap();
        println!("{}/{:?}", version, l);
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
