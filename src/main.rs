// [[file:../adaptors.note::5c844c95][5c844c95]]
use gosh_core::*;

use gut::cli::*;
use gut::fs::*;
use gut::prelude::*;

use gosh_adaptor::*;
// 5c844c95 ends here

// [[file:../adaptors.note::*base][base:1]]
#[derive(Subcommand, Debug)]
enum Task {
    /// Collect parsed results in `outfile` and write these data to
    /// `pqfile` in parquet format.
    ///
    /// # Example
    /// * gosh-adaptor -m vasp collect OUTCAR -o calculated.pq
    Collect {
        /// The chemical model for the computed output file. Supported
        /// models include vasp, gaussian, siesta, gulp
        chemical_model: String,

        /// One or more computed files to be parsed.
        Outfiles: Vec<PathBuf>,

        #[arg(short = 'o')]
        /// The data file to be wrote parsed results (energy, forces,
        /// positions, ...), in parquet format.
        pqfile: PathBuf,
    },

    /// Parse computed results from output
    Parse {
        /// The chemical model for the computed output file. Supported
        /// models include vasp, gaussian, siesta, gulp
        chemical_model: String,

        /// The computed file to be parsed.
        outfile: PathBuf,

        /// Parse all result entries found in the output
        #[arg(short, long)]
        all: bool,
    },
}
// base:1 ends here

// [[file:../adaptors.note::232fa607][232fa607]]
fn collect_all_computed_to_parquet(app: impl ModelAdaptor, outfiles: &[PathBuf], pqfile: &Path) -> Result<()> {
    let mut all_mps = vec![];
    for outfile in outfiles {
        let mps = app.parse_all(outfile)?;
        all_mps.extend(mps);
    }
    all_mps.write_parquet(pqfile)?;

    Ok(())
}

fn parse_computed_from(app: impl ModelAdaptor, all: bool, outfile: &Path) -> Result<()> {
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
// 232fa607 ends here

// [[file:../adaptors.note::*cli][cli:1]]
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

/// Read calculated results, and format them as standard external model results.
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    command: Task,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.verbose.setup_logger();

    match cli.task {
        Task::Collect {} => {
            //
        }
        Task::Parse {} => {
            //
        }
    }

    let outfile = &cli.outfile;
    info!(
        "parse computed file {:?} using model {:?}",
        outfile, &cli.chemical_model
    );
    match cli.chemical_model.as_str() {
        "mopac" => process_app!(Mopac(), cli),
        "siesta" => process_app!(Siesta(), cli),
        "gulp" => process_app!(Gulp(), cli),
        "vasp" => process_app!(Vasp(), cli),
        "gaussian" => process_app!(Gaussian(), cli),
        "null" => process_app!(Null(), cli),
        _ => todo!(),
    }

    Ok(())
}
// cli:1 ends here
