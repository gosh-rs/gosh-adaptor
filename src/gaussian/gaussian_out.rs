// [[file:../../adaptors.note::fe37d073][fe37d073]]
use crate::common::*;

use gchemol_parser::parsers::*;
// fe37d073 ends here

// [[file:../../adaptors.note::8578e6e9][8578e6e9]]
struct GaussianOut {

}

/// Represents results for one frame parsed from Gaussian output.
pub struct Frame {
    pub atomic_numbers: Vec<usize>,
    pub energy: f64,
    pub positions: Vec<[f64; 3]>,
    pub forces: Vec<[f64; 3]>,
}
// 8578e6e9 ends here

// [[file:../../adaptors.note::44f7d6ac][44f7d6ac]]
use gchemol_parser::TextReader;
use std::fs::File;
use std::io::BufReader;


/// Represents data read from Gaussian output file.
pub struct GaussianOutput {
    reader: TextReader<BufReader<File>>,
}

impl GaussianOutput {
    pub fn try_from_path(f: &Path) -> Result<Self> {
        let reader = TextReader::try_from_path(f)?;
        Ok(Self { reader })
    }
}
// 44f7d6ac ends here

// [[file:../../adaptors.note::86278e23][86278e23]]
fn read_positions(input: &mut &str) -> PResult<Vec<(usize, [f64; 3])>> {
    use winnow::combinator::alt;
    use winnow::combinator::not;

    let input_orientation = (ws("Input orientation:"), line_ending);
    let standard_orientation = (ws("Standard orientation:"), line_ending);
    let orientation = alt((input_orientation, standard_orientation));
    skip_line_till(orientation).parse_next(input)?;

    let x = seq! {
        _: " ---------", _: rest_line,
        _: " Center", _: rest_line,
        _: " Number", _: rest_line,
        _: " ---------", _: rest_line,
        repeat(1.., position_line),
    }
    .parse_next(input)?;

    Ok(x.0)
}

//       3          1           0       -3.124042    0.673175   -0.828087
fn position_line(input: &mut &str) -> PResult<(usize, [f64; 3])> {
    assert!(!input.is_empty());
    let sym_and_pos = seq! {
        _: space1, _: digit1, _: space1, // ignore
        unsigned_integer,                // atomic number
        _: space1, _: digit1,            // ignore atomic type
        ws(xyz_array),                   // coordinates
        _: line_ending,
    }
    .parse_next(input)?;
    Ok(sym_and_pos)
}

#[test]
fn test_gaussian_position() -> Result<()> {
    let input = " Leave Link  103 at Fri Apr 19 13:58:11 2019, MaxMem=    33554432 cpu:         0.0
 (Enter /home/ybyygu/gaussian/g09/l202.exe)
                          Input orientation:
 ---------------------------------------------------------------------
 Center     Atomic      Atomic             Coordinates (Angstroms)
 Number     Number       Type             X           Y           Z
 ---------------------------------------------------------------------
      1          6           0       -3.480714    0.168776    0.045565
      2          1           0       -3.124060   -0.840034    0.045565
      3          1           0       -3.124042    0.673175   -0.828087
      4          1           0       -4.550714    0.168790    0.045565
      5          6           0       -2.967372    0.894733    1.302970
      6          1           0       -3.325641    0.391464    2.176620
";

    let items = read_positions.parse(input).unwrap();
    assert_eq!(items.len(), 6);

    Ok(())
}
// 86278e23 ends here

// [[file:../../adaptors.note::bb3681aa][bb3681aa]]
//  SCF Done:  E(RB3LYP) =  -117.726685588     A.U. after   10 cycles
fn read_energy(input: &mut &str) -> PResult<f64> {
    let scf_done = " SCF Done:";
    let _ = skip_line_till(scf_done).parse_next(input)?;

    let x = seq! {
        _: space1,
        _: not_space, _: " =",            // E(RB3LYP) =
        ws(double),                       // energy
        _: "A.U. after",
        _: rest_line,
    }
    .parse_next(input)?;

    Ok(x.0)
}

