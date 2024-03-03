// [[file:../../adaptors.note::25511eb1][25511eb1]]
use crate::common::*;
use grep_reader::GrepReader;
use std::path::Path;

use crate::parsers::*;
// 25511eb1 ends here

// [[file:../../adaptors.note::c35d320f][c35d320f]]
#[derive(Debug, Clone, Default)]
struct Frame {
    energy: f64,
    positions: Vec<[f64; 3]>,
    forces: Vec<[f64; 3]>,
}
// c35d320f ends here

// [[file:../../adaptors.note::50721456][50721456]]
fn number_of_ions(input: &mut &str) -> PResult<usize> {
    use winnow::ascii::{digit1, line_ending, space0};

    let label = "number of ions     NIONS =";
    let skip = jump_to(label);
    let nions = ws(digit1).try_map(|s: &str| s.parse::<usize>());
    preceded(skip, nions).parse_next(input)
}
// 50721456 ends here

// [[file:../../adaptors.note::d5a293f0][d5a293f0]]
//   free energy    TOTEN  =       -20.54559168 eV
fn energy_toten(input: &mut &str) -> PResult<f64> {
    use winnow::ascii::{line_ending, space0};

    let energy = seq! {
        _: "  FREE ENERGIE OF THE ION-ELECTRON SYSTEM",
        _: rest_line,
        _: "  ---------------",
        _: rest_line,
        _: "  free  energy   TOTEN  =",
        _: space0,
           double,
        _: space0,
        _: "eV",
        _: line_ending,
        // ignore other two lines
        _: read_line,
        _: read_line
    }
    .context(label("energy TOTEN"))
    .parse_next(input)?;
    Ok(energy.0)
}

#[test]
fn outcar_energy() -> PResult<()> {
    let s = "  FREE ENERGIE OF THE ION-ELECTRON SYSTEM (eV)
  ---------------------------------------------------
  free  energy   TOTEN  =       -20.028155 eV

  energy  without entropy=      -20.028155  energy(sigma->0) =      -20.028155
";
    let (_, v) = energy_toten.parse_peek(s)?;
    assert_eq!(v, -20.028155);

    Ok(())
}
// d5a293f0 ends here

// [[file:../../adaptors.note::b5eb3fb1][b5eb3fb1]]
fn position_and_force(input: &mut &str) -> PResult<[f64; 6]> {
    use winnow::ascii::space1;

    repeat(6, preceded(space1, double))
        .map(|x: Vec<f64>| x.try_into().unwrap())
        .parse_next(input)
}

fn positions_and_forces(input: &mut &str) -> PResult<Vec<[f64; 6]>> {
    use winnow::ascii::line_ending;

    let values = seq! {
        _: preceded(" POSITION ", rest_line),
        _: preceded(" -------", rest_line),
           repeat(1.., terminated(position_and_force, line_ending)),
        _: preceded(" -------", rest_line),
    }
    .context(label("OUTCAR positions and forces"))
    .parse_next(input)?;
    Ok(values.0)
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
    assert_eq!(v.len(), 2);

    Ok(())
}
// b5eb3fb1 ends here

// [[file:../../adaptors.note::cf96d53e][cf96d53e]]
// For old VASP below 5.2.11
fn parse_frames_old(input: &mut &str) -> PResult<Vec<Frame>> {
    let frame = (positions_and_forces, energy_toten);

    let frames_data: Vec<_> = repeat(1.., frame).context(label("OUTCAR frames")).parse_next(input)?;
    let mut frames = vec![];
    for (p_and_f, energy) in frames_data {
        let mut frame = Frame::default();
        frame.energy = energy;
        frame.positions = p_and_f.iter().map(|x| x[..3].try_into().unwrap()).collect();
        frame.forces = p_and_f.iter().map(|x| x[3..6].try_into().unwrap()).collect();
        frames.push(frame);
    }
    Ok(frames)
}

// For VASP above 5.2.11
fn parse_frames(input: &mut &str) -> PResult<Vec<Frame>> {
    let frame = (energy_toten, positions_and_forces);

    let frames_data: Vec<_> = repeat(1.., frame).context(label("OUTCAR frames")).parse_next(input)?;
    let mut frames = vec![];
    for (energy, p_and_f) in frames_data {
        let mut frame = Frame::default();
        frame.energy = energy;
        frame.positions = p_and_f.iter().map(|x| x[0..3].try_into().unwrap()).collect();
        frame.forces = p_and_f.iter().map(|x| x[3..6].try_into().unwrap()).collect();
        frames.push(frame);
    }
    Ok(frames)
}

/// Parse `Frame` data from OUTCAR in `f`
pub fn parse_from(f: &Path) -> Result<Vec<Frame>> {
    let n_ions_part_pattern = "number of ions     NIONS =";
    let energy_part_pattern = "FREE ENERGIE OF THE ION-ELECTRON SYSTEM";
    let forces_part_pattern = "POSITION                                       TOTAL-FORCE";

    let mut reader = GrepReader::try_from_path(f.as_ref())?;
    let pattern = format!("{n_ions_part_pattern}|{energy_part_pattern}|{forces_part_pattern}");
    let n = reader.mark(&pattern, None)?;

    ensure!(n >= 2);
    let mut s = String::new();
    reader.goto_marker(0);
    reader.read_lines(1, &mut s)?;
    let natoms: usize = number_of_ions.parse(&mut &s[..]).map_err(|e| parse_error(e, &s))?;
    s.clear();

    reader.goto_next_marker();
    reader.read_lines(1, &mut s)?;
    let mut collect_frames = || {
        let last_line = s.lines().last()?;
        if last_line.contains(energy_part_pattern) {
            reader.read_lines(4, &mut s).ok()?;
            Some(1)
        } else if last_line.contains(forces_part_pattern) {
            reader.read_lines(natoms + 2, &mut s).ok()?;
            Some(2)
        } else {
            reader.goto_next_marker().ok()?;
            reader.read_lines(1, &mut s).ok()?;
            Some(0)
        }
    };
    // collect all frames
    while let Some(_) = collect_frames() {}
    // println!("{s}");
    let mut parse_frames = if s.starts_with(" POSITION") {
        parse_frames_old
    } else {
        parse_frames
    };
    let frames = parse_frames.parse(&mut &s[..]).map_err(|e| parse_error(e, &s[..]))?;

    Ok(frames)
}
// cf96d53e ends here

// [[file:../../adaptors.note::600fda9b][600fda9b]]
#[test]
fn test_vasp() -> Result<()> {
    let f = "./tests/files/vasp/OUTCAR-5.3.5";
    let frames = parse_from(f.as_ref())?;
    assert_eq!(frames.len(), 1);

    // test files from Jmol
    let f = "./tests/files/vasp/OUTCAR-5.2";
    let frames = parse_from(f.as_ref())?;
    assert_eq!(frames.len(), 1);

    // test files from Jmol
    let f = "./tests/files/vasp/OUTCAR_diamond.dat";
    let frames = parse_from(f.as_ref()).unwrap();
    assert_eq!(frames.len(), 1);

    // test files from Jmol
    let f = "tests/files/vasp/AlH3_Vasp5.dat";
    let frames = parse_from(f.as_ref())?;
    assert_eq!(frames.len(), 7);

    Ok(())
}
// 600fda9b ends here
