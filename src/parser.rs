// [[file:../adaptors.note::70d3dbdb][70d3dbdb]]
use super::*;
// 70d3dbdb ends here

// [[file:../adaptors.note::724d9a95][724d9a95]]
use gut::cli::*;

#[derive(Parser, Debug)]
#[clap(disable_help_subcommand = true)]
pub enum Cmd {
    /// Quit shell.
    #[clap(name = "quit", alias = "q", alias = "exit")]
    Quit {},

    /// Show available commands.
    #[clap(name = "help", alias = "h", alias = "?")]
    Help {},

    /// Load file from `path` for processing.
    ///
    /// WARNING: load very large file may lead to out of memory error.
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
    Println { text: String },

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
        columns: Option<String>,

        /// Writes selection into external command through a pipe.
        #[clap(long)]
        pipe: Option<String>,
    },
}
// 724d9a95 ends here

// [[file:../adaptors.note::a252f98f][a252f98f]]
// use crate::skim::Glance;

#[derive(Debug, Default, Clone)]
pub struct Action {
    glance: Option<Glance>,
}

impl Action {
    fn get_glance(&mut self) -> Result<&mut Glance> {
        if let Some(glance) = self.glance.as_mut() {
            Ok(glance)
        } else {
            bail!("File not loaded yet!");
        }
    }
}

impl gosh_repl::Actionable for Action {
    type Command = Cmd;

    /// Take action on gosh-parser commands. Return Ok(true) will exit shell
    /// loop.
    fn act_on(&mut self, cmd: &Cmd) -> Result<bool> {
        match cmd {
            Cmd::Quit {} => return Ok(true),
            Cmd::Help {} => {
                let mut app = Cmd::command();
                app.print_help();
                println!("");
            }
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
            Cmd::PrintSelection { columns, pipe } => {
                let glance = self.get_glance()?;
                let x = if let Some(col_spec) = columns {
                    glance.print_column_selection(col_spec)?
                } else {
                    glance.print_selection()
                };
                if let Some(cmdline) = pipe {
                    if let Some(command) = shlex::split(cmdline) {
                        let x = match command.as_slice() {
                            [command, args @ ..] => gut::cli::duct::cmd(command, args).stdin_bytes(x).read()?,
                            [command] => gut::cli::duct::cmd!(command).stdin_bytes(x).read()?,
                            _ => {
                                bail!("invalid cmdline: {}", cmdline);
                            }
                        };
                        println!("{}", x);
                    }
                } else {
                    println!("{}", x);
                }
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

        Ok(false)
    }
}
// a252f98f ends here
