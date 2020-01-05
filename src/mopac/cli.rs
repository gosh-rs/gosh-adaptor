// core

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*core][core:1]]
use std::path::PathBuf;

use structopt::*;

use gosh_core::*;

use guts::cli::*;

/// Read SIESTA calculated results, and format them as standard external model
/// results.
#[derive(Debug, StructOpt)]
struct Cli {
    /// MOPAC generated output file
    #[structopt(parse(from_os_str))]
    outfile: PathBuf,
}

pub fn enter_main() -> CliResult {
    // cli
    let args = Cli::from_args();
    setup_logger();

    // parse and print
    // let mp = super::model::get_siesta_results(&args.outfile)?;
    // println!("{}", mp);

    Ok(())
}
// core:1 ends here
