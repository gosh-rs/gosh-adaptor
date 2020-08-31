// mods

mod parser;

mod null;

mod gaussian;
mod gulp;
mod mopac;
mod siesta;
mod vasp;

// trait

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

// pub

pub use crate::gulp::Gulp;
pub use crate::mopac::Mopac;
pub use crate::siesta::Siesta;
pub use crate::vasp::Vasp;
pub use crate::gaussian::Gaussian;
