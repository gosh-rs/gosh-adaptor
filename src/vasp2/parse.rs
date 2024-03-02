// [[file:../../adaptors.note::25511eb1][25511eb1]]
use crate::common::*;

use grep_reader::GrepReader;
// 25511eb1 ends here

// [[file:../../adaptors.note::438c735a][438c735a]]
use winnow::error::StrContext;
use winnow::prelude::*;

type Stream<'i> = &'i str;

fn double(input: &mut &str) -> PResult<f64> {
    use winnow::ascii::float;
    float(input)
}

fn rest_line<'a>(input: &mut &'a str) -> PResult<&'a str> {
    use winnow::ascii::{line_ending, till_line_ending};
    terminated(till_line_ending, line_ending).parse_next(input)
}

fn label(s: &'static str) -> StrContext {
    StrContext::Label(s)
}
// 438c735a ends here

// [[file:../../adaptors.note::d5a293f0][d5a293f0]]
//   free energy    TOTEN  =       -20.54559168 eV
fn energy_toten<'i>(input: &mut Stream<'i>) -> PResult<f64> {
    use winnow::ascii::space0;
    use winnow::combinator::delimited;
    use winnow::combinator::preceded;

    let energy = seq! {
        _: "  free energy    TOTEN  =",
        _: space0,
           double,
        _: space0,
    }
    .context(label("energy TOTEN"))
    .parse_next(input)?;
    Ok(energy.0)
}

#[test]
fn outcar_energy() -> PResult<()> {
    let s = "  free energy    TOTEN  =        -7.52548110 eV";
    let (_, v) = energy_toten.parse_peek(s)?;
    assert_eq!(v, -7.52548110);

    Ok(())
}
// d5a293f0 ends here

// [[file:../../adaptors.note::b5eb3fb1][b5eb3fb1]]
use winnow::combinator::delimited;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::terminated;

fn positions_and_forces(input: &mut &str) -> PResult<Vec<([f64; 3], [f64; 3])>> {
    use winnow::ascii::line_ending;

    let header = preceded(" POSITION ", rest_line);
    let delim = preceded(" -------", rest_line);
    let _ = (header, delim).parse_next(input)?;
    let position_and_force_line = terminated(position_and_force, line_ending);
    let values: Vec<_> = repeat(1.., position_and_force_line)
        .context(label("OUTCAR: positions and forces"))
        .parse_next(input)?;
    Ok(values)
}

fn position_and_force(input: &mut &str) -> PResult<([f64; 3], [f64; 3])> {
    use winnow::ascii::space0;
    use winnow::combinator::delimited;
    use winnow::combinator::preceded;
    use winnow::combinator::repeat;

    let values: Vec<f64> = repeat(6, preceded(space0, double)).parse_next(input)?;
    let position = values[0..3].try_into().unwrap();
    let force = values[3..].try_into().unwrap();
    Ok((position, force))
}

#[test]
fn outcar_positons_and_forces() -> PResult<()> {
    let s = " POSITION                                       TOTAL-FORCE (eV/Angst)
 -----------------------------------------------------------------------------------
      0.00000      0.00000      0.00000         0.000000      0.000000      0.000000
      0.92500      0.92500      0.92500         0.000000      0.000000      0.000000
 -----------------------------------------------------------------------------------
    total drift:                                0.000000      0.000000      0.000000
";

    let (_, v) = positions_and_forces.parse_peek(s)?;
    dbg!(v);

    Ok(())
}
// b5eb3fb1 ends here

// [[file:../../adaptors.note::cf96d53e][cf96d53e]]
use winnow::combinator::seq;

fn position_and_force_(input: &mut &str) -> PResult<[f64; 6]> {
    use winnow::ascii::space0;
    use winnow::combinator::repeat;

    repeat(6, preceded(space0, double))
        .map(|x: Vec<f64>| x.try_into().unwrap())
        .parse_next(input)
}

fn positions_and_forces_(input: &mut &str) -> PResult<Vec<[f64; 6]>> {
    use winnow::ascii::line_ending;

    let values = seq! {
        _: preceded(" POSITION ", rest_line),
        _: preceded(" -------", rest_line),
           repeat(1.., terminated(position_and_force_, line_ending)),
    }
    .parse_next(input)?;
    Ok(values.0)
}

#[test]
fn test_vasp() -> Result<()> {
    let n_ions_part_pattern = "umber of ions     NIONS =";
    let energy_part_pattern = "FREE ENERGIE OF THE ION-ELECTRON SYSTEM";
    let forces_part_pattern = "POSITION                                       TOTAL-FORCE";

    let path = "./tests/files/vasp/OUTCAR-5.3.5";
    let mut reader = GrepReader::try_from_path(path.as_ref())?;
    let pattern = format!("{n_ions_part_pattern}|{energy_part_pattern}|{forces_part_pattern}");
    let n = reader.mark(&pattern, None)?;

    ensure!(n >= 2);
    let mut s = String::new();
    reader.goto_marker(0);
    reader.read_lines(1, &mut s)?;
    if s.contains(n_ions_part_pattern) {}
    if s.contains(energy_part_pattern) {
        //
    } else if s.contains(forces_part_pattern) {
        //
    } else {
        unreachable!()
    }

    dbg!(s);
    dbg!(n);

    Ok(())
}
// cf96d53e ends here
