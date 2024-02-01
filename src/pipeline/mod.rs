use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    jobs: Vec<Jobs>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Jobs {
    name: String,
    run: String,
}

pub fn run_pipeline(pipeline: &Pipeline) -> Result<(), Box<dyn std::error::Error>> {
    for job in &pipeline.jobs {
        let start = Instant::now();
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(&job.run);

        let output = cmd.output()?;
        println!(
            "---- Step<{}> ----- \n{}----",
            job.name,
            String::from_utf8(output.stdout)?
        );

        let duration = start.elapsed();
        println!("Time elapsed for job {} - {:?}", job.name, duration)
    }

    Ok(())
}
