// [[file:../../adaptors.note::*imports][imports:1]]
use gosh_core::gchemol::Molecule;
use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use gchemol_parser::parsers::*;
// imports:1 ends here

// [[file:../../adaptors.note::88e6dcd0][88e6dcd0]]
//           TOTAL ENERGY            =       -720.18428 EV
fn get_total_energy(s: &mut &str) -> PResult<f64> {
    // TOTAL ENERGY            =       -720.18428 EV\n
    let total_energy_label = "TOTAL ENERGY            =";
    let energy = seq! {
        _: jump_to(total_energy_label),
        ws(double),
        _: "EV", _: line_ending,
    }
    .context(label("Energy"))
    .parse_next(s)?;
    Ok(energy.0)
}

#[test]
fn test_mopac_energy() {
    let line = "TOTAL ENERGY            =       -720.18428 EV\n";
    let (r, en) = get_total_energy.parse_peek(line).unwrap();
    assert!(r == "");
    assert_eq!(-720.18428, en);
}
// 88e6dcd0 ends here

// [[file:../../adaptors.note::8c9b7cb0][8c9b7cb0]]
//  DIPOLE           X         Y         Z       TOTAL
//  POINT-CHG.    -0.521    -0.058     0.081     0.531
//  HYBRID        -0.027    -0.069    -0.010     0.075
//  SUM           -0.548    -0.127     0.071     0.567
fn dipole(s: &mut &str) -> PResult<[f64; 3]> {
    let pattern = " DIPOLE           X         Y         Z       TOTAL";
    let dipole = seq! {
        //  DIPOLE           X         Y         Z       TOTAL
        _: jump_to(pattern), _: line_ending, // jump to the relevant line
        _: read_line, _: read_line, // ignore following two lines
        //  SUM           -0.548    -0.127     0.071     0.567
        _: " SUM", ws(xyz_array), _: ws(double), _: line_ending,
    }
    .context(label("Dipole moment"))
    .parse_next(s)?;
    Ok(dipole.0)
}

#[test]
fn test_mopac_dipole() {
    let txt = " DIPOLE           X         Y         Z       TOTAL
 POINT-CHG.    -0.521    -0.058     0.081     0.531
 HYBRID        -0.027    -0.069    -0.010     0.075
 SUM           -0.548    -0.127     0.071     0.567
";
    let (r, [x, y, z]) = dipole.parse_peek(txt).unwrap();
    assert_eq!(-0.548, x);
    assert_eq!(-0.127, y);
    assert_eq!(0.071, z);
}
// 8c9b7cb0 ends here

// [[file:../../adaptors.note::1ca62637][1ca62637]]
// PARAMETER     ATOM    TYPE            VALUE       GRADIENT
//     1          1  C    CARTESIAN X    -1.644300   -55.598091  KCAL/ANGSTROM
//     2          1  C    CARTESIAN Y    -0.817800    35.571574  KCAL/ANGSTROM
//     3          1  C    CARTESIAN Z     0.125500   -22.556128  KCAL/ANGSTROM
fn structure_and_gradient_line<'a>(s: &mut &'a str) -> PResult<(&'a str, f64, f64)> {
    seq! {
        _: space0, _: digit1, _: space1, _: digit1, // ignore
        ws(alpha1),                                 // element symbol
        _: "CARTESIAN", _: space1, _: alpha1,
        ws(double),             // cartesian coordinate
        ws(double),             // cartesian gradient
        _: "KCAL/ANGSTROM", _: rest_line,
    }
    .context(label("Structure and gradients"))
    .parse_next(s)
}

#[test]
fn test_strucuture_and_gradient() {
    let line = "      4          2  C    CARTESIAN X     1.523300     6.893093  KCAL/ANGSTROM\n";
    let (_, (sym, position, gradient)) = structure_and_gradient_line.parse_peek(line).unwrap();
    assert_eq!("C", sym);
    assert_eq!(1.523300, position);
    assert_eq!(6.893093, gradient);

    // A strange output
    // it will not work
    // let line = "      4          2 Si    CARTESIAN X    -2.431384-27235.808855  KCAL/ANGSTROM";
    // let (_, (sym, position, gradient)) = structure_and_gradient_line.parse_peek(line).unwrap();
    // dbg!(sym, position, gradient);
}

fn positions_and_forces<'a>(s: &mut &'a str) -> PResult<Vec<(&'a str, [f64; 3], [f64; 3])>> {
    let p_and_g: Vec<_> = repeat(1.., structure_and_gradient_line).parse_next(s)?;

    let o = p_and_g
        .chunks_exact(3)
        .map(|ppp| {
            let px = ppp[0];
            let py = ppp[1];
            let pz = ppp[2];
            let sym = px.0;
            let coords = [px.1, py.1, pz.1];
            let forces = [px.2, py.2, pz.2];
            (sym, coords, forces)
        })
        .collect();
    Ok(o)
}

