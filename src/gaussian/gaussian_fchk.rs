// imports
// #+name: 2336ef22

use super::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use gchemol_parser::TextReader;
use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

// base
// #+name: fd80cfdb

/// Represents data read from Gaussian .fchk file.
pub struct GaussianFchk {
    reader: TextReader<BufReader<File>>,
}

impl GaussianFchk {
    pub fn try_from_path(f: &Path) -> Result<Self> {
        let reader = TextReader::try_from_path(f)?;
        Ok(Self { reader })
    }
}

// collect
// # A40,3X,A1,3X,’N=’,I12
// # Alpha Orbital Energies                     R   N=          23

// #+name: a6536f00

// check if a line is a data label
fn is_data_label(line: &str) -> bool {
    let data_labels = ["R", "I", "C", "L"];
    line.len() >= 50 && data_labels.contains(&&line[43..44])
}

impl GaussianFchk {
    fn collect_parts(&mut self) -> impl Iterator<Item = (String, String)> + '_ {
        std::iter::from_fn(move || {
            let mut buf = String::new();
            self.reader.seek_line(is_data_label).ok()?;
            let _ = self.reader.read_line(&mut buf).ok()?;
            let head = buf.clone();
            buf.clear();
            let r = self.reader.read_until(&mut buf, is_data_label);

            if r.is_err() {
                Some((head, buf))
            } else {
                Some((head, buf))
            }
        })
    }
}

#[test]
fn test_gaussian_fchk_collect() -> Result<()> {
    use vecfx::approx::*;

    let fchkfile = "tests/files/gaussian/Test.FChk";
    let mut fchk = GaussianFchk::try_from_path(fchkfile.as_ref())?;
    let parts: Vec<_> = fchk.collect_parts().collect();
    assert_eq!(parts.len(), 87);

    Ok(())
}

// pub
// #+name: 44998624

use gosh_core::gchemol::Molecule;

const HARTREE: f64 = 27.211386024367243;
const BOHR: f64 = 0.5291772105638411;

impl GaussianFchk {
    /// Parse model properties from Gaussian/fchk file.
    pub fn parse_computed(&mut self) -> Result<ModelProperties> {
        let energy_token = "Total Energy                               R";
        let n = energy_token.len();

        let mut mp = ModelProperties::default();
        let mut symbols = vec![];
        let mut positions = vec![];
        for (label, data) in self.collect_parts() {
            match &label[..n] {
                "Total Energy                               R" => {
                    let (_, e) = label.split_at(n);
                    let energy: f64 = e.trim().parse().unwrap();
                    mp.set_energy(energy * HARTREE);
                }
                "Dipole Moment                              R" => {
                    let d: Vec<_> = data.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
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
}

// test
// #+name: 14193c61

#[test]
fn test_gaussian_fchk() -> Result<()> {
    use vecfx::approx::*;

    let f = "tests/files/gaussian/Test.FChk";
    let mut fchk = GaussianFchk::try_from_path(f.as_ref())?;
    let mp = fchk.parse_computed()?;
    let energy = mp.get_energy().expect("fchk energy");
    assert_relative_eq!(energy, -1.177266855882589E+02 * HARTREE, epsilon = 1e-4);

    let dipole = mp.get_dipole();
    assert_eq!(dipole, Some([4.62871432E-03, -3.30050130E-03, 5.71484730E-03]));

    let forces = mp.get_forces().expect("fchk forces");
    assert_eq!(forces.len(), 11);
    assert_relative_eq!(forces[0][0], 9.44446832E-03 * HARTREE / BOHR, epsilon = 1e-4);

    let mol = mp.get_molecule().expect("fchk mol");
    assert_eq!(mol.natoms(), 11);

    let position = mol.positions().next().unwrap();
    assert_relative_eq!(position[0], -6.57759708E+00 * BOHR);

    Ok(())
}
