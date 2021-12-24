// imports
// #+name: 2336ef22

use super::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

// model

// Hartree to eV
const HARTREE: f64 = 27.211386024367243;
const BOHR: f64 = 0.5291772105638411;

use gosh_core::gchemol::Molecule;
use gosh_core::text_parser::TextReader;

/// Parse model properties from Gaussian/fchk file.
pub(crate) fn parse_gaussian_fchk<P: AsRef<Path>>(fchkfile: P) -> Result<ModelProperties> {
    let mut mp = ModelProperties::default();
    let energy_token = "Total Energy                               R";
    let n = energy_token.len();
    let mut symbols = vec![];
    let mut positions = vec![];

    // check if a line is a data label
    fn is_data_label(line: &str) -> bool {
        line.len() >= 50 && line.chars().next().unwrap().is_uppercase()
    }

    let r = TextReader::from_path(fchkfile)?;
    for bunch in r.partitions_preceded(is_data_label) {
        let mut data = bunch.splitn(2, "\n");
        let label = data.next().expect("chk label");
        let data = data.next().expect("chk data");
        match &label[..n] {
            "Total Energy                               R" => {
                let (_, e) = label.split_at(n);
                let energy: f64 = e.trim().parse().unwrap();
                mp.set_energy(energy * HARTREE);
            }
            "Dipole Moment                              R" => {
                let d: Vec<_> = data
                    .split_whitespace()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect();
                mp.set_dipole([d[0], d[1], d[2]]);
            }
            "Cartesian Gradient                         R" => {
                let d: Vec<_> = data
                    .split_whitespace()
                    .map(|x| HARTREE / BOHR * -x.parse::<f64>().unwrap()) // unit conversion
                    .collect();
                let gradients: Vec<_> = d.chunks_exact(3).map(|x| [x[0], x[1], x[2]]).collect();
                mp.set_forces(gradients);
            }
            "Atomic numbers                             I" => {
                symbols = data.split_whitespace().map(|s| s.to_owned()).collect();
            }
            "Current cartesian coordinates              R" => {
                let d: Vec<_> = data
                    .split_whitespace()
                    .map(|x| BOHR * x.parse::<f64>().unwrap()) // unit conversion
                    .collect();
                positions = d.chunks_exact(3).map(|x| [x[0], x[1], x[2]]).collect();
            }
            _ => {
                // ignore
            }
        }
    }
    let atoms = symbols.iter().zip(positions.into_iter());
    let mol = Molecule::from_atoms(atoms);
    mp.set_molecule(mol);

    Ok(mp)
}

// test
// #+name: 14193c61

#[test]
fn test_gaussian_fchk() -> Result<()> {
    use vecfx::approx::*;

    let fchkfile = "tests/files/gaussian/Test.FChk";
    let mp = parse_gaussian_fchk(fchkfile)?;
    let energy = mp.get_energy().expect("fchk energy");
    assert_relative_eq!(energy, -1.177266855882589E+02 * HARTREE, epsilon = 1e-4);

    let dipole = mp.get_dipole();
    assert_eq!(
        dipole,
        Some([4.62871432E-03, -3.30050130E-03, 5.71484730E-03])
    );

    let forces = mp.get_forces().expect("fchk forces");
    assert_eq!(forces.len(), 11);
    assert_relative_eq!(
        forces[0][0],
        9.44446832E-03 * HARTREE / BOHR,
        epsilon = 1e-4
    );

    let mol = mp.get_molecule().expect("fchk mol");
    assert_eq!(mol.natoms(), 11);

    let position = mol.positions().next().unwrap();
    assert_relative_eq!(position[0], -6.57759708E+00 * BOHR);

    Ok(())
}
