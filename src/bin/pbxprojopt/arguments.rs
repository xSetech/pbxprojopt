//! CLI arguments

use std::path::PathBuf;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// .pbxproject file
    #[arg(short, long)]
    pub filename: PathBuf,

}

// eof