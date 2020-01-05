// parser.rs
// :PROPERTIES:
// :header-args: :comments org :tangle src/parser.rs
// :END:

pub use nom::bytes::complete::tag;
pub use nom::bytes::complete::take_until;
pub use nom::character::complete::{alpha0, alpha1};
pub use nom::character::complete::{alphanumeric0, alphanumeric1};
pub use nom::character::complete::{digit0, digit1};
pub use nom::character::complete::{space0, space1};
pub use nom::multi::{many0, many1};
pub use nom::number::complete::double;
pub use nom::IResult;

// macros
pub use nom::do_parse;

/// Read the remaining line including eol
pub fn read_line(s: &str) -> IResult<&str, &str> {
    use nom::combinator::recognize;
    use nom::sequence::pair;

    recognize(pair(take_until("\n"), tag("\n")))(s)
}

#[test]
fn test_read_line() {
    let txt = "first line\nsecond line\r\nthird line\n";
    let (rest, line) = read_line(txt).unwrap();
    assert_eq!(line, "first line\n");
    let (rest, line) = read_line(rest).unwrap();
    assert_eq!(line, "second line\r\n");
    let (rest, line) = read_line(rest).unwrap();
    assert_eq!(line, "third line\n");
    assert_eq!(rest, "");
}

/// Match line ending preceded with zero or more whitespace chracters
pub fn eol(s: &str) -> IResult<&str, &str> {
    use nom::character::complete::line_ending;

    nom::sequence::terminated(space0, line_ending)(s)
}

/// Anything except whitespace, this parser will not consume "\n" character
pub fn not_space(s: &str) -> IResult<&str, &str> {
    use nom::bytes::complete::is_not;

    is_not(" \t\r\n")(s)
}

/// Consume three float numbers separated by one or more spaces. Return xyz array.
pub fn xyz_array(s: &str) -> IResult<&str, [f64; 3]> {
    use nom::sequence::tuple;

    let (r, (x, _, y, _, z)) = tuple((double, space1, double, space1, double))(s)?;

    Ok((r, [x, y, z]))
}
