use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::bits::{bits, streaming::tag, streaming::take};
use nom::branch::alt;
use nom::combinator::map;
use nom::error::Error;
use nom::multi::{length_count, many_m_n};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
struct Literal {
    version: u8,
    value: i32,
}

#[derive(Debug)]
struct Operator {
    version: u8,
    id: u8,
    length_type: u8,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

fn version(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn literal_id(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    tag(0x04, 3usize)(input)
}

fn integer(input: (&[u8], usize)) -> IResult<(&[u8], usize), i32> {
    let byte_with_next = map(tuple((tag(0x01, 1usize), take(4usize))), |(_, x)| x);
    let terminal_byte = map(tuple((tag(0x00, 1usize), take(4usize))), |(_, b)| b);
    map(
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
    )(input)
}

fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    map(
        tuple((version, literal_id, integer)),
        |(version, _, value)| -> Packet {
            Packet::Literal(Literal {
                version: version,
                value: value,
            })
        },
    )(input)
}

fn num_subpackets(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    // take(8usize)(input)

    map(
        tuple((take(8usize), take(3usize))),
        |(msb, lsb): (u8, u8)| lsb | (msb << 3),
    )(input)
}

fn hmm(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn subpacket(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(11usize)(input)
}

fn operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let operator_id = take(3usize);
    let length_type = take(1usize);
    let subpackets = length_count(num_subpackets, packet);
    map(
        tuple((version, operator_id, length_type, subpackets)),
        |(version, id, length_type, subpackets)| -> Packet {
            Packet::Operator(Operator {
                id: id,
                version: version,
                length_type: length_type,
                subpackets: subpackets,
            })
        },
    )(input)
}

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    alt((literal, operator))(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], Packet> {
    bits::<_, _, Error<(&[u8], usize)>, _, _>(packet)(input)
}

// type InputType = (&[u8], usize);

fn main() {
    if let Ok(lines) = read_lines("./day16/example.txt") {
        // let input = &[0xd2, 0xfe, 0x28];
        let input = &[0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60];
        let (left, packet) = parse(input).unwrap();
        // let (_, packet) = parse_packet(input).unwrap();
        println!("{:?}", packet);
        println!("{:?}", left);
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
