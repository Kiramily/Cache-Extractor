use std::{
    env,
    fs::{copy, create_dir, read_dir},
    io,
    path::{Path, PathBuf},
};

use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    let matches = App::new("Discord Cache Extractor")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("output-directory")
                .alias("output-dir")
                .short('o')
                .required(false)
                .takes_value(true),
        )
        .get_matches();
	
    let cache_dir: PathBuf = [
        dirs::config_dir()
            .expect("Failed to get Config Directory")
            .to_str()
            .unwrap(),
        "discord",
        "cache",
    ]
    .iter()
    .collect();

    if cache_dir.exists() {
        let mut output_dir: PathBuf = [env::current_dir().unwrap().to_str().unwrap(), "extracted"]
            .iter()
            .collect();

        if let Some(arg_out_dir) = matches.value_of("output-directory") {
			// Use the Path from the Arguments instead
            output_dir = PathBuf::from(arg_out_dir);
        }

        if !output_dir.exists() {
            create_dir(&output_dir).expect("Failed to create output Directory");
        }

        let files = get_files_from_cache(&cache_dir).expect("Failed to get the Cached Files");

        for file in files {
            copy_file(&file, &output_dir);
        }
    }
}

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
