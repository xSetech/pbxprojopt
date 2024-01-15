//! .pbxproject file optimizer

pub mod arguments;
pub mod lexer;

use std::fs::read_to_string;
use std::process::exit;

use clap::Parser as ClapParser;
use pest::Parser as PestParser;

use lexer::PBXProjLexer;

fn view(content: &String) {
    let r = PBXProjLexer::parse(
        lexer::Rule::File,
        content,
    );
    println!("{:?}", r);
}

fn main() {
    let args: arguments::Args = arguments::Args::parse();
    if !args.filename.exists() {
        eprintln!("error: file not found: {}", args.filename.display());
        exit(1);
    }
    let content: String = read_to_string(args.filename).unwrap();
    view(&content);
}

// eof