// core

use gosh_core::gchemol;
use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;
use std::path::Path;

/// Siesta model adaptor.
pub struct Null();

impl crate::ModelAdaptor for Null {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        unimplemented!()
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        use gchemol::prelude::*;

        let mol = gchemol::Molecule::from_file(outfile)?;
        let mut mp = ModelProperties::default();
        mp.set_energy(0.0);
        let f: Vec<_> = mol.positions().map(|_| [0.0; 3]).collect();
        mp.set_forces(f);

        Ok(mp)
    }
}
