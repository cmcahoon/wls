use std::io;
use std::fs::metadata;
use std::fs::read_dir;

pub fn recurse(path: &str) -> io::Result<()> {
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
        println!("{:?}", file_name);
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