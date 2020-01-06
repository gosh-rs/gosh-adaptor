// imports

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*imports][imports:1]]
use crate::parser::*;
// imports:1 ends here

// energy

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*energy][energy:1]]
// Final energy =     -91.56438967 eV
// Final Gnorm  =       0.00027018
fn get_total_energy(s: &str) -> IResult<&str, f64> {
    let token = "\n  Final energy =";
    let jump = jump_to(token);
    let tag_ev = tag("eV");
    do_parse!(s, jump >> space0 >> e: double >> space0 >> tag_ev >> eol >> (e))
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
    let (_, en) = get_total_energy(line).unwrap();
    assert_eq!(-91.56438967, en);
}
// energy:1 ends here

// structure
//   Final cartesian coordinates of atoms :

// --------------------------------------------------------------------------------
//    No.  Atomic        x           y          z          Radius
//         Label       (Angs)      (Angs)     (Angs)       (Angs)
// --------------------------------------------------------------------------------
//      1  C     c    -0.032974   -0.007766    0.174240    0.000000
//      2  C     c     0.091865   -0.004549    3.012901    0.000000
//      3  H     c     1.128930    0.063389    2.902369    0.000000

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*structure][structure:1]]
fn get_structure(s: &str) -> IResult<&str, Vec<(&str, [f64; 3])>> {
    let token = "\n  Final cartesian coordinates of atoms :";
    let jump = jump_to(token);
    let read_atoms = many1(structure_line);
    do_parse!(
        s,
        jump >> eol >>        // ignore head line
        read_line   >>        // ignore blank line
        read_line   >>        // ignore "----"
        read_line   >>        // ignore "No."
        read_line   >>        // ignore "Label"
        read_line   >>        // ignore "---"
        atoms: read_atoms  >> // element symbol and coordinates
        (atoms)
    )
}

fn structure_line(s: &str) -> IResult<&str, (&str, [f64; 3])> {
    do_parse!(
        s,
        space0 >> digit1 >> space1 >>                   // ignore
        symbol: alpha1 >> space1 >> alpha1 >> space1 >> // element symbol
        coords: xyz_array >> space1 >> double >> eol >> // coordinates
        ((symbol, coords))
    )
}
// structure:1 ends here

// forces
//   Final Cartesian derivatives :

// --------------------------------------------------------------------------------
//    No.  Atomic          x             y             z           Radius
//         Label       (eV/Angs)     (eV/Angs)    (eV/Angs)      (eV/Angs)
// --------------------------------------------------------------------------------
//       1 C     c       0.000272      0.000953     -0.003050      0.000000
//       2 C     c       0.004596      0.004329      0.002279      0.000000
//       3 H     c      -0.003862     -0.003268      0.001706      0.000000
//       4 C     c      -0.006011     -0.001707     -0.000802      0.000000

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*forces][forces:1]]
fn get_forces(s: &str) -> IResult<&str, Vec<[f64; 3]>> {
    let token = "\n  Final Cartesian derivatives :";
    let jump = jump_to(token);
    let read_grads = many1(structure_line);
    do_parse!(
        s,
        jump >> eol >>        // ignore head line
        read_line   >>        // ignore blank line
        read_line   >>        // ignore "----"
        read_line   >>        // ignore "No."
        read_line   >>        // ignore "Label"
        read_line   >>        // ignore "---"
        grads: read_grads  >> // element symbol and coordinates
        ({
            grads.into_iter().map(|(_, g)| g).collect()
        })
    )
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
    let (_, forces) = get_forces(txt).unwrap();
    assert_eq!(16, forces.len());
}
// forces:1 ends here

// model

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*model][model:1]]
use gosh_core::gchemol::Molecule;
use gosh_core::guts;
use gosh_models::ModelProperties;

use guts::fs::*;
use guts::prelude::*;

fn get_gulp_results(s: &str) -> IResult<&str, ModelProperties> {
    do_parse!(
        s,
        energy: get_total_energy >> // energy
        atoms: get_structure >>     // symbols and coordinates
        forces: get_forces >>       // forces
        ({
            let mut mp = ModelProperties::default();
            mp.set_energy(energy);
            mp.set_forces(forces);
            // construct molecule
            let mut mol = Molecule::new("gulp");
            mol.add_atoms_from(atoms);
            mp.set_molecule(mol);
            mp
        })
    )
}

/// Get all results for multiple structures.
pub(crate) fn get_gulp_results_all<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    let s = read_file(fout)?;
    let (_, mps) = many1(get_gulp_results)(&s)
        .map_err(|e| format_err!("Failed to parse gulp results:\n{:?}", e))?;
    Ok(mps)
}

#[test]
fn test_get_gulp_results() {
    let f = "./tests/files/gulp-multiple/gulp.out";
    let mps = get_gulp_results_all(f).unwrap();
    assert_eq!(mps.len(), 6);
}
// model:1 ends here
