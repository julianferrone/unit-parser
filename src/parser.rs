use std::fmt::{self, Debug, Display, Formatter};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, digit1, multispace0, multispace1},
        is_alphabetic,
    },
    combinator::{map, map_res, opt, recognize},
    error::ParseError,
    multi::{many0, many1, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult, Parser,
};

use crate::{
    ConcreteNumber, ConcreteNumberBuilder, CustomError, PhysicalQuantity, PhysicalQuantityBuilder,
};

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
    .or(Ok((input, ("dimensionless", 1isize))))
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
            "dimensionless" => pq.build(),
            // dimensionless for now—should really be an error
            _ => pq.build(),
        }
    })(input)
}

fn units(input: &str) -> IResult<&str, Vec<PhysicalQuantity>> {
    separated_list0(multispace1, unit_as_physical_quantity)(input)
}

fn combined_unit(input: &str) -> IResult<&str, PhysicalQuantity> {
    let units = units(input)?;
    let combined_unit = units
        .1
        .iter()
        .fold(PhysicalQuantityBuilder::new().build(), |acc, &pq| acc * pq);
    Ok((units.0, combined_unit))
}

pub fn get_concrete_number_as_tuple(input: &str) -> IResult<&str, (f64, PhysicalQuantity)> {
    pair(ws(double), ws(combined_unit))(input)
}

 fn concrete_number(input: &str) -> IResult<&str, ConcreteNumber> {
    map(
        get_concrete_number_as_tuple,
        |(magnitude, physical_quantity)| {
            ConcreteNumberBuilder::new()
                .magnitude(magnitude)
                .physical_quantity(physical_quantity)
                .build()
        },
    )(input)
}

#[derive(Clone)]
pub enum Expr {
    Value(ConcreteNumber),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

#[derive(Debug)]
pub enum Oper {
    Add,
    Sub,
    Mul,
    Div,
}

impl Expr {
    pub fn evaluate(self) -> Result<ConcreteNumber, CustomError> {
        match self {
            Expr::Value(concrete_number) => Ok(concrete_number),
            Expr::Add(expr_a, expr_b) => {
                let expr_a = expr_a.clone().evaluate();
                let expr_b = expr_b.clone().evaluate();
                if expr_a.is_ok() && expr_b.is_ok() {
                    expr_a.unwrap() + expr_b.unwrap()
                } else {
                    return Err(CustomError::SubExpressionError);
                }
            }
            Expr::Sub(expr_a, expr_b) => {
                let expr_a = expr_a.clone().evaluate();
                let expr_b = expr_b.clone().evaluate();
                if expr_a.is_ok() && expr_b.is_ok() {
                    expr_a.unwrap() - expr_b.unwrap()
                } else {
                    return Err(CustomError::SubExpressionError);
                }
            }
            Expr::Mul(expr_a, expr_b) => {
                let expr_a = expr_a.clone().evaluate();
                let expr_b = expr_b.clone().evaluate();
                if expr_a.is_ok() && expr_b.is_ok() {
                    Ok(expr_a.unwrap() * expr_b.unwrap())
                } else {
                    return Err(CustomError::SubExpressionError);
                }
            }
            Expr::Div(expr_a, expr_b) => {
                let expr_a = expr_a.clone().evaluate();
                let expr_b = expr_b.clone().evaluate();
                if expr_a.is_ok() && expr_b.is_ok() {
                    Ok(expr_a.unwrap() / expr_b.unwrap())
                } else {
                    return Err(CustomError::SubExpressionError);
                }
            }
            Expr::Paren(expression) => expression.clone().evaluate(),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Value(val) => write!(format, "{}", val),
            Add(ref left, ref right) => write!(format, "{} + {}", left, right),
            Sub(ref left, ref right) => write!(format, "{} - {}", left, right),
            Mul(ref left, ref right) => write!(format, "{} * {}", left, right),
            Div(ref left, ref right) => write!(format, "{} / {}", left, right),
            Paren(ref expr) => write!(format, "({})", expr),
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Value(val) => write!(format, "{}", val),
            Add(ref left, ref right) => write!(format, "({:?} + {:?})", left, right),
            Sub(ref left, ref right) => write!(format, "({:?} - {:?})", left, right),
            Mul(ref left, ref right) => write!(format, "({:?} * {:?})", left, right),
            Div(ref left, ref right) => write!(format, "({:?} / {:?})", left, right),
            Paren(ref expr) => write!(format, "[{:?}]", expr),
        }
    }
}

fn parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        delimited(tag("("), map(expr, |e| Expr::Paren(Box::new(e))), tag(")")),
        multispace0,
    )
    .parse(input)
}

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn factor(input: &str) -> IResult<&str, Expr> {
    alt((map(ws(concrete_number), Expr::Value), parens)).parse(input)
}

fn fold_exprs(initial: Expr, remainder: Vec<(Oper, Expr)>) -> Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        match oper {
            Oper::Add => Expr::Add(Box::new(acc), Box::new(expr)),
            Oper::Sub => Expr::Sub(Box::new(acc), Box::new(expr)),
            Oper::Mul => Expr::Mul(Box::new(acc), Box::new(expr)),
            Oper::Div => Expr::Div(Box::new(acc), Box::new(expr)),
        }
    })
}

fn term_mul(input: &str) -> IResult<&str, (Oper, Expr)> {
    let (input, mul) = preceded(ws(tag("*")), ws(factor)).parse(input)?;
    Ok((input, (Oper::Mul, mul)))
}

fn term_div(input: &str) -> IResult<&str, (Oper, Expr)> {
    let (input, div) = preceded(ws(tag("/")), ws(factor)).parse(input)?;
    Ok((input, (Oper::Div, div)))
}

fn term(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = factor(input)?;
    let (i, remainder) = many0(alt((term_mul, term_div))).parse(input)?;

    Ok((i, fold_exprs(initial, remainder)))
}

fn expr_add(input: &str) -> IResult<&str, (Oper, Expr)> {
    let (input, add) = preceded(ws(tag("+")), term).parse(input)?;
    Ok((input, (Oper::Add, add)))
}

fn expr_sub(input: &str) -> IResult<&str, (Oper, Expr)> {
    let (input, sub) = preceded(ws(tag("-")), term).parse(input)?;
    Ok((input, (Oper::Sub, sub)))
}

fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = term(input)?;
    let (input, remainder) = many0(alt((expr_add, expr_sub))).parse(input)?;

    Ok((input, fold_exprs(initial, remainder)))
}

pub fn evaluate_physical_equation(input: &str) -> Result<ConcreteNumber, CustomError> {
    let input = format!("({})", input);
    let (remainder, expr) = expr(input.as_str()).or_else(|e| {
        Err(CustomError::ParseError(format!(
            "ERR: Could not parse input: {e}"
        )))
    })?;
    if !remainder.is_empty() {
        return Err(CustomError::ParseError(format!(
            "ERR: Could not parse full input. Remaining input: {remainder}"
        )));
    } else {
        let result = expr.evaluate();
        result
    }
}
