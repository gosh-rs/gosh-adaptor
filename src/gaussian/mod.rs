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
        unimplemented!()
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        self::gaussian_fchk::parse_gaussian_fchk(outfile)
    }
}
// b33ce62d ends here
