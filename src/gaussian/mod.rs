// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/gaussian/mod.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*mod.rs][mod.rs:1]]
mod gaussian_fchk;
mod gaussian_out;

use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;

use std::path::Path;

/// Gaussian model adaptor
pub struct Gaussian();

impl crate::ModelAdaptor for Gaussian {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        unimplemented!()
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        self::gaussian_fchk::parse_gaussian_fchk(outfile)
    }
}
// mod.rs:1 ends here
