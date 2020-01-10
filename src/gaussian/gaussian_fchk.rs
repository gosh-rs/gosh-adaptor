// imports

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;

// collect

type FileReader = BufReader<File>;

fn text_file_reader<P: AsRef<Path>>(p: P) -> Result<FileReader> {
    let f = File::open(p.as_ref())?;
    let reader = BufReader::new(f);
    Ok(reader)
}

struct FchkDataRecords {
    label: String,
    lines: std::io::Lines<FileReader>,
}

impl Iterator for FchkDataRecords {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut data_lines = String::new();
        while let Some(line) = self.lines.next() {
            let line = line.unwrap();
            if is_data_label(&line) {
                let head = self.label.to_string();
                self.label = line.to_string();
                // skip the first empty line
                if !head.is_empty() {
                    return Some((head, data_lines));
                }
            } else {
                data_lines += &line;
                // the line ending
                data_lines += "\n";
            }
        }
        // Handle the final section
        if self.label.is_empty() {
            None
        } else {
            let head = self.label.to_string();
            self.label.clear();
            Some((head, data_lines))
        }
    }
}

fn is_data_label(line: &str) -> bool {
    line.len() >= 50 && line.chars().next().unwrap().is_uppercase()
}

impl FchkDataRecords {
    pub fn new(reader: FileReader) -> Self {
        Self {
            lines: reader.lines(),
            label: String::new(),
        }
    }
}

// model

// Hartree to eV
const HARTREE: f64 = 27.211386024367243;
const BOHR: f64 = 0.5291772105638411;

use gosh_core::gchemol::Molecule;

/// Parse model properties from Gaussian/fchk file.
pub(crate) fn parse_gaussian_fchk<P: AsRef<Path>>(fchkfile: P) -> Result<ModelProperties> {
    let r = text_file_reader(fchkfile)?;
    let parts = FchkDataRecords::new(r);

    let mut mp = ModelProperties::default();
    let energy_token = "Total Energy                               R";
    let n = energy_token.len();
    let mut symbols = vec![];
    let mut positions = vec![];
    for (label, data) in parts {
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
    let mut mol = Molecule::new("gaussian fchk");
    let atoms = symbols.into_iter().zip(positions.into_iter());
    mol.add_atoms_from(atoms);
    mp.set_molecule(mol);

    Ok(mp)
}

// test

#[test]
fn test_gaussian_fchk() -> Result<()> {
    use approx::*;

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

    let positions = mol.positions();
    assert_relative_eq!(positions[0][0], -6.57759708E+00 * BOHR);

    Ok(())
}
