use std::time::Instant;

mod job_type;

use self::job_type::{Executable, JobType};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    jobs: Vec<JobType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Jobs {
    name: String,
    run: String,
}

pub fn run_pipeline(pipeline: &Pipeline) -> Result<(), Box<dyn std::error::Error>> {
    for job in &pipeline.jobs {
        let start = Instant::now();

        job.execute()?;
        let duration = start.elapsed();
        println!("Time elapsed in executing job is: {:?}", duration);
    }

    Ok(())
}
