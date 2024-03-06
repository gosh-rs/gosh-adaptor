// [[file:../../adaptors.note::25511eb1][25511eb1]]
use crate::common::*;

use grep_reader::GrepReader;
use std::path::Path;

use gchemol_parser::parsers::*;
// 25511eb1 ends here

// [[file:../../adaptors.note::c35d320f][c35d320f]]
#[derive(Debug, Clone, Default, Serialize)]
pub struct Frame {
    pub symbols: Vec<String>,
    pub energy: f64,
    pub positions: Vec<[f64; 3]>,
    pub forces: Vec<[f64; 3]>,
    pub lattice: [[f64; 3]; 3],
    pub stress: Option<[f64; 6]>,
}
// c35d320f ends here

// [[file:../../adaptors.note::6f1bf8e7][6f1bf8e7]]
use gchemol_parser::TextReader;
use std::fs::File;
use std::io::BufReader;

pub struct Outcar {
    reader: TextReader<BufReader<File>>,
}

impl Outcar {
    pub fn try_from_path(f: &Path) -> Result<Self> {
        let reader = TextReader::try_from_path(f)?;
        Ok(Self { reader })
    }
}
// 6f1bf8e7 ends here

// [[file:../../adaptors.note::3713b178][3713b178]]
// O_s or Ca_pv or V
fn element_symbol_in_potcar<'a>(input: &mut &'a str) -> PResult<&'a str> {
    use winnow::ascii::alpha1;
    use winnow::combinator::opt;
    terminated(alpha1, opt(("_", alpha1))).parse_next(input)
}

fn atom_type<'a>(input: &mut &'a str) -> PResult<&'a str> {
    use winnow::ascii::{space0, space1};
    use winnow::combinator::alt;

    let sym = seq! {
        // "POTCAR:" or "TITEL ="
        _: space1, _: alt(("POTCAR:", "TITEL  =")), _: space0,
        // PAW_GGA
        _: not_space, _: space1,
        // Sr_sv
        element_symbol_in_potcar,
        _: rest_line,
    }
    .context(label("OUTCAR atom type"))
    .parse_next(input)?;

    Ok(sym.0)
}

pub(self) fn atom_types<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    repeat(1.., atom_type).parse_next(input)
}

#[test]
fn outcar_atom_type() {
    let line = "   TITEL  = PAW H 06May1998                                                     \n";
    let (_rest, sym) = atom_type.parse_peek(line).unwrap();
    assert_eq!(sym, "H");

    let line = " POTCAR:   PAW_PBE K_sv 06Sep2000                 \n";
    let (_, sym) = atom_type.parse_peek(line).unwrap();
    assert_eq!(sym, "K");
}
// 3713b178 ends here

// [[file:../../adaptors.note::01a764cc][01a764cc]]
// ions per type =             192  95   1   1
pub(self) fn num_ions_per_type<'a>(input: &mut &'a str) -> PResult<Vec<usize>> {
    use winnow::ascii::{space0, space1, line_ending};
    use winnow::combinator::alt;

    let sym = seq! {
        _: space1, _: "ions per type =", _: space0,
        // 192  95   1   1
        repeat(1.., preceded(space0, unsigned_integer)), _: space0,
        _: line_ending,
    }
    .context(label("OUTCAR ions per type"))
    .parse_next(input)?;

    Ok(sym.0)
}

#[test]
fn outcar_num_ions_per_type() {
    let line = "   ions per type =             192  95   1   1\n";
    let (_, nums) = num_ions_per_type.parse_peek(line).unwrap();
    assert_eq!(nums.len(), 4);
}
// 01a764cc ends here

