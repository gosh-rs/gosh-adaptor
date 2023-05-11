// [[file:../adaptors.note::dc949951][dc949951]]
use super::*;
use crate::parser::Action;
use crate::parser::Cmd;

use gosh_repl::Interpreter as Interp;
use gut::cli::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct GoshParser {
    /// Execute gosh-parser script
    #[clap(short = 'x')]
    script_file: Option<PathBuf>,

    #[clap(flatten)]
    verbose: Verbosity,
}

pub fn repl_enter_main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let action = Action::default();
    // entry shell mode or subcommands mode
    if args.len() > 1 {
        let args = GoshParser::parse();
        args.verbose.setup_logger();

        if let Some(script_file) = &args.script_file {
            info!("Execute script file: {:?}", script_file);
            Interp::new(action).interpret_script_file(script_file)?;
        } else {
            info!("Reading batch script from stdin ..");
            use std::io::{self, Read};

            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Interp::new(action).interpret_script(&buffer)?;
        }
    } else {
        Interp::new(action).with_prompt("gosh-parser> ").run()?;
    }

    Ok(())
}
// dc949951 ends here
