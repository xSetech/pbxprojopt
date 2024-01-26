// SPDX-License-Identifier: GPL-3.0-or-later

//! .pbxproject file optimizer

pub mod arguments;
pub mod lexer;
pub mod plist;

use std::fs::read_to_string;
use std::process::exit;

use clap::Parser as ClapParser;
use pest::Parser as PestParser;

use lexer::{PBXProjLexer, visualize_matched_grammar_rule};

fn main() {
    let args: arguments::Args = arguments::Args::parse();
    if !args.filename.exists() {
        eprintln!("error: file not found: {}", args.filename.display());
        exit(1);
    }
    let content: String = read_to_string(args.filename).unwrap();
    let lexer_result = PBXProjLexer::parse(
        lexer::Rule::File,
        &content,
    );
    if let Err(failed_grammar_rule) = lexer_result {
        eprintln!("Failed to parse the file's content: {:?}", failed_grammar_rule);
        exit(1);
    }
    let lexer_result = lexer_result.unwrap();
    for pair in lexer_result {
        visualize_matched_grammar_rule(&pair, 0, &mut None, true);
    }
}

// eof