#[test]
fn test_gaussian_energy() -> Result<()> {
    let line = " RMSDP=2.91D-09 MaxDP=2.04D-08 DE= 1.42D-13 OVMax= 0.00D+00

 SCF Done:  E(RB3LYP) =  -117.726685588     A.U. after   10 cycles
            NFock= 10  Conv=0.29D-08     -V/T= 2.0158
";
    let (_, energy) = read_energy.parse_peek(line).unwrap();
    assert_eq!(energy, -117.726685588);

    Ok(())
}
// bb3681aa ends here

// [[file:../../adaptors.note::46e080e9][46e080e9]]
fn read_forces(input: &mut &str) -> PResult<Vec<[f64; 3]>> {
    let label = " Center     Atomic                   Forces (Hartrees/Bohr)";
    let _ = skip_line_till(terminated(label, line_ending)).parse_next(input)?;

    let x = seq! {
        _: " Number     Number              X              Y              Z", _: line_ending,
        _: " -------------------------------------------------------------------", _: line_ending,
        repeat(1.., forces_line),
    }
    .parse_next(input)?;

    Ok(x.0)
}

//     4          6           0.006479521   -0.000810488    0.002127718
fn forces_line(input: &mut &str) -> PResult<[f64; 3]> {
    let ([x, y, z],) = seq! {
        _: space1, _: digit1, _: space1, // ignore
        _: unsigned_integer,             // atomic number
        ws(xyz_array),                   // coordinates
        _: line_ending,
    }
    .parse_next(input)?;
    Ok([-x, -y, -z])
}

#[test]
fn test_forces() {
    let input = "
 Dipole        = 2.14981920D-04-5.03313123D-05-7.03866449D-04
 -------------------------------------------------------------------
 Center     Atomic                   Forces (Hartrees/Bohr)
 Number     Number              X              Y              Z
 -------------------------------------------------------------------
    1          6           0.004041519   -0.005232655    0.002167205
    2          6          -0.004044116   -0.005258824    0.002176484
    3          6           0.002582590    0.006053054    0.002164962
    4          6           0.006479521   -0.000810488    0.002127718
    5          6          -0.006479199   -0.000813615    0.002127130
    6          6          -0.002560082    0.006039539    0.002154972
    7          6          -0.006418727    0.000777238   -0.002445657
    8          6          -0.003904203    0.005233739   -0.002445106
    9          6           0.002567807   -0.005996818   -0.002364696
   10          6          -0.002591686   -0.005997044   -0.002368035
 -------------------------------------------------------------------
 Cartesian Forces:  Max     0.006995940 RMS     0.003848089
";
    let (_, forces) = read_forces.parse_peek(input).unwrap();
    assert_eq!(10, forces.len());
}
// 46e080e9 ends here

// [[file:../../adaptors.note::ad1147bb][ad1147bb]]
fn read_frames(input: &mut &str) -> PResult<Vec<Frame>> {
    let to_frame = |(positions, energy, forces): (Vec<(usize, [f64; 3])>, _, _)| Frame {
        atomic_numbers: positions.iter().copied().map(|(num, pos)| num).collect(),
        positions: positions.iter().copied().map(|(num, pos)| pos).collect(),
        energy,
        forces,
    };

    let mut read_frame = (read_positions, read_energy, read_forces).map(to_frame);
    let frames: Vec<_> = repeat(0.., read_frame).context(label("xx")).parse_next(input)?;
    // ignore the rest for Parser.parse
    let _ = rest.parse_next(input)?;

    Ok(frames)
}

pub fn parse_frames(f: &Path) -> Result<Vec<Frame>> {
    let s = crate::gut::fs::read_file(f)?;
    let frames = read_frames.parse(&s).map_err(|e| parse_error(e, &s))?;
    Ok(frames)
}
// ad1147bb ends here

// [[file:../../adaptors.note::81d81ba0][81d81ba0]]
#[test]
fn test_gassian_out() -> Result<()> {
    let f = "./tests/files/gaussian/H2O_G03_zopt.log";
    let frames = parse_frames(f.as_ref())?;
    assert_eq!(frames.len(), 14);

    Ok(())
}
// 81d81ba0 ends here
