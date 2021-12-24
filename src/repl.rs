// [[file:../adaptors.note::*mods][mods:1]]
mod helper;
// mods:1 ends here

// [[file:../adaptors.note::2d224f12][2d224f12]]
use super::*;
// 2d224f12 ends here

// [[file:../adaptors.note::845cbd1e][845cbd1e]]
use crate::skim::Glance;
use rustyline::Editor;

const PROMPT: &str = "gosh-parser> ";

struct Interpreter {
    history_file: PathBuf,
    editor: Editor<helper::MyHelper>,
    glance: Option<Glance>,
}
// 845cbd1e ends here

// [[file:../adaptors.note::aa47dc5f][aa47dc5f]]
impl Interpreter {
    fn new() -> Self {
        Self {
            editor: create_readline_editor(),
            glance: None,
            history_file: get_history_file(),
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

impl Interpreter {
    fn continue_read_eval_print(&mut self) -> bool {
        match self.editor.readline(PROMPT) {
            Err(rustyline::error::ReadlineError::Eof) => false,
            Ok(line) => {
                let line = line.trim();
                if !line.is_empty() {
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

    fn start_repl(&mut self) -> Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        println!("This is the interactive parser, version {}.", version);
        println!("Enter \"help\" or \"?\" for a list of commands.");
        println!("Press Ctrl-D or enter \"quit\" or \"q\" to exit.");
        println!("");

        // let _ = self.load_history();
        while self.continue_read_eval_print() {
            trace!("excuted one loop");
        }
        // self.save_history()?;

        Ok(())
    }
}
// aa47dc5f ends here

// [[file:../adaptors.note::360871b3][360871b3]]
fn get_history_file() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join(".gosh-parser.history")
}

impl Interpreter {
    fn load_history(&mut self) -> Result<()> {
        self.editor.load_history(&self.history_file).context("no history")?;
        Ok(())
    }

    fn save_history(&mut self) -> Result<()> {
        self.editor
            .save_history(&self.history_file)
            .context("write gosh history file")?;
        Ok(())
    }
}
// 360871b3 ends here

// [[file:../adaptors.note::05b99d70][05b99d70]]
impl Interpreter {
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

// [[file:../adaptors.note::4ee77758][4ee77758]]
use clap::IntoApp;
use clap::Parser;

#[derive(Parser, Debug)]
enum Cmd {
    /// Quit go shell.
    #[clap(name = "quit", alias = "q", alias = "exit")]
    Quit {},

    /// Show available commands.
    #[clap(name = "help", alias = "h", alias = "?")]
    Help {},

    /// Load file from `path` for skimming.
    #[clap(name = "load")]
    Load {
        #[clap(name = "FILENAME")]
        path: String,
    },

    /// Move cursor to line `line_num`
    #[clap(name = "goto-line")]
    GotoLine {
        #[clap(name = "LINE-NUMBER")]
        line_num: usize,

        /// Specify line range relative to current line (cursor position)
        #[clap(long)]
        relative: bool,
    },

    /// Display a line of `text`
    #[clap(name = "println")]
    Println {
        text: String,
    },

    /// Move cursor the line matching search `pattern`.
    #[clap(name = "search-forward")]
    SearchForward {
        #[clap(name = "PATTERN")]
        pattern: String,
    },

    /// Select a list of lines: 1-3 or 4
    #[clap(name = "select-lines")]
    SelectLines {
        #[clap(name = "LINE-SPECS")]
        specs: String,

        /// Specify line range relative to current line (cursor position)
        #[clap(long)]
        relative: bool,
    },

    /// Print selected lines
    #[clap(name = "print-selection")]
    PrintSelection {
        /// Print the text within selected columns.
        #[clap(long)]
        columns: Option<String>
    },
}

impl Interpreter {
    /// Interpret one line.
    fn continue_interpret_line(&mut self, line: &str) -> bool {
        if let Some(mut args) = shlex::split(line) {
            assert!(args.len() >= 1);
            args.insert(0, "gosh-parser".into());

            match Cmd::try_parse_from(&args) {
                // show subcommands
                Ok(Cmd::Help {}) => {
                    let mut app = Cmd::into_app();
                    app.print_help();
                    println!("");
                }
                // handle quit command first
                Ok(Cmd::Quit {}) => return false,
                // apply subcommand
                Ok(x) => {
                    if let Err(e) = self.apply_cmd(&x) {
                        eprintln!("{:?}", e);
                    }
                }
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
// 4ee77758 ends here

// [[file:../adaptors.note::46706bac][46706bac]]
impl Interpreter {
    fn get_glance(&mut self) -> Result<&mut Glance> {
        if let Some(glance) = self.glance.as_mut() {
            Ok(glance)
        } else {
            bail!("File not loaded yet!");
        }
    }

    fn apply_cmd(&mut self, cmd: &Cmd) -> Result<()> {
        match cmd {
            Cmd::Quit {} | Cmd::Help {} => {}
            Cmd::Load { path } => {
                self.glance = Glance::try_from_path(path.as_ref())?.into();
            }
            Cmd::GotoLine { line_num, relative } => {
                let glance = self.get_glance()?;
                if *relative {
                    glance.goto_line_relative(*line_num);
                } else {
                    glance.goto_line(*line_num);
                }
            }
            Cmd::SearchForward { pattern } => {
                let glance = self.get_glance()?;
                glance.search_forward(pattern)?;
            }
            Cmd::PrintSelection { columns } => {
                let glance = self.get_glance()?;
                let x = if let Some(col_spec) = columns {
                    glance.print_column_selection(col_spec)?
                } else {
                    glance.print_selection()
                };
                println!("{}", x);
            }
            Cmd::Println { text } => {
                println!("{}", text);
            }
            Cmd::SelectLines { specs, relative } => {
                let glance = self.get_glance()?;
                if *relative {
                    glance.select_lines_relative(specs);
                } else {
                    glance.select_lines(specs);
                }
            }
            o => {
                eprintln!("{:?}: not implemented yet!", o);
            }
        }

        Ok(())
    }
}
// 46706bac ends here

// [[file:../adaptors.note::9415b107][9415b107]]
impl self::helper::MyHelper {
    fn get_subcommands() -> Vec<String> {
        let app = Cmd::into_app();
        app.get_subcommands().map(|s| s.get_name().into()).collect()
    }

    fn suitable_for_path_complete(line: &str) -> bool {
        line.trim().starts_with("load")
    }
}
// 9415b107 ends here

// [[file:../adaptors.note::dc949951][dc949951]]
use gut::cli_clap::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Gosh {
    /// Execute gosh parser script
    #[clap(short = 'x')]
    script_file: Option<PathBuf>,

    #[clap(flatten)]
    verbose: Verbosity,
}

pub fn repl_enter_main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // entry shell mode or subcommands mode
    if args.len() > 1 {
        let args = Gosh::parse();
        args.verbose.setup_logger();

        if let Some(script_file) = &args.script_file {
            info!("Execute script file: {:?}", script_file);
            Interpreter::new().interpret_script_file(script_file)?;
        } else {
            info!("Reading batch script from stdin ..");
            use std::io::{self, Read};

            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Interpreter::new().interpret_script(&buffer)?;
        }
    } else {
        Interpreter::new().start_repl()?;
    }

    Ok(())
}
// dc949951 ends here
