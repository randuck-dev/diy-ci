use std::{env, fs::File, os};

use clap::Parser;
use pipeline::{run_pipeline, Pipeline};
mod pipeline;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::consts::OS == "windows" {
        println!("Windows not supported");
        return Ok(());
    }

    let args = Args::parse();
    let file = std::fs::File::open(args.file)?;

    let pipeline = serde_yaml::from_reader::<File, Pipeline>(file)?;

    run_pipeline(&pipeline)?;

    Ok(())
}
