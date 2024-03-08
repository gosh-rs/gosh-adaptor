// [[file:../../adaptors.note::fe37d073][fe37d073]]
use crate::common::*;

use gchemol_parser::parsers::*;
// fe37d073 ends here

// [[file:../../adaptors.note::8578e6e9][8578e6e9]]
struct GaussianOut {}

/// Represents results for one frame parsed from Gaussian output.
#[derive(Debug, Default, Clone)]
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
fn orientation_line(line: &mut &str) -> PResult<()> {
    use winnow::combinator::alt;

    let input_orientation = (ws("Input orientation:"), line_ending);
    let standard_orientation = (ws("Standard orientation:"), line_ending);
    alt((input_orientation, standard_orientation))
        .parse_next(line)
        .map(|_| ())
}

fn read_positions(input: &mut &str) -> PResult<Vec<(usize, [f64; 3])>> {
    skip_line_till(orientation_line).parse_next(input)?;

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
    use winnow::combinator::alt;

    let scf_done = " SCF Done:";
    let _ = skip_line_till(scf_done).parse_next(input)?;

    let x = seq! {
        _: space1,
        _: not_space, _: " =",                // E(RB3LYP) =
        ws(double),                           // energy
        _: alt(("A.U. after", "a.u. after")), // G03 sometimes using a.u. instead of A.U.
        _: rest_line,
    }
    .context(label("energy"))
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
    let (forces,) = seq! {
        _: space1, _: digit1, _: space1, // ignore
        _: unsigned_integer,             // atomic number
        ws(xyz_array),                   // coordinates
        _: line_ending,
    }
    .parse_next(input)?;
    Ok(forces)
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
pub fn read_frame(input: &mut &str) -> PResult<Frame> {
    let to_frame = |(positions, energy, forces): (Vec<(usize, [f64; 3])>, _, _)| Frame {
        atomic_numbers: positions.iter().copied().map(|(num, pos)| num).collect(),
        positions: positions.iter().copied().map(|(num, pos)| pos).collect(),
        energy,
        forces,
    };

    let frame = (read_positions, read_energy, read_forces)
        .map(to_frame)
        .parse_next(input)?;

    // ignore the rest for Parser.parse
    let _ = rest.parse_next(input)?;

    Ok(frame)
}
// ad1147bb ends here

// [[file:../../adaptors.note::87c2a83c][87c2a83c]]
impl GaussianOutput {
    // Input orientation or Standard orientation for positions?
    fn get_orientation(&mut self) -> Result<&'static str> {
        self.reader.goto_start();

        let input_orientation = "Input orientation:";
        let standard_orientation = "Standard orientation:";
        let forces = "Center     Atomic                   Forces (Hartrees/Bohr)";
        let matching = [standard_orientation, forces];
        self.reader.seek_line(|line| matching.contains(&line.trim()))?;
        let last_line = self.reader.peek_line()?;
        // restore cursor
        self.reader.goto_start();

        let orient = if last_line.trim() == forces {
            input_orientation
        } else {
            standard_orientation
        };
        Ok(orient)
    }

    /// Parse all frames for trajectory in Gaussian output file.
    pub fn parse_frames(&mut self) -> Result<Vec<Frame>> {
        let orientation = self.get_orientation()?;
        // Skip the beginning part
        self.reader.seek_line(|line| line.trim() == orientation)?;

        let mut frames = Vec::new();
        let mut buf = String::new();
        let mut iframe = 0;
        let mut collect_frame = || {
            self.reader.read_line(&mut buf)?;
            // catch the last part, without the pattern line
            let r = self.reader.read_until(&mut buf, |line| line.trim() == orientation);
            match read_frame.parse(&buf) {
                Ok(frame) => {
                    frames.push(frame);
                    println!(
                        "Parsed frame {iframe} at position {}",
                        self.reader.get_current_position()?
                    );
                }
                Err(e) => {
                    warn!("Found parsing error for frame {iframe}: {}", e.to_string());
                    info!("Input for this frame:");
                    info!("{}", &buf);
                }
            }
            iframe += 1;
            buf.clear();
            r?;
            Ok_(())
        };
        // ignore parsing errors
        while let Ok(_) = collect_frame() {}

        Ok(frames)
    }
}
// 87c2a83c ends here

// [[file:../../adaptors.note::81d81ba0][81d81ba0]]
#[test]
fn test_gassian_out() -> Result<()> {
    let f = "tests/files/gaussian/Ala3-3-10-helix.log";
    let mut gauss_out = GaussianOutput::try_from_path(f.as_ref())?;
    let frames = gauss_out.parse_frames()?;
    assert_eq!(frames.len(), 1);

    let f = "tests/files/gaussian/g09.log";
    let mut gauss_out = GaussianOutput::try_from_path(f.as_ref())?;
    let frames = gauss_out.parse_frames()?;
    assert_eq!(frames.len(), 1);

    let f = "tests/files/gaussian/H2O_G03_zopt.log";
    let mut gauss_out = GaussianOutput::try_from_path(f.as_ref())?;
    let frames = gauss_out.parse_frames()?;
    assert_eq!(frames.len(), 14);

    Ok(())
}
// 81d81ba0 ends here
