use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate dirs;

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
            let s = format!("[![action](https://github.com/{github_username}/{current_dir}/actions/workflows/{file_name}/badge.svg)](https://github.com/{github_username}/{current_dir}/actions)");
            writeln!(writer, "{s}");
        } else {
            let s = format!("![action](https://github.com/{github_username}/{current_dir}/actions/workflows/{file_name}/badge.svg)");
            writeln!(writer, "{s}");
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<()> {
    let mut matches = Command::new("GitHub Actions Badge")
        .version("0.3.0")
        .author("YutaGoto <yutagoto@gmail.com>")
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
                .num_args(0..=1)
                .help("GitHub user name")
                .required(false),
        )
        .get_matches();

    let home_dir_buf = match dirs::home_dir() {
        Some(k) => k,
        None => return Err(anyhow!("error while load home dir")),
    };

    let home_dir = match home_dir_buf.to_str() {
        Some(d) => d,
        None => return Err(anyhow!("error to convert str")),
    };

    let mut read_file = home_dir.to_string();
    read_file.push_str("/.gitconfig");

    let mut config_user = "".to_string();

    if let Ok(lines) = read_lines(read_file) {
        for line in lines.into_iter().map_while(Result::ok) {
            if line.contains("name =") {
                config_user = line.trim().replace("name = ", "")
            }
        }
    }

    let withlink = matches.contains_id("withlink");
    let github_name = matches
        .remove_one::<String>("githubname")
        .unwrap_or_default();

    let github_username = if !config_user.is_empty() {
        config_user
    } else if github_name.is_empty() {
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
