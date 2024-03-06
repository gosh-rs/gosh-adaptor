// [[file:../../adaptors.note::5265cd7e][5265cd7e]]
mod model;
mod parse;

use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use std::path::Path;

/// Mopac model adaptor
pub struct Mopac();

impl crate::ModelAdaptor for Mopac {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        self::model::get_mopac_results(outfile)
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        let all = self.parse_all(outfile)?;
        let last = all.into_iter().last().unwrap();
        Ok(last)
    }
}
// 5265cd7e ends here
