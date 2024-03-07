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
        let frames = self::gaussian_out::parse_frames(outfile.as_ref())?;
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
        self::gaussian_fchk::parse_gaussian_fchk(outfile)
    }
}
// b33ce62d ends here
