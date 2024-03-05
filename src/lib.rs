// mods
// #+name: 3022ca5d

mod gaussian;
mod gulp;
mod mopac;
mod null;
mod parser;
mod repl;
mod siesta;
mod skim;

mod vasp;
// for winnow
mod parquet;
mod parsers;

//mod xtb;

mod common {
    pub use gosh_core::gut::prelude::*;
    pub use std::path::{Path, PathBuf};
}

// imports
// #+name: 1650f65e

use gosh_core::*;

use gchemol::prelude::*;
use gchemol::{Atom, Molecule};

// pub/trait
// #+name: 395f043c

use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;
use std::path::Path;

/// Common interface for model adaptors
pub trait ModelAdaptor {
    /// Parse the last entry of ModelProperties from a calculation output file
    ///
    /// # Return
    ///
    /// - ModelProperties, the calculated properties, including energy, forces, ...
    fn parse_last<P: AsRef<Path>>(&self, _outfile: P) -> Result<ModelProperties>;

    /// Parse all properties in multi-step calculation, sush as optimization or
    /// multi-molecule batch calculation.
    ///
    /// # Return
    ///
    /// - a list of ModelProperties
    fn parse_all<P: AsRef<Path>>(&self, _outfile: P) -> Result<Vec<ModelProperties>>;
}

// exports
// #+name: 111a01d3

pub use crate::gaussian::Gaussian;
pub use crate::gulp::Gulp;
pub use crate::mopac::Mopac;
pub use crate::null::Null;
pub use crate::siesta::Siesta;
pub use crate::vasp::Vasp;

pub use crate::repl::repl_enter_main;

/// Write parsed results in parquet format.
pub use crate::parquet::ParquetWrite;

// exports/docs
// #+name: cfaadb2d

#[cfg(feature = "adhoc")]
/// Docs for local mods
pub mod docs {
    macro_rules! export_doc {
        ($l:ident) => {
            pub mod $l {
                pub use crate::$l::*;
            }
        };
    }

    export_doc!(parsers);
    export_doc!(vasp);
}
