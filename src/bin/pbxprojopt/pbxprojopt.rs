//! .pbxproject file optimizer

pub mod arguments;
pub mod lexer;
pub mod plist;

use std::fs::read_to_string;
use std::process::exit;

use ansi_term::Style;
use clap::Parser as ClapParser;
use pest::Parser as PestParser;
use pest::iterators::Pair;

use lexer::PBXProjLexer;

/// Recursively print all matched grammar rules and associated text.
fn visualize_matched_grammar_rule(pair: &Pair<lexer::Rule>, offset: usize) -> bool {
    let span = pair.clone().as_span();
    let inner_pairs = pair.clone().into_inner();

    // Print the rule name and the start/end pos if it's terminal
    let rule_name: String = format!("{:?}", pair.as_rule());
    let rule_name_style: Style = Style::new().bold();
    let text_span_style: Style = Style::new().dimmed();
    let arrow: &str = if offset != 0 { "->" } else { "" };
    let text_span: String = if inner_pairs.len() == 0 {
        format!("<{},{}>", span.start(), span.end())
    } else { String::new() };
    print!(
        "{}{}{}", arrow, rule_name_style.paint(&rule_name), text_span_style.paint(&text_span)
    );

    // If the rule terminates, also print the corresponding text
    if inner_pairs.len() == 0 {
        print!(": {}", span.as_str().escape_debug());
        return true;
    }

    // Print all subtokens
    let printed: usize = arrow.len() + rule_name.len() + text_span.len();
    let mut first: bool = true;
    for inner_pair in inner_pairs {

        // Vertically align successive subtokens
        if first {
            first = false;
        } else {
            print!("{}", " ".repeat(printed + offset));
        }

        let need_newline: bool = visualize_matched_grammar_rule(&inner_pair, printed + offset);
        if need_newline {
            println!("");
        }
    }

    return false;
}

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
        visualize_matched_grammar_rule(&pair, 0);
    }
}

// eof