// [[file:../../adaptors.note::a5341d1a][a5341d1a]]
pub(self) fn get_atom_types_from(f: &Path) -> Result<Vec<String>> {
    use gchemol_parser::TextReader;

    let mut reader = TextReader::try_from_path(f)?;
    let match_collect =
        |line: &str| line.contains("TITEL  =") || line.contains("POTCAR:") || line.contains("ions per type =");
    let match_stop = |line: &str| line.contains("ions per type =");

    let mut collected = String::new();
    for line in reader.lines() {
        if match_collect(&line) {
            collected.push_str(&line);
            collected.push_str("\n");
        }
        if match_stop(&line) {
            break;
        }
    }

    let (types, nums) = (atom_types, num_ions_per_type)
        .parse(&collected)
        .map_err(|e| parse_error(e, &collected))?;

    let symbols: Vec<_> = types
        .into_iter()
        .zip(nums)
        .flat_map(|(s, n)| std::iter::repeat(s.to_owned()).take(n))
        .collect();
    Ok(symbols)
}

#[test]
fn outcar_atom_types() -> Result<()> {
    let f = "./tests/files/vasp/OUTCAR-5.3.5";
    let symbols = get_atom_types_from(f.as_ref())?;
    assert_eq!(symbols.len(), 289);

    let f = "tests/files/vasp/OUTCAR_diamond.dat";
    let symbols = get_atom_types_from(f.as_ref())?;
    assert_eq!(symbols.len(), 2);

    let f = "tests/files/vasp/OUTCAR-5.2";
    let symbols = get_atom_types_from(f.as_ref())?;
    assert_eq!(symbols.len(), 8);

    let f = "tests/files/vasp/AlH3_Vasp5.dat";
    let symbols = get_atom_types_from(f.as_ref())?;
    assert_eq!(symbols.len(), 8);

    Ok(())
}
// a5341d1a ends here

// [[file:../../adaptors.note::5843bea2][5843bea2]]
impl Outcar {
    pub(self) fn parse_atom_types(&mut self) -> Result<Vec<String>> {
        use winnow::token::take_until;

        let potcar_or_titel = |line: &str| line.contains("TITEL  =") || line.contains("POTCAR:");
        self.reader.seek_line(potcar_or_titel)?;

        let mut buf = String::new();
        let ions_per_type = |line: &str| line.contains(" ions per type =");
        self.reader.read_until(&mut buf, ions_per_type)?;
        self.reader.read_line(&mut buf)?;

        let (types, nums) = seq! {
            atom_types,
            _: jump_until("   ions per type ="),
            num_ions_per_type
        }
        .parse(&buf)
        .map_err(|e| parse_error(e, &buf))?;

        let symbols: Vec<_> = types
            .into_iter()
            .zip(nums)
            .flat_map(|(s, n)| std::iter::repeat(s.to_owned()).take(n))
            .collect();
        Ok(symbols)
    }
}

#[test]
fn outcar_atom_types_new() -> Result<()> {
    let mut outcar = Outcar::try_from_path("./tests/files/vasp/OUTCAR-5.3.5".as_ref())?;
    let symbols = outcar.parse_atom_types().unwrap();
    assert_eq!(symbols.len(), 289);

    let mut outcar = Outcar::try_from_path("tests/files/vasp/OUTCAR_diamond.dat".as_ref())?;
    let symbols = outcar.parse_atom_types().unwrap();
    assert_eq!(symbols.len(), 2);

    let mut outcar = Outcar::try_from_path("tests/files/vasp/OUTCAR-5.2".as_ref())?;
    let symbols = outcar.parse_atom_types().unwrap();
    assert_eq!(symbols.len(), 8);

    let mut outcar = Outcar::try_from_path("tests/files/vasp/AlH3_Vasp5.dat".as_ref())?;
    let symbols = outcar.parse_atom_types().unwrap();
    assert_eq!(symbols.len(), 8);

    Ok(())
}
// 5843bea2 ends here

