// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/gulp/mod.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*mod.rs][mod.rs:1]]
// mod model;
mod parse;

use gosh_core::gut::prelude::*;
use gosh_models::ModelProperties;

use std::path::Path;

/// Gulp model adaptor
pub struct Gulp();

impl crate::ModelAdaptor for Gulp {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        self::parse::get_gulp_results_all(outfile)
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        let all = self.parse_all(outfile)?;
        if let Some(last) = all.into_iter().last() {
            Ok(last)
        } else{
            bail!("parsed no result!");
        }
    }
}
// mod.rs:1 ends here
