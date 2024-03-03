


// * lib.rs
// :PROPERTIES:
// :header-args: :comments org :tangle src/lib.rs
// :END:
// ** mods
// #+name: 3022ca5d

mod gaussian;
mod gulp;
mod mopac;
mod null;
mod parser;
mod repl;
mod siesta;
mod skim;

// FIXME: remove
mod vasp;
// for winnow
mod parsers;

mod vasp2;

//mod xtb;

mod common {
    pub use gosh_core::gut::prelude::*;
}

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



// ** imports
// #+name: 1650f65e

use gosh_core::*;

use gchemol::prelude::*;
use gchemol::{Atom, Molecule};



// ** exports
// #+name: 111a01d3

pub use crate::gulp::Gulp;
pub use crate::mopac::Mopac;
pub use crate::siesta::Siesta;
pub use crate::vasp::Vasp;
pub use crate::gaussian::Gaussian;
pub use crate::null::Null;

pub use crate::repl::repl_enter_main;