//       FINAL  POINT  AND  DERIVATIVES
//
//   PARAMETER     ATOM    TYPE            VALUE       GRADIENT
//      1          1  C    CARTESIAN X     0.226237    20.003912  KCAL/ANGSTROM
//      2          1  C    CARTESIAN Y    -0.013364    38.431901  KCAL/ANGSTROM
//      3          1  C    CARTESIAN Z    -0.595854    -8.686483  KCAL/ANGSTROM
//      4          2  C    CARTESIAN X     0.136244    15.864154  KCAL/ANGSTROM
//      5          2  C    CARTESIAN Y    -0.253911    38.980394  KCAL/ANGSTROM
//      6          2  C    CARTESIAN Z     3.386110     3.736531  KCAL/ANGSTROM
fn get_structure_and_gradients<'a>(s: &mut &'a str) -> PResult<Vec<(&'a str, [f64; 3], [f64; 3])>> {
    let mut skip = (
        jump_to("FINAL  POINT  AND  DERIVATIVES"),
        read_line,
        read_line,
        read_line,
    );
    skip.parse_next(s)?;

    let atoms = positions_and_forces
        .context(label("structures and gradients"))
        .parse_next(s)?;
    Ok(atoms)
}

#[test]
fn mopac_get_atoms() {
    let txt = "       FINAL  POINT  AND  DERIVATIVES

   PARAMETER     ATOM    TYPE            VALUE       GRADIENT
      1          1  C    CARTESIAN X    -1.743000   -80.695675  KCAL/ANGSTROM
      2          1  C    CARTESIAN Y    -0.725100    73.306387  KCAL/ANGSTROM
      3          1  C    CARTESIAN Z     0.044900   -23.565223  KCAL/ANGSTROM
      4          2  C    CARTESIAN X     1.523300     6.893093  KCAL/ANGSTROM
      5          2  C    CARTESIAN Y    -0.946300   -16.682683  KCAL/ANGSTROM
      6          2  C    CARTESIAN Z    -0.005100    22.532087  KCAL/ANGSTROM
      7          3  C    CARTESIAN X    -1.248600   -12.624765  KCAL/ANGSTROM
      8          3  C    CARTESIAN Y     0.589400   -35.843890  KCAL/ANGSTROM
      9          3  C    CARTESIAN Z    -0.026800     1.107735  KCAL/ANGSTROM
     10          4  C    CARTESIAN X     1.222600   -40.743520  KCAL/ANGSTROM
     11          4  C    CARTESIAN Y     0.386900    34.401001  KCAL/ANGSTROM
     12          4  C    CARTESIAN Z     0.076200    -5.837845  KCAL/ANGSTROM\n";

    let (_, atoms) = get_structure_and_gradients.parse_peek(txt).unwrap();
    assert_eq!(4, atoms.len());
}

// unit conversion
const DEBYE: f64 = 0.20819434;
const KCAL_MOL: f64 = 1.0 / 23.061;

/// Get all calculation results.
fn parse_mopac_out_frame(s: &mut &str) -> PResult<ModelProperties> {
    let (energy, data, dipole) = (
        get_total_energy,            // force consistent energy
        get_structure_and_gradients, // coordinates and gradients
        dipole,                      // dipole moments
    )
        .context(label("mopac frame"))
        .parse_next(s)?;

    let mut mp = ModelProperties::default();
    mp.set_energy(energy);
    mp.set_dipole([dipole[0] * DEBYE, dipole[1] * DEBYE, dipole[2] * DEBYE]);
    // structure and gradients
    let atoms = data.iter().map(|(s, p, _)| (*s, *p));
    let natoms = atoms.len();
    let mol = Molecule::from_atoms(atoms);
    mp.set_molecule(mol);
    // set forces
    let forces: Vec<_> = data
        .iter()
        .map(|(_, _, f)| [-f[0] * KCAL_MOL, -f[1] * KCAL_MOL, -f[2] * KCAL_MOL])
        .collect();
    assert_eq!(forces.len(), natoms, "found invalid data: {s:?}");
    mp.set_forces(forces);

    Ok(mp)
}
// 1ca62637 ends here

// [[file:../../adaptors.note::fcd3cf01][fcd3cf01]]
fn parse_mopac_out(s: &mut &str) -> PResult<Vec<ModelProperties>> {
    let frames = repeat(1.., parse_mopac_out_frame);
    // Consume rest lines to reach EOF, so that Parser.parse is ok for that
    terminated(frames, rest).parse_next(s)
}

pub(crate) fn get_results(s: &str) -> Result<Vec<ModelProperties>> {
    parse_mopac_out.parse(s).map_err(|e| parse_error(e, s))
}
// fcd3cf01 ends here
