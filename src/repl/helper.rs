// [[file:../../adaptors.note::e0fe07d2][e0fe07d2]]
use super::*;
use crate::parser::Cmd;

use rustyline::completion::{FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::Context;
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

#[derive(Helper, Highlighter, Validator)]
pub struct MyHelper {
    completer: FilenameCompleter,
    colored_prompt: String,
}

impl rustyline::completion::Completer for MyHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Result<(usize, Vec<Pair>), ReadlineError> {
        if Cmd::suitable_for_path_complete(line) {
            self.completer.complete(line, pos, ctx)
        } else {
            let commands = Cmd::get_subcommands();
            let pairs = commands
                .into_iter()
                .filter_map(|x| {
                    if x.starts_with(line) {
                        new_candidate(&x).into()
                    } else {
                        None
                    }
                })
                .collect();
            Ok((0, pairs))
        }
    }
}

impl MyHelper {
    pub fn new() -> Self {
        Self {
            completer: FilenameCompleter::new(),
            colored_prompt: "".to_owned(),
        }
    }
}

// FIXME: cannot be derived using rustyline_derive
impl rustyline::hint::Hinter for MyHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        None
    }
}

fn new_candidate(x: &str) -> Pair {
    Pair {
        display: x.into(),
        replacement: x.into(),
    }
}
// e0fe07d2 ends here
