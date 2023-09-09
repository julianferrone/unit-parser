use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, digit1},
        is_alphabetic, is_newline, is_space,
    },
    combinator::{map, map_res, opt, recognize},
    multi::{fold_many0, many0, many1, separated_list0},
    number::complete::{double, float},
    sequence::{pair, preceded, separated_pair, tuple},
    IResult,
};

use crate::{PhysicalQuantity, PhysicalQuantityBuilder};

// fn nonws_char(c: char) -> bool {
//     !is_space(c as u8) && !is_newline(c as u8)
// }

fn alphabet_char(c: char) -> bool {
    is_alphabetic(c as u8)
}

pub fn word(input: &str) -> IResult<&str, &str> {
    take_while1(alphabet_char)(input)
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        isize::from_str_radix(s, 10)
    })(input)?;

    Ok((i, number))
}

pub fn unit_as_tuple(input: &str) -> IResult<&str, (&str, isize)> {
    alt((
        separated_pair(word, char('^'), parse_isize),
        map(word, |s: &str| (s, 1isize)),
    ))(input)
}

fn unit_as_physical_quantity(input: &str) -> IResult<&str, PhysicalQuantity> {
    map(unit_as_tuple, |(s, i)| {
        let pq = PhysicalQuantityBuilder::new();
        match s {
            "s" => pq.time(i).build(),
            "m" => pq.length(i).build(),
            "kg" => pq.mass(i).build(),
            "A" => pq.current(i).build(),
            "K" => pq.temperature(i).build(),
            "mol" => pq.amount_of_substance(i).build(),
            "cd" => pq.luminous_intensity(i).build(),
            "Hz" => pq.time(i * -1).build(),
            "N" => pq.time(i * -2).length(i).mass(i).build(),
            "Pa" => pq.time(i * -2).length(-1 * i).mass(i).build(),
            "J" => pq.time(i * -2).length(2 * i).mass(i).build(),
            "W" => pq.time(i * -3).length(2 * i).mass(i).build(),
            "C" => pq.time(i).current(i).build(),
            "V" => pq
                .time(i * -3)
                .length(i * 2)
                .mass(i)
                .current(i * -1)
                .build(),
            "Wb" => pq
                .time(i * -2)
                .length(i * 2)
                .mass(i)
                .current(i * -1)
                .build(),
            "T" => pq.time(i * -2).mass(i).current(i * -1).build(),
            "F" => pq
                .time(i * 4)
                .length(i * -2)
                .mass(i * -1)
                .current(i * 2)
                .build(),
            "ohm" | "Ω" => pq
                .time(i * -3)
                .length(i * 2)
                .mass(i)
                .current(i * -2)
                .build(),
            "S" => pq
                .time(i * 3)
                .length(i * -2)
                .mass(i * -1)
                .current(i * 2)
                .build(),
            "H" => pq
                .time(i * -2)
                .length(i * 2)
                .mass(i)
                .current(i * -2)
                .build(),
            "kat" => pq.time(i * -1).amount_of_substance(i).build(),
            // dimensionless for now—should really be an error
            _ => pq.build(),
        }
    })(input)
}

fn units(input: &str) -> IResult<&str, Vec<PhysicalQuantity>> {
    separated_list0(char(' '), unit_as_physical_quantity)(input)
}

fn combined_unit(input: &str) -> IResult<&str, PhysicalQuantity> {
    let units = units(input)?;
    let combined_unit = units
        .1
        .iter()
        .fold(PhysicalQuantityBuilder::new().build(), |acc, &pq| acc * pq);
    Ok((units.0, combined_unit))
}

pub fn concrete_number(input: &str) -> IResult<&str, (f64, PhysicalQuantity)> {
    separated_pair(double, many1(char(' ')), combined_unit)(input)
}

// fn concrete_number(input: &str) -> IResult<&str, ConcreteNumber> {
//     let (input, (magnitude, physical_quantity)) = (float_from_str, ).parse(input)?;
// };
