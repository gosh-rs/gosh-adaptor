// mopac-adaptor.rs
// :PROPERTIES:
// :header-args: :comments org :tangle src/bin/mopac-adaptor.rs
// :END:

use gosh_core::guts::cli::*;

fn main() -> CliResult {
    let mopac = gosh_adaptors::Mopac();
    gosh_adaptors::enter_main(mopac)
}
