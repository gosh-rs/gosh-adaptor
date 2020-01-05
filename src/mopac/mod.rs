// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/mopac/mod.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*mod.rs][mod.rs:1]]
mod model;
mod parse;

use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;

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
// mod.rs:1 ends here
