// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/siesta/mod.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*mod.rs][mod.rs:1]]
mod model;
mod parse;

use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;
use std::path::Path;

/// Siesta model adaptor.
pub struct Siesta();

impl crate::ModelAdaptor for Siesta {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        unimplemented!()
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        self::model::get_siesta_results(outfile.as_ref())
    }
}
// mod.rs:1 ends here
