// [[file:../../adaptors.note::43309458][43309458]]
use gchemol_parser::parsers::*;
// use gosh_core::text_parser::parsers::*;
// 43309458 ends here

// [[file:../../adaptors.note::f14d6ea0][f14d6ea0]]
// Final energy =     -91.56438967 eV
// Final Gnorm  =       0.00027018
fn get_total_energy(s: &mut &str) -> PResult<f64> {
    let _ = jump_to("\n  Final energy =").parse_next(s)?;
    terminated(ws(double), "eV").parse_next(s)
}

#[test]
fn test_energy() {
    let line = "
  **** Optimisation achieved ****


  Final energy =     -91.56438967 eV
  Final Gnorm  =       0.00027018

  Components of energy :

--------------------------------------------------------------------------------
  Interatomic potentials     =           0.00000000 eV
  Bond-order potentials      =         -91.56438967 eV
  Monopole - monopole (real) =           0.00000000 eV
--------------------------------------------------------------------------------
  Total lattice energy       =         -91.56438967 eV
--------------------------------------------------------------------------------
  Total lattice energy       =           -8834.5639 kJ/mol
--------------------------------------------------------------------------------
";
    let (_, en) = get_total_energy.parse_peek(line).unwrap();
    assert_eq!(-91.56438967, en);
}
// f14d6ea0 ends here

// [[file:../../adaptors.note::e7b05197][e7b05197]]
fn get_structure<'a>(s: &mut &'a str) -> PResult<Vec<(&'a str, [f64; 3])>> {
    let token = "\n  Final cartesian coordinates of atoms :";
    let x = seq! {
        _: jump_to(token), _: line_ending, // ignore head line
        _: read_line,                      // ignore blank line
        _: read_line,                      // ignore "----"
        _: read_line,                      // ignore "No."
        _: read_line,                      // ignore "Label"
        _: read_line,                      // ignore "---"
        repeat(1.., structure_line),       // element symbol and coordinates
    }
    .parse_next(s)?;
    Ok(x.0)
}

fn structure_line<'a>(s: &mut &'a str) -> PResult<(&'a str, [f64; 3])> {
    seq! {
        _: ws(digit1),          // ignore
        ws(alpha1),             // element symbol,
        _: alpha1, _: space1,   // ignore
        ws(xyz_array),          // coordinates
        _: rest_line,           // ignore
    }
    .parse_next(s)
}
// e7b05197 ends here

// [[file:../../adaptors.note::d07f0f0f][d07f0f0f]]
fn get_forces(s: &mut &str) -> PResult<Vec<[f64; 3]>> {
    let token = "\n  Final Cartesian derivatives :";
    let grads: (Vec<_>,) = seq! {
        _: jump_to(token), _: line_ending, // ignore head line
        _: read_line,                      // ignore blank line
        _: read_line,                      // ignore "----"
        _: read_line,                      // ignore "No."
        _: read_line,                      // ignore "Label"
        _: read_line,                      // ignore "---"
        repeat(1.., structure_line),       // element symbol and coordinates
    }
    .parse_next(s)?;

    Ok(grads.0.into_iter().map(|(_, g)| g).collect())
}

#[test]
fn test_forces() {
    let txt = "

  Final Cartesian derivatives :

--------------------------------------------------------------------------------
   No.  Atomic          x             y             z           Radius
        Label       (eV/Angs)     (eV/Angs)    (eV/Angs)      (eV/Angs)
--------------------------------------------------------------------------------
      1 C     c       0.002751      0.001390     -0.003469      0.000000
      2 C     c       0.001449      0.002452      0.003151      0.000000
      3 C     c       0.000000      0.000000      0.000000      0.000000
      4 C     c       0.000770     -0.000683     -0.002221      0.000000
      5 C     c      -0.001782      0.000383     -0.001570      0.000000
      6 C     c      -0.000510     -0.000158     -0.001059      0.000000
      7 C     c      -0.002169     -0.000789     -0.001802      0.000000
      8 C     c      -0.000586     -0.000156     -0.000930      0.000000
      9 C     c      -0.001191     -0.000769     -0.000263      0.000000
     10 C     c       0.000717      0.002025      0.002112      0.000000
     11 C     c       0.000607     -0.001269      0.000072      0.000000
     12 C     c       0.002147      0.000618     -0.002932      0.000000
     13 C     c      -0.003072     -0.000312     -0.000294      0.000000
     14 C     c      -0.000142     -0.003763      0.002650      0.000000
     15 C     c      -0.000068      0.002719      0.001997      0.000000
     16 C     c       0.001216      0.002239      0.003134      0.000000
--------------------------------------------------------------------------------
  Maximum abs         0.003072      0.003763      0.003469      0.000000
--------------------------------------------------------------------------------
  ";
    let (_, forces) = get_forces.parse_peek(txt).unwrap();
    assert_eq!(16, forces.len());
}
// d07f0f0f ends here

// [[file:../../adaptors.note::4a4abe15][4a4abe15]]
use gosh_core::gchemol::Molecule;
use gosh_core::gut;
use gosh_model::ModelProperties;

use gut::fs::*;
use gut::prelude::*;

fn get_gulp_results(s: &mut &str) -> PResult<Vec<ModelProperties>> {
    let parse_frame = (get_total_energy, get_structure, get_forces);
    let frames: Vec<_> = repeat(1.., parse_frame).parse_next(s)?;
    // consuming rest text for Parser.parse usage
    let _ = rest.parse_next(s)?;

    let mps = frames
        .into_iter()
        .map(|(energy, atoms, forces)| {
            let mut mp = ModelProperties::default();
            mp.set_energy(energy);
            mp.set_forces(forces);
            // construct molecule
            let mol = Molecule::from_atoms(atoms);
            mp.set_molecule(mol);
            mp
        })
        .collect();
    Ok(mps)
}

/// Get all results for multiple structures.
pub(crate) fn get_gulp_results_all<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    let s = read_file(fout)?;
    let mps = get_gulp_results.parse(&s).map_err(|e| parse_error(e, &s))?;
    Ok(mps)
}

#[test]
fn test_get_gulp_results() {
    let f = "./tests/files/gulp-multiple/gulp.out";
    let mps = get_gulp_results_all(f).unwrap();
    assert_eq!(mps.len(), 6);
}
// 4a4abe15 ends here
