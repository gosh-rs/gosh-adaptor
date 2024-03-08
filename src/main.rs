// [[file:../adaptors.note::9afa51a4][9afa51a4]]
use gosh_core::*;

use gut::cli::*;
use gut::fs::*;
use gut::prelude::*;

use gosh_adaptor::*;

macro_rules! process_app {
    ($app:expr, $args:expr) => {{
        let app = $app;
        if let Some(Task::Dump { pqfile }) = &$args.command {
            let mps = app.parse_all(&$args.outfile)?;
            mps.write_parquet(pqfile)?;
        } else {
            parse(app, $args.all, &$args.outfile)?;
        }
    }};
}

#[derive(Subcommand, Debug)]
enum Task {
    /// Parsed computed results in `outfile` and write to `pqfile` in parquet
    /// format.
    ///
    /// # Example
    /// * gosh-adaptor OUTCAR dump calculated.pq
    Dump { pqfile: PathBuf },
}

/// Read calculated results, and format them as standard external model results.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    /// Chemical model, possible values: mopac, siesta, vasp, gulp.
    chemical_model: String,

    /// calculated output file
    outfile: PathBuf,

    /// Parse all result entries found in the output
    #[arg(short, long)]
    all: bool,

    #[command(subcommand)]
    command: Option<Task>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    args.verbose.setup_logger();

    let outfile = &args.outfile;
    info!(
        "parse computed file {:?} using model {:?}",
        outfile, &args.chemical_model
    );
    match args.chemical_model.as_str() {
        "mopac" => process_app!(Mopac(), args),
        "siesta" => process_app!(Siesta(), args),
        "gulp" => process_app!(Gulp(), args),
        "vasp" => process_app!(Vasp(), args),
        "gaussian" => process_app!(Gaussian(), args),
        "null" => process_app!(Null(), args),
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
