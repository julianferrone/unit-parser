use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, digit1},
        is_alphabetic, is_newline, is_space,
    },
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1, separated_list0},
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

pub fn unit_as_tuples(input: &str) -> IResult<&str, (&str, isize)> {
    alt((
        separated_pair(word, char('^'), parse_isize),
        map(word, |s: &str| (s, 1isize)),
    ))(input)
}

fn units_as_physical_quantities(input: &str) -> IResult<&str, PhysicalQuantity> {
    map(unit_as_tuples, |(s, i)| match s {
        "s" => PhysicalQuantityBuilder::new().time(i).build(),
        "m" => PhysicalQuantityBuilder::new().length(i).build(),
        "kg" => PhysicalQuantityBuilder::new().mass(i).build(),
        "A" => PhysicalQuantityBuilder::new().current(i).build(),
        "K" => PhysicalQuantityBuilder::new().temperature(i).build(),
        "mol" => PhysicalQuantityBuilder::new().amount_of_substance(i).build(),
        "cd" => PhysicalQuantityBuilder::new().luminous_intensity(i).build(),
        _ => PhysicalQuantityBuilder::new().build(),
    })(input)
}

fn units(input: &str) -> IResult<&str, Vec<PhysicalQuantity>> {
    separated_list0(char(' '), units_as_physical_quantities)(input)
}

pub fn concrete_number(input: &str) -> IResult<&str, (f64, Vec<PhysicalQuantity>)> {
    separated_pair(double, many1(char(' ')), units)(input)
}

// fn concrete_number(input: &str) -> IResult<&str, ConcreteNumber> {
//     let (input, (magnitude, physical_quantity)) = (float_from_str, ).parse(input)?;
// };
