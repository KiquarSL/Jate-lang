use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    sources: Vec<PathBuf>,
}
