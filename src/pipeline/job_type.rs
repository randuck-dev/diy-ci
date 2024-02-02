use std::{env, fmt::format, path::Path};

use git2::{Cred, RemoteCallbacks};
use serde::{Deserialize, Serialize};

pub trait Executable {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JobType {
    CloneJob(CloneArgs),
    RunJob(RunArgs),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CloneArgs {
    url: String,
    name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RunArgs {
    name: String,
    run: String,
}

impl Executable for CloneArgs {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Cloning {}", self.url);
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            let private_key = format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap());
            let public_key = format!("{}/.ssh/id_ed25519.pub", env::var("HOME").unwrap());
            let passphrase = env::var("SSH_PASSPHRASE").unwrap();
            Cred::ssh_key(
                username_from_url.unwrap(),
                Some(Path::new(public_key.as_str())),
                Path::new(private_key.as_str()),
                Some(&passphrase),
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        let tmp_path = format!("/tmp/diy-ci/{}", self.name);
        builder.clone(&self.url, Path::new(tmp_path.as_str()))?;

        Ok(())
    }
}

impl Executable for RunArgs {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = std::process::Command::new("bash");
        cmd.arg("-c").arg(&self.run);

        let output = cmd.output()?;
        println!(
            "---- Step<{}> ----- \n{}----",
            self.name,
            String::from_utf8(output.stdout)?
        );

        Ok(())
    }
}

impl Executable for JobType {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        return match self {
            JobType::CloneJob(clone) => clone.execute(),
            JobType::RunJob(runnable) => runnable.execute(),
        };
    }
}
