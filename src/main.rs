use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use std::env;

fn main() -> Result<()> {
    let matches = Command::new("GitHub Actions Badge")
        .version("0.1.0")
        .author("YutaGoto <sample@example.com>")
        .about("Generate GitHub Actions Badge for Markdown")
        .arg(
            Arg::new("withlink")
                .long("with-link")
                .help("Generate link to GitHub Actions with action URL"),
        )
        .get_matches();

    let withlink = matches.is_present("withlink");

    let github_username = match env::var("GITHUB_USERNAME") {
        Ok(val) => val,
        Err(_) => return Err(anyhow!("Not set GITHUB_USERNAME")),
    };

    let current_dirs_result = env::current_dir()?;
    let current_dirs = current_dirs_result.to_str().unwrap();
    let mut current_dirs_vec = current_dirs.split('/').collect::<Vec<&str>>();
    current_dirs_vec.reverse();
    let current_dir = current_dirs_vec[0];

    let result = std::fs::read_dir(".github/workflows");
    let files = match result {
        Ok(files) => files,
        Err(err) => {
            return Err(err.into());
        }
    };

    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.ends_with(".yml") {
            if withlink {
                println!("[![action](https://github.com/{github_username}/{repo_name}/actions/workflows/{action_file}/badge.svg)](https://github.com/{github_username}/{repo_name}/actions)", github_username=github_username, repo_name=current_dir, action_file=file_name);
            } else {
                println!("![action](https://github.com/{github_username}/{repo_name}/actions/workflows/{action_file}/badge.svg)", github_username=github_username, repo_name=current_dir, action_file=file_name);
            }
        }
    }

    Ok(())
}
