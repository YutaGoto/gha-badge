use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use std::env;

#[allow(unused_must_use)]
fn write_badge_text(
    github_username: &str,
    file_name: &str,
    current_dir: &str,
    withlink: bool,
    mut writer: impl std::io::Write,
) {
    if file_name.ends_with(".yml") {
        if withlink {
            let s = format!("[![action](https://github.com/{github_username}/{repo_name}/actions/workflows/{action_file}/badge.svg)](https://github.com/{github_username}/{repo_name}/actions)", github_username=github_username, repo_name=current_dir, action_file=file_name);
            writeln!(writer, "{}", s);
        } else {
            let s = format!("![action](https://github.com/{github_username}/{repo_name}/actions/workflows/{action_file}/badge.svg)", github_username=github_username, repo_name=current_dir, action_file=file_name);
            writeln!(writer, "{}", s);
        }
    }
}

fn main() -> Result<()> {
    let mut matches = Command::new("GitHub Actions Badge")
        .version("0.1.0")
        .author("YutaGoto <sample@example.com>")
        .about("Generate GitHub Actions Badge for Markdown")
        .arg(
            Arg::new("withlink")
                .long("with-link")
                .help("Generate link to GitHub Actions with action URL"),
        )
        .arg(
            Arg::new("githubname")
                .long("github-name")
                .short('n')
                .takes_value(true)
                .help("GitHub user name")
                .required(false),
        )
        .get_matches();

    let withlink = matches.contains_id("withlink");
    let github_name = matches
        .remove_one::<String>("githubname").unwrap_or_default();

    let github_username = if github_name.is_empty() {
        match env::var("GITHUB_USERNAME") {
            Ok(val) => val,
            Err(_) => return Err(anyhow!("Not set GITHUB_USERNAME")),
        }
    } else {
        github_name
    };

    let current_dir_result = env::current_dir()?;
    let mut dirs_vec = current_dir_result
        .to_str()
        .unwrap()
        .split('/')
        .collect::<Vec<&str>>();
    dirs_vec.reverse();
    let current_dir = dirs_vec[0];

    let result = std::fs::read_dir(".github/workflows");
    let files = match result {
        Ok(files) => files,
        Err(_) => {
            return Err(anyhow!("Not found .github/workflows"));
        }
    };

    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        write_badge_text(
            &github_username,
            file_name,
            current_dir,
            withlink,
            &mut std::io::stdout(),
        );
    }

    Ok(())
}

#[test]
fn test_write_badge_text_with_link() {
    let mut writer = Vec::new();
    write_badge_text("sample", "test.yml", "test", true, &mut writer);
    let s = String::from_utf8(writer).unwrap();
    assert_eq!(s, "[![action](https://github.com/sample/test/actions/workflows/test.yml/badge.svg)](https://github.com/sample/test/actions)\n");
}

#[test]
fn test_write_badge_text_without_link() {
    let mut writer = Vec::new();
    write_badge_text("sample", "test.yml", "test", false, &mut writer);
    let s = String::from_utf8(writer).unwrap();
    assert_eq!(
        s,
        "![action](https://github.com/sample/test/actions/workflows/test.yml/badge.svg)\n"
    );
}
