use nom::{
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, digit1},
        is_alphabetic, is_newline, is_space,
    },
    combinator::{map, recognize, map_res, opt},
    multi::{many0, many1, separated_list0},
    number::complete::{double, float},
    sequence::{pair, separated_pair, tuple, preceded},
    IResult,
};

fn nonws_char(c: char) -> bool {
    !is_space(c as u8) && !is_newline(c as u8)
}

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

pub fn unit(input: &str) -> IResult<&str, (&str, isize)> {
    separated_pair(word, char('^'), parse_isize)(input)
}

fn units(input: &str) -> IResult<&str, Vec<(&str, isize)>> {
    separated_list0(char(' '), unit)(input)
}

pub fn concrete_number(input: &str) -> IResult<&str, (f64, Vec<(&str, isize)>)> {
    separated_pair(double, many1(char(' ')), units)(input)
}

// fn concrete_number(input: &str) -> IResult<&str, ConcreteNumber> {
//     let (input, (magnitude, physical_quantity)) = (float_from_str, ).parse(input)?;
// };
