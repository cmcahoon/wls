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
        let file_name = entry_path.file_name().unwrap();

        let file_size = get_size(entry_path.clone()).unwrap();

        println!("{} {}", file_size, file_name.to_str().unwrap());
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

fn get_size(path: PathBuf) -> Result<u64, std::io::Error> {
    match metadata(path) {
        Ok(stats) => return Ok(stats.len()),
        Err(error) => return Err(error),
    };
}