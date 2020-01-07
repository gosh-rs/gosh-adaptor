// imports

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*imports][imports:1]]
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::{space0, space1};
use nom::number::complete::double;
use nom::sequence::tuple;

use nom::do_parse;
use nom::IResult;
// imports:1 ends here

// energy

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*energy][energy:1]]
//           TOTAL ENERGY            =       -720.18428 EV
fn get_total_energy(s: &str) -> IResult<&str, f64> {
    let token = "TOTAL ENERGY            =";
    let jump = jump_to(token);
    let tag_ev = tag("EV");
    do_parse!(
        s,
        jump >> space0 >> energy: double >> space0 >> tag_ev >> eol >> (energy)
    )
}

#[test]
fn test_mopac_energy() {
    let line = "TOTAL ENERGY            =       -720.18428 EV\n";
    let (r, en) = get_total_energy(line).unwrap();
    assert!(r == "");
    assert_eq!(-720.18428, en);
}
// energy:1 ends here

// dipole

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*dipole][dipole:1]]
use crate::parser::*;

//  DIPOLE           X         Y         Z       TOTAL
//  POINT-CHG.    -0.521    -0.058     0.081     0.531
//  HYBRID        -0.027    -0.069    -0.010     0.075
//  SUM           -0.548    -0.127     0.071     0.567
fn get_dipole(s: &str) -> IResult<&str, [f64; 3]> {
    let token = " DIPOLE           X         Y         Z       TOTAL";
    let goto_token = take_until(token);
    let header = tag(token);
    let tag_sum = tag(" SUM");
    nom::do_parse!(
        s,
        goto_token >>              // jump to the relevant line
        header     >> eol       >> // head line
        read_line  >> read_line >> // ignore following two lines
        tag_sum    >> space1    >> d: xyz_array >> space1 >> double >> eol >> // final line
        (d)
    )
}

#[test]
fn test_mopac_dipole() {
    let txt = " DIPOLE           X         Y         Z       TOTAL
 POINT-CHG.    -0.521    -0.058     0.081     0.531
 HYBRID        -0.027    -0.069    -0.010     0.075
 SUM           -0.548    -0.127     0.071     0.567
";
    let (r, [x, y, z]) = get_dipole(txt).unwrap();
    assert_eq!(-0.548, x);
    assert_eq!(-0.127, y);
    assert_eq!(0.071, z);
}
// dipole:1 ends here

// structure and forces

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*structure and forces][structure and forces:1]]
// PARAMETER     ATOM    TYPE            VALUE       GRADIENT
//     1          1  C    CARTESIAN X    -1.644300   -55.598091  KCAL/ANGSTROM
//     2          1  C    CARTESIAN Y    -0.817800    35.571574  KCAL/ANGSTROM
//     3          1  C    CARTESIAN Z     0.125500   -22.556128  KCAL/ANGSTROM
fn structure_and_gradient_line(s: &str) -> IResult<&str, (&str, f64, f64)> {
    let kcal_angs = tag("KCAL/ANGSTROM");
    let cartesian = tag("CARTESIAN");
    do_parse!(
        s,
        space0 >> digit1 >> space1 >> digit1 >> space1 >> // ignore
        s: alpha1 >> space1 >>                            // element symbol
        cartesian >> space1 >> alpha1 >> space1 >>        // ignore
        p: double >> space1 >> g: double >> space1 >>     // coordinate and gradient
        kcal_angs >> eol >>                               // ignore
        ((s, p, g))
    )
}

#[test]
fn test_strucuture_and_gradient() {
    let line = "      4          2  C    CARTESIAN X     1.523300     6.893093  KCAL/ANGSTROM\n";
    let (_, (sym, position, gradient)) = structure_and_gradient_line(line).unwrap();
    assert_eq!("C", sym);
    assert_eq!(1.523300, position);
    assert_eq!(6.893093, gradient);
}

fn get_atom_and_forces(s: &str) -> IResult<&str, (&str, [f64; 3], [f64; 3])> {
    do_parse!(
        s,
        px: structure_and_gradient_line >> // x component
        py: structure_and_gradient_line >> // y component
        pz: structure_and_gradient_line >> // z component
        ({
            let sym = px.0;
            let coords = [px.1, py.1, pz.1];
            let forces = [px.2, py.2, pz.2];
            (sym, coords, forces)
        })
    )
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
fn get_structure_and_gradients(s: &str) -> IResult<&str, Vec<(&str, [f64; 3], [f64; 3])>> {
    let token = "FINAL  POINT  AND  DERIVATIVES";
    let jump = jump_to(token);
    let read_many = many1(get_atom_and_forces);
    do_parse!(
        s,
        jump >> eol >>            // head line
        read_line >> read_line >> // skip two lines
        atoms: read_many >>       // collect all lines
        (atoms)
    )
}

#[test]
fn test_get_atoms() {
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
     12          4  C    CARTESIAN Z     0.076200    -5.837845  KCAL/ANGSTROM\n\n";

    let (_, atoms) = get_structure_and_gradients(txt).unwrap();
    assert_eq!(4, atoms.len());
}

// unit conversion
const DEBYE: f64 = 0.20819434;
const KCAL_MOL: f64 = 1.0 / 23.061;

use gosh_core::gchemol::Molecule;
use gosh_models::ModelProperties;
/// Get all calculation results.
fn get_mopac_results(s: &str) -> IResult<&str, ModelProperties> {
    do_parse!(
        s,
        energy: get_total_energy >> // force consistent energy
        data: get_structure_and_gradients >> // xxx
        dipole : get_dipole                        >>
        ({
            let mut mp = ModelProperties::default();
            mp.set_energy(energy);
            mp.set_dipole([
                dipole[0] * DEBYE,
                dipole[1] * DEBYE,
                dipole[2] * DEBYE,
            ]);
            // structure and gradients
            let atoms = data.iter().map(|(s, p, _)| (*s, *p));
            let mut mol = Molecule::new("mp");
            mol.add_atoms_from(atoms);
            mp.set_molecule(mol);
            // set forces
            let forces: Vec<_> = data.iter().map(|(_, _, f)| {
                [-f[0] * KCAL_MOL, -f[1] * KCAL_MOL, -f[2] * KCAL_MOL]
            }).collect();
            mp.set_forces(forces);
            mp
        })
    )
}

pub(crate) fn get_results(s: &str) -> IResult<&str, Vec<ModelProperties>> {
    many1(get_mopac_results)(s)
}
// structure and forces:1 ends here
