// mod.rs
// :PROPERTIES:
// :header-args: :tangle src/vasp/mod.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*mod.rs][mod.rs:1]]
mod parse;

use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use std::path::Path;

/// VASP model adaptor
pub struct Vasp();

impl crate::ModelAdaptor for Vasp {
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>> {
        self::parse::parse_vasp_outcar(outfile)
    }

    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties> {
        let all = self.parse_all(outfile)?;
        if let Some(last) = all.into_iter().last() {
            Ok(last)
        } else {
            bail!("parsed no result!");
        }
    }
}
// mod.rs:1 ends here
