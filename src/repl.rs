// [[file:../adaptors.note::*mods][mods:1]]
mod helper;
// mods:1 ends here

// [[file:../adaptors.note::2d224f12][2d224f12]]
use super::*;

use crate::parser::Action;
use crate::parser::Cmd;
// 2d224f12 ends here

// [[file:../adaptors.note::845cbd1e][845cbd1e]]
use rustyline::Editor;

/// An shell-like REPL interpreter.
pub struct Interpreter<A: Actionable> {
    prompt: String,
    history_file: Option<PathBuf>,

    editor: Editor<helper::MyHelper>,
    action: A,
}
// 845cbd1e ends here

// [[file:../adaptors.note::aa47dc5f][aa47dc5f]]
impl<A: Actionable> Interpreter<A> {
    /// Interpret one line.
    fn continue_interpret_line(&mut self, line: &str) -> bool {
        if let Some(mut args) = shlex::split(line) {
            assert!(args.len() >= 1);
            args.insert(0, "gosh-parser".into());

            match A::try_parse_from(&args) {
                // apply subcommand
                Ok(x) => match self.action.act_on(&x) {
                    Ok(exit) => {
                        if exit {
                            return false;
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                    }
                },
                // show subcommand usage
                Err(e) => println!("{:}", e),
            }
            true
        } else {
            dbg!(line);
            false
        }
    }
}

fn create_readline_editor() -> Editor<helper::MyHelper> {
    use rustyline::{ColorMode, CompletionType, Config, Editor};

    let config = Config::builder()
        .color_mode(rustyline::ColorMode::Enabled)
        .completion_type(CompletionType::Fuzzy)
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .max_history_size(1000)
        .build();

    let mut rl = Editor::with_config(config);
    let h = self::helper::MyHelper::new();
    rl.set_helper(Some(h));
    rl
}

impl<A: Actionable> Interpreter<A> {
    fn continue_read_eval_print(&mut self) -> bool {
        match self.editor.readline(&self.prompt) {
            Err(rustyline::error::ReadlineError::Eof) => false,
            Ok(line) => {
                let line = line.trim();
                if !line.is_empty() {
                    self.editor.add_history_entry(line);
                    self.continue_interpret_line(&line)
                } else {
                    true
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }
}
// aa47dc5f ends here

// [[file:../adaptors.note::360871b3][360871b3]]
impl<A: Actionable> Interpreter<A> {
    fn load_history(&mut self) -> Result<()> {
        if let Some(h) = self.history_file.as_ref() {
            self.editor.load_history(h).context("no history")?;
        }
        Ok(())
    }

    fn save_history(&mut self) -> Result<()> {
        if let Some(h) = self.history_file.as_ref() {
            self.editor.save_history(h).context("write history file")?;
        }
        Ok(())
    }
}
// 360871b3 ends here

// [[file:../adaptors.note::05b99d70][05b99d70]]
impl<A: Actionable> Interpreter<A> {
    fn interpret_script(&mut self, script: &str) -> Result<()> {
        let lines = script.lines().filter(|s| !s.trim().is_empty());
        for line in lines {
            debug!("Execute: {:?}", line);
            if !self.continue_interpret_line(&line) {
                break;
            }
        }

        Ok(())
    }

    fn interpret_script_file(&mut self, script_file: &Path) -> Result<()> {
        let s = gut::fs::read_file(script_file)?;
        self.interpret_script(&s)?;
        Ok(())
    }
}
// 05b99d70 ends here

// [[file:../adaptors.note::f3bcb018][f3bcb018]]
pub trait Actionable {
    type Command;

    /// Act on `cmd`
    fn act_on(&mut self, cmd: &Self::Command) -> Result<bool>;

    /// parse Command from shell line input.
    fn try_parse_from<I, T>(iter: I) -> Result<Self::Command>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone;
}

pub trait HelpfulCommand {
    fn get_subcommands() -> Vec<String>;
    fn suitable_for_path_complete(line: &str) -> bool;
}

impl<A: Actionable> Interpreter<A> {
    pub fn new(action: A) -> Self {
        Self {
            prompt: "> ".to_string(),
            editor: create_readline_editor(),
            history_file: None,
            action,
        }
    }

    /// Set absolute path to history file for permanently storing command history.
    pub fn with_history_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        let p = path.into();
        self.history_file = Some(p);
        self
    }

    /// Set prompting string for REPL.
    pub fn with_prompt(mut self, s: &str) -> Self {
        self.prompt = s.into();
        self
    }

    pub fn start_repl(&mut self) -> Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        println!("This is the interactive parser, version {}.", version);
        println!("Enter \"help\" or \"?\" for a list of commands.");
        println!("Press Ctrl-D or enter \"quit\" or \"q\" to exit.");
        println!("");

        let _ = self.load_history();
        while self.continue_read_eval_print() {
            trace!("excuted one loop");
        }
        self.save_history()?;

        Ok(())
    }
}
// f3bcb018 ends here

// [[file:../adaptors.note::dc949951][dc949951]]
use gut::cli_clap::*;
use std::path::PathBuf;

use clap::IntoApp;
use clap::Parser;

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
            Interpreter::new(action).interpret_script_file(script_file)?;
        } else {
            info!("Reading batch script from stdin ..");
            use std::io::{self, Read};

            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Interpreter::new(action).interpret_script(&buffer)?;
        }
    } else {
        Interpreter::new(action).with_prompt("gosh-parser> ").start_repl()?;
    }

    Ok(())
}
// dc949951 ends here
