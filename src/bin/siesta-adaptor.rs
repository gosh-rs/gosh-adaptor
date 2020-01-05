// siesta-adaptor.rs
// :PROPERTIES:
// :header-args: :tangle src/bin/siesta-adaptor.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*siesta-adaptor.rs][siesta-adaptor.rs:1]]
use guts::cli::*;
use guts::prelude::*;

fn main() -> CliResult {
    let siesta = gosh_adaptors::Siesta();
    gosh_adaptors::enter_main(siesta)
}
// siesta-adaptor.rs:1 ends here
