// [[file:../adaptors.note::9afa51a4][9afa51a4]]
use gosh_core::*;

use gut1::cli::*;
use gut::fs::*;
use gut::prelude::*;
use structopt::*;

use gosh_adaptor::*;

/// Read calculated results, and format them as standard external model results.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbosity,

    /// Chemical model, possible values: mopac, siesta, vasp, gulp.
    chemical_model: String,

    /// calculated output file
    #[structopt(parse(from_os_str))]
    outfile: PathBuf,

    /// Parse all result entries found in the output
    #[structopt(short = "a", long = "all")]
    all: bool,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    args.verbose.setup_logger();

    let outfile = &args.outfile;
    info!(
        "parse computed file {:?} using model {:?}",
        outfile, &args.chemical_model
    );
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
        "null" => {
            let app = Null();
            parse(app, args.all, outfile)?;
        }
        _ => todo!(),
    }

    Ok(())
}

fn parse<M: ModelAdaptor>(app: M, all: bool, outfile: &Path) -> Result<()> {
    if all {
        info!("Parsing all structure entries ...");
        for d in app.parse_all(outfile).context("parse all failure")? {
            if d.is_empty() {
                bail!("No data extracted from: {:?}", outfile);
            }
            println!("{:}", d);
        }
    } else {
        info!("Parsing the last structure entry ...");
        let d = app.parse_last(outfile).context("parse last failure")?;
        if d.is_empty() {
            bail!("No data extracted from: {:?}", outfile);
        }
        println!("{:}", d);
    }

    Ok(())
}
// 9afa51a4 ends here
