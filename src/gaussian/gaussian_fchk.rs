// imports

use crate::parser::*;

// collect parts

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
                data_lines.push_str(&line);
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

/// Parse model properties from Gaussian/fchk file.
pub(crate) fn parse_gaussian_fchk<P: AsRef<Path>>(fchkfile: P) -> Result<ModelProperties> {
    let r = text_file_reader(fchkfile)?;
    let parts = FchkDataRecords::new(r);

    let mut mp = ModelProperties::default();
    let energy_token = "Total Energy                               R";
    let n = energy_token.len();
    for (label, data) in parts {
        match &label[..n] {
            "Total Energy                               R" => {
                let (_, e) = label.split_at(n);
                let energy: f64 = e.trim().parse().unwrap();
                mp.set_energy(energy);
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
                    .map(|x| -x.parse::<f64>().unwrap())
                    .collect();
                let gradients: Vec<_> = d.chunks_exact(3).map(|x| [x[0], x[1], x[2]]).collect();
                mp.set_forces(gradients);
            }
            "Current cartesian coordinates              R" => {
                // TODO
            }
            _ => {
                // ignore
            }
        }
    }

    Ok(mp)
}

// test

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;

#[test]
fn test_gaussian_fchk() -> Result<()> {
    let fchkfile = "tests/files/gaussian/Test.FChk";
    let mp = parse_gaussian_fchk(fchkfile)?;
    let energy = mp.get_energy();
    assert_eq!(energy, Some(-1.177266855882589E+02));
    let dipole = mp.get_dipole();
    assert_eq!(
        dipole,
        Some([4.62871432E-03, -3.30050130E-03, 5.71484730E-03])
    );

    let forces = mp.get_forces().expect("fchk forces");
    assert_eq!(forces.len(), 11);
    assert_eq!(forces[0], [9.44446832E-03, 4.83256604E-03, 1.82167947E-02]);

    Ok(())
}
