use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use nom::bits::{bits, streaming::tag, streaming::take};
use nom::branch::alt;
use nom::combinator::{map, rest_len};
use nom::error::Error;
use nom::multi::{length_count, length_data, many0, many_m_n};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
struct Literal {
    version: u8,
    value: u64,
}

#[derive(Debug)]
struct Operator {
    version: u8,
    operator: OperatorType,
    length_type: u8,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn version(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    take(3usize)(input)
}

fn literal_id(input: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    tag(0x04, 3usize)(input)
}

fn integer(input: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
    let byte_with_next = map(tuple((tag(0x01, 1usize), take(4usize))), |(_, x)| x);
    let terminal_byte = map(tuple((tag(0x00, 1usize), take(4usize))), |(_, b)| b);
    map(
        tuple((many0(byte_with_next), terminal_byte)),
        |(parts, terminator): (Vec<u64>, _)| {
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
            let p = Packet::Literal(Literal {
                version: version,
                value: value,
            });
            p
        },
    )(input)
}

fn num_subpackets(input: (&[u8], usize)) -> IResult<(&[u8], usize), u16> {
    take(11usize)(input)
}

fn length_data_packets(
) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>, Error<(&[u8], usize)>> {
    move |input: (&[u8], usize)| -> IResult<(&[u8], usize), Vec<Packet>, Error<(&[u8], usize)>> {
        let (mut rest, num_bits) = take(15usize)(input)?;
        let (_, starting_bits) = rest_len(rest)?;
        assert!(num_bits <= starting_bits);
        let mut bits_consumed = 0;
        let mut out = vec![];
        while bits_consumed != num_bits {
            let (new_rest, packet_data) = packet(rest)?;
            bits_consumed = starting_bits - rest_len(new_rest)?.1;
            rest = new_rest;
            out.push(packet_data);
        }

        Ok((rest, out))
    }
}

fn operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let operator_id = map(
        alt((
            tag(0x00, 3usize),
            tag(0x01, 3usize),
            tag(0x02, 3usize),
            tag(0x03, 3usize),
            // 0x04 is a literal.
            tag(0x05, 3usize),
            tag(0x06, 3usize),
            tag(0x07, 3usize),
        )),
        |x| match x {
            0x00 => OperatorType::Sum,
            0x01 => OperatorType::Product,
            0x02 => OperatorType::Minimum,
            0x03 => OperatorType::Maximum,
            0x05 => OperatorType::GreaterThan,
            0x06 => OperatorType::LessThan,
            0x07 => OperatorType::EqualTo,
            _ => unreachable!(),
        },
    );
    let subpackets_0 = tuple((tag(0x00, 1usize), length_data_packets()));
    let subpackets_1 = tuple((tag(0x01, 1usize), length_count(num_subpackets, packet)));

    let subpackets = alt((subpackets_0, subpackets_1));

    map(
        tuple((version, operator_id, subpackets)),
        |(version, operator, (length_type, subpackets))| -> Packet {
            let p = Packet::Operator(Operator {
                operator: operator,
                version: version,
                length_type: length_type,
                subpackets: subpackets,
            });

            p
        },
    )(input)
}

fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    alt((literal, operator))(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], Packet> {
    bits::<_, _, Error<(&[u8], usize)>, _, _>(packet)(input)
}

fn sum_packet(p: &Packet) -> i32 {
    match p {
        Packet::Literal(lit) => lit.version as i32,
        Packet::Operator(op) => {
            // op.version as i32
            op.subpackets
                .iter()
                .fold(op.version as i32, |acc, x| acc + sum_packet(x))
        }
    }
}

