// main.rs
// :PROPERTIES:
// :header-args: :tangle src/main.rs
// :END:

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*main.rs][main.rs:1]]
use gosh_core::*;

use guts::cli::*;
use guts::fs::*;
use structopt::*;

use gosh_adaptors::*;

/// Read calculated results, and format them as standard external model results.
#[derive(Debug, StructOpt)]
struct Cli {
    /// Chemical model, possible values: mopac, siesta, vasp, gulp.
    chemical_model: String,

    /// calculated output file
    #[structopt(parse(from_os_str))]
    outfile: PathBuf,

    /// Parse all result entries found in the output
    #[structopt(short = "a", long = "all")]
    all: bool,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    setup_logger();

    let outfile = &args.outfile;
    match args.chemical_model.as_str() {
        "mopac" => {
            let app = Mopac();
            parse(app, args.all, outfile)?;
        }
        "siesta" => {
            let app = Siesta();
            parse(app, args.all, outfile)?;
        }
        "gulp" => {
            let app = Gulp();
            parse(app, args.all, outfile)?;
        }
        "vasp" => {
            let app = Vasp();
            parse(app, args.all, outfile)?;
        }
        "gaussian" => {
            let app = Gaussian();
            parse(app, args.all, outfile)?;
        }
        _ => todo!(),
    }

    Ok(())
}

fn parse<M: ModelAdaptor>(app: M, all: bool, outfile: &Path) -> CliResult {
    if all {
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
// main.rs:1 ends here
