use std::{
    fs::{copy, create_dir, read_dir},
    io,
    path::{Path, PathBuf},
};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let mut discord_cache_dir = dirs::config_dir().unwrap();
    discord_cache_dir.push("discord");
    discord_cache_dir.push("Cache");
    if discord_cache_dir.exists() {
        let files = get_files_from_cache(&discord_cache_dir).unwrap();
        let extract_dir = get_or_create_extraction_dir().unwrap();

        for file in files {
            match infer::get_from_path(&file).unwrap() {
                Some(kind) => {
                    let mut extracted_file = PathBuf::from(&extract_dir);
                    extracted_file.push(format!("{}.{}", rand_str(16), kind.extension()));
                    copy(file, &extracted_file).unwrap();
                }
                None => {
                    println!("Failed to get mime type for: {}", file.display())
                }
            }
        }
    } else {
        println!("Cache directory does not exist");
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

fn get_or_create_extraction_dir() -> Result<PathBuf, io::Error> {
    let mut path = PathBuf::from(dirs::picture_dir().unwrap());
    path.push("extracted");
    if !path.exists() {
        create_dir(&path)?;
    }
    Ok(path)
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
