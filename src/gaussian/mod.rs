// [[file:../../adaptors.note::b33ce62d][b33ce62d]]
use super::*;

mod gaussian_fchk;
mod gaussian_out;

use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use std::path::Path;

/// Gaussian model adaptor
pub struct Gaussian();

impl crate::ModelAdaptor for Gaussian {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        let mut gauss_out = self::gaussian_out::GaussianOutput::try_from_path(outfile.as_ref())?;
        let frames = gauss_out.parse_frames()?;
        let mps = frames
            .into_iter()
            .map(|frame| {
                let mut mp = ModelProperties::default();
                mp.set_energy(frame.energy);
                mp.set_forces(frame.forces);
                let atoms: Vec<_> = frame.atomic_numbers.into_iter().zip(frame.positions).collect();
                let mol = Molecule::from_atoms(atoms);
                mp.set_molecule(mol);
                mp
            })
            .collect();
        Ok(mps)
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        use gchemol_parser::TextReader;

        // FIXME: rewrite below
        let f = outfile.as_ref();
        let mut reader = TextReader::try_from_path(f)?;
        if let Some(first_line) = reader.lines().next() {
            if first_line.starts_with(" Entering Gaussian System") {
                info!("Recognized as Gaussian output file.");
                let all = self.parse_all(f)?;
                if let Some(last) = all.into_iter().last() {
                    return Ok(last);
                } else {
                    bail!("parsed no result!")
                }
            }
        }
        let mut fchk = self::gaussian_fchk::GaussianFchk::try_from_path(f)?;
        fchk.parse_computed()
    }
}
// b33ce62d ends here
