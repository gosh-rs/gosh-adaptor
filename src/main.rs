// [[file:../adaptors.note::5c844c95][5c844c95]]
use gosh_core::*;

use gut::cli::*;
use gut::fs::*;
use gut::prelude::*;

use gosh_adaptor::*;
// 5c844c95 ends here

// [[file:../adaptors.note::047a05ac][047a05ac]]
#[derive(Parser, Debug)]
struct CollectParquet {
    /// One or more computed files to be parsed. If not specified,
    /// file names will be read from stdin, so use in combination with
    /// `find` or `fd`.
    outfiles: Option<Vec<PathBuf>>,

    #[arg(short = 'o')]
    /// The data file to be wrote parsed results (energy, forces,
    /// positions, ...), in parquet format.
    pqfile: PathBuf,

    /// Parse the last result entry found in the output. By default,
    /// collect all valid results.
    #[arg(short, long)]
    last: bool,
}

#[derive(Parser, Debug)]
struct ParseComputed {
    /// The computed file to be parsed.
    outfile: PathBuf,

    /// Parse all result entries found in the output. By default, the
    /// final result will be parsed.
    #[arg(short, long)]
    all: bool,
}
// 047a05ac ends here

// [[file:../adaptors.note::232fa607][232fa607]]
fn read_paths_from_stdin() -> Vec<PathBuf> {
    info!("Read computed output files from stdin");
    std::io::stdin()
        .lines()
        .filter_map(|line| line.map(|line| PathBuf::from(&line)).ok())
        .collect()
}

impl CollectParquet {
    fn process(&self, app: impl ModelAdaptor) -> Result<()> {
        let mut all_mps = vec![];
        let outfiles = self
            .outfiles
            .as_ref()
            .cloned()
            .unwrap_or_else(|| read_paths_from_stdin());
        info!("I will collect results from {} computed output files", outfiles.len());
        for outfile in &outfiles {
            if self.last {
                let mp = app.parse_last(outfile)?;
                all_mps.push(mp);
            } else {
                let mps = app.parse_all(outfile)?;
                all_mps.extend(mps);
            }
        }
        all_mps.write_parquet(&self.pqfile)?;

        Ok(())
    }
}

impl ParseComputed {
    fn process(&self, app: impl ModelAdaptor) -> Result<()> {
        if self.all {
            info!("Parsing all structure entries ...");
            for d in app.parse_all(&self.outfile).context("parse all failure")? {
                if d.is_empty() {
                    bail!("No data extracted from: {:?}", self.outfile);
                }
                println!("{:}", d);
            }
        } else {
            info!("Parsing the last structure entry ...");
            let d = app.parse_last(&self.outfile).context("parse last failure")?;
            if d.is_empty() {
                bail!("No data extracted from: {:?}", self.outfile);
            }
            println!("{:}", d);
        }

        Ok(())
    }
}
// 232fa607 ends here

// [[file:../adaptors.note::3416c6c6][3416c6c6]]
macro_rules! process_app {
    ($chemical_model:expr, $task:expr) => {{
        info!("Parsing computed file using model {:?}", $chemical_model);
        match $chemical_model.as_str() {
            "mopac" => {
                let app = Mopac();
                $task.process(app)?;
            }
            "siesta" => {
                let app = Siesta();
                $task.process(app)?;
            }
            "gulp" => {
                let app = Gulp();
                $task.process(app)?;
            }
            "vasp" => {
                let app = Vasp();
                $task.process(app)?;
            }
            "gaussian" => {
                let app = Gaussian();
                $task.process(app)?;
            }
            "null" => {
                let app = Null();
                $task.process(app)?;
            }
            "ckpts" => {
                let app = Ckpts();
                $task.process(app)?;
            }
            _ => todo!(),
        }
    }};
}
// 3416c6c6 ends here

// [[file:../adaptors.note::fe21c9aa][fe21c9aa]]
/// Read calculated results, and format them as standard external model results.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
// #[clap(args_conflicts_with_subcommands = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    task: Task,

    /// The chemical model for the computed output file. Supported
    /// models include vasp, gaussian, siesta, gulp, ckpts (i.e, checkpointed db)
    #[clap(required = true)]
    chemical_model: String,
}

#[derive(Subcommand, Debug)]
enum Task {
    /// Collect parsed results in `outfile` and write these data to
    /// `pqfile` in parquet format.
    ///
    /// # Example
    ///
    /// > gosh-adaptor vasp collect OUTCAR -o calculated.pq
    Collect(CollectParquet),

    /// Print parse computed results in model properties format from output.
    ///
    /// # Example
    ///
    /// > gosh-adaptor vasp parse OUTCAR --all
    Parse(ParseComputed),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.verbose.setup_logger();

    let chemical_model = &cli.chemical_model;
    match cli.task {
        Task::Collect(task) => {
            process_app!(chemical_model, &task)
        }
        Task::Parse(task) => {
            process_app!(chemical_model, &task)
        }
    }

    Ok(())
}
// fe21c9aa ends here
