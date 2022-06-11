use chrono::{DateTime, Utc};
use std::io;
use std::fs::metadata;
use std::fs::read_dir;
use std::path::PathBuf;

pub fn recurse(path: &str) -> Result<(), io::Error> {
    // ERROR: Only process directories
    let is_dir = is_directory(path);
    if !is_dir {
        panic!("ERROR: Path was not a directory.");
    }

    // Process the directory
    let results = read_dir(path)?;
    for result in results {
        let entry = result?;
        let entry_path = entry.path();

        // Query attributes
        //
        // TODO: Add some real error handling!
        let file_name = entry_path.file_name().unwrap();
        let file_size = get_size(entry_path.clone()).unwrap();
        let last_accessed_time = get_last_accessed_time(entry_path.clone()).unwrap();

        println!("{} {} {}",
                 file_size,
                 last_accessed_time,
                 file_name.to_str().unwrap());
    }

    Ok(())
}

fn is_directory(path: &str) -> bool {
    let stat = metadata(path);
    let stat = match stat {
        Ok(stat) => stat,
        Err(error) => panic!("Could not get file metadata: {:?}", error),
    };

    stat.is_dir()
}

fn get_size(path: PathBuf) -> Result<u64, io::Error> {
    return match metadata(path) {
        Ok(stats) => Ok(stats.len()),
        Err(error) => Err(error),
    };
}

fn get_last_accessed_time(path: PathBuf) -> Result<String, io::Error> {
    return match metadata(path) {
        Ok(stats) => {
            return match stats.accessed() {
                Ok(last_accessed_time) => {
                    let last_accessed_time_dt: DateTime<Utc> = last_accessed_time.into();
                    Ok(last_accessed_time_dt.format("%b %_d %G %H:%I").to_string())
                },
                Err(error) => Err(error),
            };
        },
        Err(error) => Err(error),
    };
}