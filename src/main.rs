use std::{
    env,
    fs::{copy, create_dir, create_dir_all, read_dir, remove_dir_all},
    io,
    path::{Path, PathBuf},
};

use clap::{App, Arg, ArgMatches};
use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use regex::Regex;

const REGEX_FILE_PATTERN: &str = r"^f_";

#[cfg(test)]
mod test {
    use regex::Regex;

    use crate::REGEX_FILE_PATTERN;

    #[test]
    fn test_regex() {
        let pattern = Regex::new(REGEX_FILE_PATTERN).unwrap();
        assert_eq!(pattern.is_match("f_00002f"), true);
        assert_eq!(pattern.is_match("index"), false);
    }
}

fn main() {
    let matches = get_cli_arguments();
    let apps = matches.value_of("application").unwrap().split('+');
    let pattern = Regex::new(matches.value_of("file-filter").unwrap())
        .expect("Failed to create Regex Pattern");

    apps.for_each(|app| {
        let mut path =
            PathBuf::from(dirs::config_dir().expect("Failed to get the config Directory"));

        match app {
            "discord" => {
                path.push("discord/Cache");
            }
            "guilded" => {
                path.push("guilded/Cache");
            }
            "vscode" => {
                path.push("Code/Cache");
            }
            "vscode-insider" => {
                path.push("Code - Insider/Cache");
            }
            "custom" => {
                if let Some(custom_path) = matches.value_of("input") {
                    path = PathBuf::from(custom_path);
                }
            }
            _ => {
                println!("[!] Unknown Application: {}", app);
            }
        }

        if path.exists() {
            let mut output = PathBuf::from(env::current_dir().unwrap());
            if let Some(outdir) = matches.value_of("output-directory") {
                output = PathBuf::from(outdir);
            } else {
                output.push("extracted");
            }
            output.push(app);
            create_dir_all(&output).expect("Failed to create the output Directory");
            let files =
                read_cache(&path, &pattern).expect("Failed to get read the cache Directory");

            let pb = ProgressBar::new(files.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{msg} {spinner:.green} [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}")
                    .progress_chars("#>-"),
            );
            pb.set_message(format!("{}", app));

            for file in files {
                copy_file(&file, &output);
                pb.inc(1)
            }
            pb.finish_with_message(format!("{} Done.", app));

            if matches.is_present("clear-cache") {
                remove_dir_all(&path).expect("Failed to clear the Cache");
            }
        } else {
            println!("[!] Cache Directory missing for: {}", app)
        }
    })
}

fn get_cli_arguments() -> ArgMatches {
    App::new("Cache Extractor")
	.version(env!("CARGO_PKG_VERSION"))
	.arg(
		Arg::new("output-directory")
			.about("Defines the output Directory for the extracted Files")
			.alias("output-dir")
			.short('o')
			.required(false)
			.takes_value(true),
	)
	.arg(
		Arg::new("clear-cache")
			.about("Clear the Cache after Extracting (Discord needs to be Closed) (Use at own Risk!)")
			.alias("clear-cache")
			.short('c')
			.takes_value(false),
	)
	.arg(
		Arg::new("application")
			.alias("application")
			.short('a')
			.takes_value(true)
			.required(true)
	)
	.arg(
		Arg::new("input")
			.alias("input")
			.short('i')
			.requires_if("custom", "application")
			.takes_value(true)
	)
	.arg(
		Arg::new("file-filter")
			.about("Filename must match the Pattern")
			.aliases(&["file-filter", "ff", "filter"])
			.short('f')
			.takes_value(true)
			.default_value(REGEX_FILE_PATTERN)
	)
	.get_matches()
}

fn read_cache(path: &Path, filter: &Regex) -> Result<Vec<PathBuf>, io::Error> {
    Ok(read_dir(path)
        .unwrap()
        .into_iter()
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap().path())
        .filter(|f| filter.is_match(f.file_name().unwrap().to_str().unwrap()))
        .collect())
}

/// Checks the Mime type and Copies the File to the Destination
fn copy_file(cached: &PathBuf, out_dir: &PathBuf) {
    match infer::get_from_path(&cached).expect("Can't read file") {
        Some(kind) => {
            let mut file = out_dir.clone();
            file.push(format!("{}.{}", rand_str(16), kind.extension()));
            copy(&cached, &file).expect("Failed to copy File");
        }
        None => {}
    }
}

/// Generates a random String
fn rand_str(size: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}
