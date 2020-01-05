// mods

pub mod mopac;
pub mod parser;
pub mod siesta;

// trait

use gosh_core::guts::prelude::*;
use gosh_models::ModelProperties;
use std::path::Path;

/// Common interface for model adaptors
pub trait ModelAdaptor {
    /// Parse the last entry of ModelProperties from a calculation output file
    ///
    /// # Return
    ///
    /// - ModelProperties, the calculated properties, including energy, forces, ...
    fn parse_last<P: AsRef<Path>>(&self, outfile: P) -> Result<ModelProperties>;

    /// Parse all properties in multi-step calculation, sush as optimization or
    /// multi-molecule batch calculation.
    ///
    /// # Return
    ///
    /// - a list of ModelProperties
    fn parse_all<P: AsRef<Path>>(&self, outfile: P) -> Result<Vec<ModelProperties>>;
}

// cli

use gosh_core::*;

use guts::cli::*;
use guts::fs::*;
use structopt::*;

/// Read alculated results, and format them as standard external model results.
#[derive(Debug, StructOpt)]
struct Cli {
    /// calculated output file
    #[structopt(parse(from_os_str))]
    outfile: PathBuf,

    /// Parse all result entries found in the output
    #[structopt(short = "a", long = "all")]
    all: bool,
}

pub fn enter_main<A: ModelAdaptor>(app: A) -> CliResult {
    let args = Cli::from_args();
    setup_logger();

    let outfile = &args.outfile;
    if args.all {
        for d in app.parse_all(outfile)? {
            if d.is_empty() {
                panic!("No data extracted from: {:?}", outfile);
            }
            println!("{:}", d);
        }
    } else {
        let d = app.parse_last(outfile)?;
        if d.is_empty() {
            panic!("No data extracted from: {:?}", outfile);
        }
        println!("{:}", d);
    }

    Ok(())
}

// pub

pub use crate::mopac::Mopac;
pub use crate::siesta::Siesta;
