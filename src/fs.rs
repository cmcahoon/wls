use chrono::{DateTime, Utc};
use std::io;
use std::fs::metadata;
use std::fs::read_dir;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use crate::utils::mode_to_string;

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
        let permissions = get_permissions(&entry_path).unwrap();
        let num_hard_links = get_number_hard_links(&entry_path).unwrap();
        let uid = get_uid(&entry_path).unwrap();
        let gid = get_gid(&entry_path).unwrap();
        let file_name = entry_path.file_name().unwrap();
        let file_size = get_size(entry_path.clone()).unwrap();
        let last_modified_time = get_last_modified_time(entry_path.clone()).unwrap();

        println!
        (
            "{} {} {} {} {} {} {}",
            mode_to_string(permissions),
            num_hard_links,
            uid,
            gid,
            file_size,
            last_modified_time,
            file_name.to_str().unwrap()
        );
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

fn get_last_modified_time(path: PathBuf) -> Result<String, io::Error> {
    return match metadata(path) {
        Ok(stats) => {
            return match stats.modified() {
                Ok(last_modified_time) => {
                    let last_modified_time_dt: DateTime<Utc> = last_modified_time.into();
                    Ok(last_modified_time_dt.format("%b %_d %G %H:%I").to_string())
                },
                Err(error) => Err(error),
            };
        },
        Err(error) => Err(error),
    };
}

fn get_uid(path: &PathBuf) -> Result<u32, io::Error> {
    return match metadata(path) {
        Ok(stats) => Ok(stats.uid()),
        Err(error) => Err(error),
    }
}

fn get_gid(path: &PathBuf) -> Result<u32, io::Error> {
    return match metadata(path) {
        Ok(stats) => Ok(stats.gid()),
        Err(error) => Err(error),
    }
}

fn get_permissions(path: &PathBuf) -> Result<u32, io::Error> {
    match metadata(path) {
        Ok(stats) => Ok(stats.mode()),
        Err(error) => Err(error)
    }
}

fn get_number_hard_links(path: &PathBuf) -> Result<u64, io::Error> {
    match metadata(path) {
        Ok(stats) => Ok(stats.nlink()),
        Err(error) => Err(error)
    }
}