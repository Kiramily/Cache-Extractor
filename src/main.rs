use std::{
    env,
    fs::{copy, create_dir, read_dir, remove_dir_all},
    io,
    path::{Path, PathBuf},
};

use clap::{App, Arg, ArgMatches};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

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

    apps.for_each(|app| {
        let mut start_path =
            PathBuf::from(dirs::config_dir().expect("Failed to get the config Directory"));

        match app {
            "discord" => {
                start_path.push("discord/Cache");
            }
            "guilded" => {
                start_path.push("guilded/Cache");
            }
            "vscode" => {
                start_path.push("Code/Cache");
            }
            "vscode-insider" => {
                start_path.push("Code - Insider/Cache");
            }
            "custom" => {
                if let Some(path) = matches.value_of("input") {
                    start_path = PathBuf::from(path);
                }
            }
            _ => {
                println!("[!] Unknown Application: {}", app);
            }
        }

        if start_path.exists() {
            // TODO Copy
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

// fn main_old() {
// 	let cache_dir: PathBuf = [
//         dirs::config_dir()
//             .expect("Failed to get Config Directory")
//             .to_str()
//             .unwrap(),
//         "discord",
//         "cache",
//     ]
//     .iter()
//     .collect();

//     if cache_dir.exists() {
//         let mut output_dir: PathBuf = [env::current_dir().unwrap().to_str().unwrap(), "extracted"]
//             .iter()
//             .collect();

//         if let Some(arg_out_dir) = matches.value_of("output-directory") {
//             // Use the Path from the Arguments instead
//             output_dir = PathBuf::from(arg_out_dir);
//         }

//         if !output_dir.exists() {
//             create_dir(&output_dir).expect("Failed to create output Directory");
//         }

//         let files = get_files_from_cache(&cache_dir).expect("Failed to get the Cached Files");

//         for file in files {
//             copy_file(&file, &output_dir);
//         }

//         if matches.is_present("clear-cache") {
//             remove_dir_all(&cache_dir).expect("Failed to clear the Cache");
//         }
//     }
// }

/// Checks the Mime type and Copies the File to the Destination
fn copy_file(cached: &PathBuf, out_dir: &PathBuf) {
    match infer::get_from_path(&cached).expect("Can't read file") {
        Some(kind) => {
            let mut file = out_dir.clone();
            file.push(format!("{}.{}", rand_str(16), kind.extension()));
            copy(&cached, &file).expect("Failed to copy File");
        }
        None => {
            println!("[!] Failed to get mime type for: {}", cached.display())
        }
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

/// Reads the Cache directory and filters out the Files not starting with 'f_'.
fn get_files_from_cache(path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(read_dir(path)
        .unwrap()
        .into_iter()
        .filter(|f| f.is_ok())
        .map(|f| f.unwrap().path())
        .filter(|f| f.file_name().unwrap().to_str().unwrap().starts_with("f_"))
        .collect())
}