// [[file:../../adaptors.note::d5a293f0][d5a293f0]]
//   free energy    TOTEN  =       -20.54559168 eV
fn energy_toten(input: &mut &str) -> PResult<f64> {
    use winnow::ascii::{line_ending, space0};

    let energy = seq! {
        _: "  FREE ENERGIE OF THE ION-ELECTRON SYSTEM (eV)", _: line_ending,
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

// [[file:../../adaptors.note::cfcdcec2][cfcdcec2]]
pub(self) fn stress<'a>(input: &mut &'a str) -> PResult<[f64; 6]> {
    use winnow::ascii::line_ending;

    let x = seq! {
        _: "  FORCE on cell =-STRESS in cart. coord.  units (eV)",
        _: jump_to("in kB"),
        //   in kB      -0.65036    -0.79073    -0.79508    -0.01450     0.00000     0.00000
        stress_values,
    }
    .context(label("OUTCAR stress"))
    .parse_next(input)?;
    Ok(x.0[..6].try_into().unwrap())
}

fn stress_values(input: &mut &str) -> PResult<Vec<f64>> {
    use winnow::ascii::space1;
    repeat(6, preceded(space1, double)).parse_next(input)
}
// cfcdcec2 ends here

// [[file:../../adaptors.note::ee469f9d][ee469f9d]]
fn lattice_vectors<'a>(input: &mut &'a str) -> PResult<[[f64; 3]; 3]> {
    use winnow::ascii::space1;
    let vectors = seq! {
        _: " VOLUME and BASIS-vectors are now :", _: rest_line,
        // -----------------------------------------------------------------------------
        _: rest_line,
        //   energy-cutoff  :     1000.00
        _: rest_line,
        //   volume of cell :      120.31
        _: rest_line,
        // direct lattice vectors                 reciprocal lattice vectors
        _: space1, _: "direct lattice vectors", _: space1, _: "reciprocal lattice vectors", _: rest_line,
        _: space1, xyz_array, _: rest_line,
        _: space1, xyz_array, _: rest_line,
        _: space1, xyz_array, _: rest_line,
    }
    .context(label("OUTCAR direct lattice vectors"))
    .parse_next(input)?;

    Ok([vectors.0, vectors.1, vectors.2])
}
// ee469f9d ends here

// [[file:../../adaptors.note::b5eb3fb1][b5eb3fb1]]
fn position_and_force(input: &mut &str) -> PResult<[f64; 6]> {
    use winnow::ascii::space1;

    repeat(6, preceded(space1, double))
        .map(|x: Vec<f64>| x.try_into().unwrap())
        .parse_next(input)
}