fn calculate_packet(p: &Packet) -> i64 {
    match p {
        Packet::Literal(lit) => lit.value as i64,
        Packet::Operator(op) => match op.operator {
            OperatorType::Sum => op
                .subpackets
                .iter()
                .fold(0, |acc, x| acc + calculate_packet(x)),
            OperatorType::Product => op
                .subpackets
                .iter()
                .fold(1, |acc, x| acc * calculate_packet(x)),
            OperatorType::Minimum => op
                .subpackets
                .iter()
                .map(|x| calculate_packet(x))
                .min()
                .unwrap(),
            OperatorType::Maximum => op
                .subpackets
                .iter()
                .map(|x| calculate_packet(x))
                .max()
                .unwrap(),
            OperatorType::GreaterThan => {
                assert_eq!(op.subpackets.len(), 2);
                if calculate_packet(&op.subpackets[0]) > calculate_packet(&op.subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            OperatorType::LessThan => {
                assert_eq!(op.subpackets.len(), 2);
                if calculate_packet(&op.subpackets[0]) < calculate_packet(&op.subpackets[1]) {
                    1
                } else {
                    0
                }
            }
            OperatorType::EqualTo => {
                assert_eq!(op.subpackets.len(), 2);
                if calculate_packet(&op.subpackets[0]) == calculate_packet(&op.subpackets[1]) {
                    1
                } else {
                    0
                }
            }
        },
    }
}

fn main() {
    if let Ok(lines) = read_lines("./day16/example.txt") {
        // let input = &[0xd2, 0xfe, 0x28];
        // let input = &[0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60];
        // let input = &[0x8A, 0x00, 0x4A, 0x80, 0x1A, 0x80, 0x02, 0xF4, 0x78];
        // let input = &[
        //     0x62, 0x00, 0x80, 0x00, 0x16, 0x11, 0x56, 0x2C, 0x88, 0x02, 0x11, 0x8E, 0x34,
        // ];
        // let input = &[
        //     0xC0, 0x01, 0x50, 0x00, 0x01, 0x61, 0x15, 0xA2, 0xE0, 0x80, 0x2F, 0x18, 0x23, 0x40,
        // ];
        // let input = &[
        //     0xA0, 0x01, 0x6C, 0x88, 0x01, 0x62, 0x01, 0x7C, 0x36, 0x86, 0xB1, 0x8A, 0x3D, 0x47,
        //     0x80,
        // ];
        // let input = &[0xC2, 0x00, 0xB4, 0x0A, 0x82];
        // let input = &[0x04, 0x00, 0x5A, 0xC3, 0x38, 0x90];
        // let input = &[0x88, 0x00, 0x86, 0xC3, 0xE8, 0x81, 0x12];
        // let input = &[0xCE, 0x00, 0xC4, 0x3D, 0x88, 0x11, 0x20];
        // let input = &[0xD8, 0x00, 0x5A, 0xC2, 0xA8, 0xF0];
        // let input = &[0xF6, 0x00, 0xBC, 0x2D, 0x8F];
        // let input = &[0x9C, 0x00, 0x5A, 0xC2, 0xF8, 0xF0];
        // let input = &[
        //     0x9C, 0x01, 0x41, 0x08, 0x02, 0x50, 0x32, 0x0F, 0x18, 0x02, 0x10, 0x4A, 0x08,
        // ];
        let input = &[
            0x40, 0x54, 0x46, 0x08, 0x02, 0x53, 0x2B, 0x12, 0xFE, 0xE8, 0xB1, 0x80, 0x21, 0x3B,
            0x19, 0xFA, 0x5A, 0xA7, 0x76, 0x01, 0xC0, 0x10, 0xE4, 0xEC, 0x25, 0x71, 0xA9, 0xED,
            0xFE, 0x35, 0x6C, 0x70, 0x08, 0xE7, 0xB1, 0x41, 0x89, 0x8C, 0x1F, 0x4E, 0x50, 0xDA,
            0x74, 0x38, 0xC0, 0x11, 0xD0, 0x05, 0xE4, 0xF6, 0xE7, 0x27, 0xB7, 0x38, 0xFC, 0x40,
            0x18, 0x0C, 0xB3, 0xED, 0x80, 0x23, 0x23, 0xA8, 0xC3, 0xFE, 0xD8, 0xC4, 0xE8, 0x84,
            0x42, 0x97, 0xD8, 0x8C, 0x57, 0x8C, 0x26, 0x00, 0x8E, 0x00, 0x43, 0x73, 0xBC, 0xA6,
            0xB1, 0xC1, 0xC9, 0x99, 0x45, 0x42, 0x37, 0x98, 0x02, 0x58, 0x00, 0xD0, 0xCF, 0xF7,
            0xDC, 0x19, 0x9C, 0x90, 0x94, 0xE3, 0x59, 0x80, 0x25, 0x3F, 0xB5, 0x0A, 0x00, 0xD4,
            0xC4, 0x01, 0xB8, 0x71, 0x04, 0xA0, 0xC8, 0x00, 0x21, 0x71, 0xCE, 0x31, 0xC4, 0x12,
            0x01, 0x06, 0x2C, 0x01, 0x39, 0x3A, 0xE2, 0xF5, 0xBC, 0xF7, 0xB6, 0xE9, 0x69, 0xF3,
            0xC5, 0x53, 0xF2, 0xF0, 0xA1, 0x00, 0x91, 0xF2, 0xD7, 0x19, 0xC0, 0x0C, 0xD0, 0x40,
            0x1A, 0x8F, 0xB1, 0xC6, 0x34, 0x08, 0x03, 0x30, 0x8A, 0x09, 0x47, 0xB3, 0x00, 0x56,
            0x80, 0x33, 0x61, 0x00, 0x66, 0x15, 0xC4, 0x68, 0xE4, 0x20, 0x0E, 0x47, 0xE8, 0x41,
            0x1D, 0x26, 0x69, 0x7F, 0xC3, 0xF9, 0x17, 0x40, 0x09, 0x4E, 0x16, 0x4D, 0xFA, 0x04,
            0x53, 0xF4, 0x68, 0x99, 0x01, 0x50, 0x02, 0xA6, 0xE3, 0x9F, 0x3B, 0x98, 0x02, 0xB8,
            0x00, 0xD0, 0x4A, 0x24, 0xCC, 0x76, 0x3E, 0xDB, 0xB4, 0xAF, 0xF9, 0x23, 0xA9, 0x6E,
            0xD4, 0xBD, 0xC0, 0x1F, 0x87, 0x32, 0x9F, 0xA4, 0x91, 0xE0, 0x81, 0x80, 0x25, 0x3A,
            0x4D, 0xE0, 0x08, 0x4C, 0x5B, 0x7F, 0x5B, 0x97, 0x8C, 0xC4, 0x10, 0x01, 0x2F, 0x9C,
            0xFA, 0x84, 0xC9, 0x39, 0x00, 0xA5, 0x13, 0x5B, 0xD7, 0x39, 0x83, 0x5F, 0x00, 0x54,
            0x00, 0x10, 0xF8, 0xBF, 0x1D, 0x22, 0xA0, 0x80, 0x37, 0x06, 0xE0, 0xA4, 0x7B, 0x30,
            0x09, 0xA5, 0x87, 0xE7, 0xD5, 0xE4, 0xD3, 0xA5, 0x9B, 0x4C, 0x00, 0xE9, 0x56, 0x73,
            0x00, 0xAE, 0x79, 0x1E, 0x0D, 0xCA, 0x3C, 0x4A, 0x32, 0xCD, 0xBD, 0xC4, 0x83, 0x00,
            0x56, 0x63, 0x9D, 0x57, 0xC0, 0x0D, 0x4C, 0x40, 0x1C, 0x87, 0x91, 0x16, 0x23, 0x80,
            0x02, 0x11, 0x08, 0xE2, 0x6C, 0x6D, 0x99, 0x1D, 0x10, 0x08, 0x25, 0x49, 0x21, 0x8C,
            0xDC, 0x67, 0x14, 0x79, 0xA9, 0x72, 0x33, 0xD4, 0x39, 0x93, 0xD7, 0x00, 0x56, 0x66,
            0x3F, 0xAC, 0x63, 0x0C, 0xB4, 0x4D, 0x2E, 0x38, 0x05, 0x92, 0xFB, 0x93, 0xC4, 0xF4,
            0x0C, 0xA7, 0xD1, 0xA6, 0x0F, 0xE6, 0x43, 0x48, 0x03, 0x9C, 0xE0, 0x06, 0x9E, 0x5F,
            0x56, 0x56, 0x97, 0xD5, 0x94, 0x24, 0xB9, 0x2A, 0xF2, 0x46, 0xAC, 0x06, 0x5D, 0xB0,
            0x18, 0x12, 0x80, 0x5A, 0xD9, 0x01, 0x55, 0x20, 0x04, 0xFD, 0xB8, 0x01, 0xE2, 0x00,
            0x73, 0x80, 0x16, 0x40, 0x3C, 0xC0, 0x00, 0xDD, 0x2E, 0x00, 0x53, 0x80, 0x1E, 0x60,
            0x07, 0x00, 0x09, 0x1A, 0x80, 0x1E, 0xD2, 0x00, 0x65, 0xE6, 0x00, 0x71, 0x80, 0x1A,
            0x80, 0x0A, 0xEB, 0x00, 0x15, 0x13, 0x16, 0x45, 0x00, 0x14, 0x38, 0x80, 0x10, 0xB8,
            0x61, 0x05, 0xE1, 0x39, 0x80, 0x35, 0x04, 0x23, 0xF4, 0x47, 0x20, 0x04, 0x36, 0x16,
            0x46, 0x88, 0xA4, 0x00, 0x1E, 0x04, 0x88, 0xAC, 0x90, 0xFC, 0xDF, 0x31, 0x07, 0x49,
            0x29, 0x45, 0x2E, 0x76, 0x12, 0xB1, 0x51, 0x80, 0x3A, 0x20, 0x0E, 0xC3, 0x98, 0x67,
            0x0E, 0x84, 0x01, 0xB8, 0x2D, 0x04, 0xE3, 0x18, 0x80, 0x39, 0x04, 0x63, 0x44, 0x65,
            0x20, 0x04, 0x0A, 0x44, 0xAA, 0x71, 0xC2, 0x56, 0x53, 0xB6, 0xF2, 0xFE, 0x80, 0x12,
            0x4C, 0x9F, 0xF1, 0x8E, 0xDF, 0xCA, 0x10, 0x92, 0x75, 0xA1, 0x40, 0x28, 0x9C, 0xDF,
            0x7B, 0x3A, 0xEE, 0xB0, 0xC9, 0x54, 0xF4, 0xB5, 0xFC, 0x7C, 0xD2, 0x62, 0x3E, 0x85,
            0x97, 0x26, 0xFB, 0x6E, 0x57, 0xDA, 0x49, 0x9E, 0xA7, 0x7B, 0x6B, 0x68, 0xE0, 0x40,
            0x1D, 0x99, 0x6D, 0x9C, 0x42, 0x92, 0xA8, 0x81, 0x80, 0x39, 0x26, 0xFB, 0x26, 0x23,
            0x2A, 0x13, 0x35, 0x98, 0xA1, 0x18, 0x02, 0x34, 0x00, 0xFA, 0x4A, 0xDA, 0xDD, 0x5A,
            0x97, 0xCE, 0xEC, 0x0D, 0x37, 0x69, 0x6F, 0xC0, 0xE6, 0x00, 0x9D, 0x00, 0x2A, 0x93,
            0x7B, 0x45, 0x9B, 0xDA, 0x3C, 0xC7, 0xFF, 0xD6, 0x52, 0x00, 0xF2, 0xE5, 0x31, 0x58,
            0x1A, 0xD8, 0x02, 0x30, 0x32, 0x6E, 0x11, 0xF5, 0x2D, 0xFA, 0xEA, 0xAA, 0x11, 0xDC,
            0xC0, 0x10, 0x91, 0xD8, 0xBE, 0x00, 0x39, 0xB2, 0x96, 0xAB, 0x9C, 0xE5, 0xB5, 0x76,
            0x13, 0x00, 0x53, 0x00, 0x15, 0x29, 0xBE, 0x38, 0xCD, 0xF1, 0xD2, 0x2C, 0x10, 0x05,
            0x09, 0x29, 0x8B, 0x99, 0x50, 0x02, 0x0B, 0x30, 0x9B, 0x30, 0x98, 0xC0, 0x02, 0xF4,
            0x19, 0x10, 0x02, 0x26, 0xDC,
        ];
        let (left, packet) = parse(input).unwrap();
        // let (_, packet) = parse_packet(input).unwrap();
        println!("{:?}", packet);
        println!("{:?}", left);

        println!("{:?}", sum_packet(&packet));
        println!("{:?}", calculate_packet(&packet));
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
