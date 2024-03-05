// [[file:../adaptors.note::273abf0b][273abf0b]]
use crate::common::*;

use winnow::error::ParserError;
use winnow::error::StrContext;
use winnow::stream::Stream;
// 273abf0b ends here

// [[file:../adaptors.note::0512156a][0512156a]]
pub use winnow::combinator::seq;
pub use winnow::combinator::{delimited, preceded, repeat, separated, terminated};
pub use winnow::Parser;
pub use winnow::prelude::*;
// 0512156a ends here

// [[file:../adaptors.note::fb1326ab][fb1326ab]]
pub fn label(s: &'static str) -> StrContext {
    StrContext::Label(s)
}

pub fn parse_error(e: winnow::error::ParseError<&str, winnow::error::ContextError>, input: &str) -> Error {
    anyhow!("found parse error:\n{:}\ninput={input:?}", e.to_string())
}

/// Match one unsigned integer: 123
pub fn unsigned_integer<'a>(input: &mut &'a str) -> PResult<usize> {
    use winnow::ascii::digit1;
    digit1
        .try_map(|x: &str| x.parse())
        .context(label("usize"))
        .parse_next(input)
}

/// Parse a f64 float number
pub fn double(input: &mut &str) -> PResult<f64> {
    use winnow::ascii::float;
    float(input)
}

/// Anything except whitespace, this parser will not consume "\n" character
pub fn not_space<'a>(input: &mut &'a str) -> PResult<&'a str> {
    winnow::token::take_till(1.., |c| " \t\r\n".contains(c))
        .context(label("not_space"))
        .parse_next(input)
}

/// Consume three float numbers separated by one or more spaces. Return xyz array.
pub fn xyz_array(s: &mut &str) -> PResult<[f64; 3]> {
    use winnow::ascii::space1;
    let x = seq! {double, _: space1, double, _: space1, double}.parse_next(s)?;
    Ok([x.0, x.1, x.2])
}

/// Read a new line including eol (\n) or consume the rest if there is no eol
/// char.
pub fn read_line<'a>(s: &mut &'a str) -> PResult<&'a str> {
    use winnow::ascii::line_ending;
    use winnow::ascii::till_line_ending;
    use winnow::combinator::opt;
    use winnow::combinator::rest;

    // use winnow::combinator::recognize;
    // if there is no newline in `s`, take the whole str
    let o = (till_line_ending, opt(line_ending)).recognize().parse_next(s)?;
    Ok(o)
}

/// Take the rest line. The line ending is not included.
pub fn rest_line<'a>(input: &mut &'a str) -> PResult<&'a str> {
    use winnow::ascii::{line_ending, till_line_ending};
    terminated(till_line_ending, line_ending)
        .context(label("rest line"))
        .parse_next(input)
}

/// Take and consuming to `literal`.
pub fn jump_to<'a>(literal: &str) -> impl FnMut(&mut &str) -> PResult<()> + '_ {
    use winnow::token::take_until;
    move |input: &mut &str| {
        let _: (&str, &str) = (take_until(0.., literal), literal)
            .context(label("jump_to"))
            .parse_next(input)?;
        Ok(())
    }
}

/// Take until found `literal`. The `literal` will not be consumed.
pub fn jump_until<'a>(literal: &str) -> impl FnMut(&mut &str) -> PResult<()> + '_ {
    use winnow::token::take_until;
    move |input: &mut &str| {
        let _: &str = take_until(0.., literal)
            .context(label("jump_until"))
            .parse_next(input)?;
        Ok(())
    }
}

/// A combinator that takes a parser `inner` and produces a parser
/// that also consumes both leading and trailing whitespace, returning
/// the output of `inner`.
pub fn ws<'a, ParseInner, Output, Error>(inner: ParseInner) -> impl Parser<&'a str, Output, Error>
where
    ParseInner: Parser<&'a str, Output, Error>,
    Error: ParserError<&'a str>,
{
    use winnow::ascii::{multispace0, space0};
    delimited(space0, inner, multispace0)
}
// fb1326ab ends here

// [[file:../adaptors.note::10e5dba2][10e5dba2]]
#[test]
fn test_ws() -> PResult<()> {
    use winnow::ascii::{digit1, line_ending, space0};

    let s = " 123 ";
    let (_, x) = ws(digit1).parse_peek(s)?;
    assert_eq!(x, "123");

    let s = "123 ";
    let (_, x) = ws(digit1).parse_peek(s)?;
    assert_eq!(x, "123");

    let s = "123\n";
    let (_, x) = ws(digit1).parse_peek(s)?;
    assert_eq!(x, "123");

    Ok(())
}

#[test]
fn test_jump_to() {
    let x = "xxbcc aa cc";
    let (r, _) = jump_to("aa").parse_peek(x).unwrap();
    assert_eq!(r, " cc");
}

#[test]
fn test_read_line() {
    let txt = "first line\nsecond line\r\nthird line\n";
    let (rest, line) = read_line.parse_peek(txt).unwrap();
    assert_eq!(line, "first line\n");
    let (rest, line) = read_line.parse_peek(rest).unwrap();
    assert_eq!(line, "second line\r\n");
    let (rest, line) = read_line.parse_peek(rest).unwrap();
    assert_eq!(line, "third line\n");
    assert_eq!(rest, "");

    // when there is no newline
    let txt = "no newline at the end";
    let (rest, line) = read_line.parse_peek(txt).unwrap();
    assert_eq!(line, txt);
    assert_eq!(rest, "");

    let txt = "no";
    let (_, line) = not_space.parse_peek(txt).unwrap();
    assert_eq!(line, "no");

    let txt = "no ";
    let (_, line) = not_space.parse_peek(txt).unwrap();
    assert_eq!(line, "no");

    let txt = "no-a\n";
    let (_, line) = not_space.parse_peek(txt).unwrap();
    assert_eq!(line, "no-a");

    let txt = "no+b\t";
    let (_, line) = not_space.parse_peek(txt).unwrap();
    assert_eq!(line, "no+b");

    let txt = " no-a\n";
    let x = not_space.parse_peek(txt);
    assert!(x.is_err());
}
// 10e5dba2 ends here