fn positions_and_forces(input: &mut &str) -> PResult<Vec<[f64; 6]>> {
    use winnow::ascii::{line_ending, space1};

    let values = seq! {
        _: " POSITION", _: space1, _: "TOTAL-FORCE (eV/Angst)", _: line_ending,
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

// [[file:../../adaptors.note::639df118][639df118]]
impl Outcar {
    pub(self) fn collect_frame_text(&mut self, natoms: usize) -> Result<String> {
        let energy_part_pattern = "  FREE ENERGIE OF THE ION-ELECTRON SYSTEM";
        let forces_part_pattern = " POSITION                                       TOTAL-FORCE";
        let energy_or_forces = |line: &str| line.contains(energy_part_pattern) || line.contains(forces_part_pattern);

        let mut buf = String::new();
        self.reader.read_until(&mut buf, energy_or_forces)?;
        let n = self.reader.read_line(&mut buf)?;
        let m = buf.len() - n;
        let last_line = &buf[m..];
        let old_vasp = last_line.contains(energy_part_pattern);
        // For VASP older than 5.2.11, the energy part is before the forces part
        if old_vasp {
            // remove the energy part, and append after the forces part
            let mut energy_part: String = buf.drain(m..).collect();
            for _ in 0..3 {
                self.reader.read_line(&mut energy_part)?;
            }
            // read to including the forces part
            self.reader
                .read_until(&mut buf, |line| line.contains(forces_part_pattern))?;
            for _ in 0..natoms + 3 {
                self.reader.read_line(&mut buf)?;
            }
            buf.push_str(&energy_part);
        } else {
            // read in the forces part
            self.reader
                .read_until(&mut buf, |line| line.contains(energy_part_pattern))?;
            for _ in 0..3 {
                self.reader.read_line(&mut buf)?;
            }
        }
        Ok(buf)
    }
}
// 639df118 ends here

// [[file:../../adaptors.note::608566c9][608566c9]]
fn parse_frame(frame_text: &mut &str) -> PResult<Frame> {
    use winnow::combinator::opt;

    let stress_part_pattern = "  FORCE on cell =-STRESS in cart. coord.  units (eV):";
    let has_stress_part = frame_text.contains(stress_part_pattern);
    let lattice_part_pattern = " VOLUME and BASIS-vectors are now :";
    let energy_part_pattern = "  FREE ENERGIE OF THE ION-ELECTRON SYSTEM (eV)";
    let forces_part_pattern = " POSITION                                       TOTAL-FORCE (eV/Angst)";

    let mut frame = Frame::default();
    if has_stress_part {
        jump_until(stress_part_pattern)
            .context(label("stress part pattern"))
            .parse_next(frame_text)?;
        frame.stress = stress.parse_next(frame_text).ok();
    }

    // Parse lattice vectors
    jump_until(lattice_part_pattern).parse_next(frame_text)?;
    frame.lattice = lattice_vectors.parse_next(frame_text)?;

    // Parse positions and forces
    jump_until(forces_part_pattern).parse_next(frame_text)?;
    let p_and_f = positions_and_forces.parse_next(frame_text)?;
    frame.positions = p_and_f.iter().map(|x| x[..3].try_into().unwrap()).collect();
    frame.forces = p_and_f.iter().map(|x| x[3..6].try_into().unwrap()).collect();

    // Parse total energy
    jump_until(energy_part_pattern).parse_next(frame_text)?;
    frame.energy = energy_toten.parse_next(frame_text)?;

    Ok(frame)
}

impl Outcar {
    /// Parse all frames data including energy, positions, forces,
    /// symbols, lattice, stress, ...
    pub fn parse_frames(&mut self) -> Result<Vec<Frame>> {
        let symbols = self.parse_atom_types()?;
        let natoms = symbols.len();

        let mut frames = Vec::new();
        while let Ok(frame_text) = self.collect_frame_text(natoms) {
            let mut frame = parse_frame
                .parse(&frame_text)
                .map_err(|e| parse_error(e, &frame_text))?;
            let pos = self.reader.get_current_position()?;
            info!("Parsed one frame at position {pos}");
            frame.symbols = symbols.clone();
            frames.push(frame);
        }

        Ok(frames)
    }
}
// 608566c9 ends here

// [[file:../../adaptors.note::600fda9b][600fda9b]]
#[test]
fn vasp_outcar() -> Result<()> {
    let f = "./tests/files/vasp/OUTCAR-5.3.5";
    let mut outcar = Outcar::try_from_path(f.as_ref())?;
    let frames = outcar.parse_frames().unwrap();
    assert_eq!(frames.len(), 1);

    // test files from Jmol
    let f = "./tests/files/vasp/OUTCAR-5.2";
    let mut outcar = Outcar::try_from_path(f.as_ref())?;
    let frames = outcar.parse_frames().unwrap();
    assert_eq!(frames.len(), 1);

    // test files from Jmol
    let f = "./tests/files/vasp/OUTCAR_diamond.dat";
    let mut outcar = Outcar::try_from_path(f.as_ref())?;
    let frames = outcar.parse_frames().unwrap();
    assert_eq!(frames.len(), 1);

    // test files from Jmol
    let f = "tests/files/vasp/AlH3_Vasp5.dat";
    let mut outcar = Outcar::try_from_path(f.as_ref())?;
    let frames = outcar.parse_frames().unwrap();
    assert_eq!(frames.len(), 7);
    assert!(frames[0].stress.is_some());

    // let f = "/home/ybyygu/Documents/ni1/04zr2/OUTCAR";
    // let mut outcar = Outcar::try_from_path(f.as_ref())?;
    // let frames = outcar.parse_frames().unwrap();
    // for f in frames {
    //     dbg!(f.stress);
    // }

    Ok(())
}
// 600fda9b ends here
