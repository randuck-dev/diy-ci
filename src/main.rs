use std::{env, os};

use clap::{builder::Str, Parser};
use serde::{Deserialize, Serialize};

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

    let pipeline: Pipeline = serde_yaml::from_reader(file)?;

    let mut cmd = std::process::Command::new("bash");
    cmd.arg("-c").arg(&pipeline.jobs[0].run);

    let output = cmd.output()?;
    println!("Output:\n----\n{}----", String::from_utf8(output.stdout)?);

    Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Pipeline {
    jobs: Vec<Jobs>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Jobs {
    name: String,
    run: String,
}
