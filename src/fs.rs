use chrono::{DateTime, Utc};
use colored::Colorize;
use libc::{c_char, getgrgid_r, getpwuid_r, group, passwd, size_t};
use std::{io};
use std::fs::metadata;
use std::fs::read_dir;
use std::mem::MaybeUninit;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::ptr::null_mut;
use crate::utils::{mode_to_string, Size};

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
        let user_name = get_user_name(uid);
        let gid = get_gid(&entry_path).unwrap();
        let group_name = get_group_name(gid);

        let mut file_name = entry_path.file_name().unwrap().to_str().unwrap().to_string();
        if entry.metadata()?.is_dir() {
            file_name.push_str("/");
        }

        let file_size = Size::from_bytes(get_size(entry_path.clone()).unwrap());
        let last_modified_time = get_last_modified_time(entry_path.clone()).unwrap();

        println!
        (
            "{} {} {} {} {} {} {}",
            mode_to_string(permissions),
            num_hard_links,
            user_name,
            group_name,
            file_size,
            last_modified_time,
            if file_name.ends_with("/") { file_name.blue().bold() } else { file_name.normal() }
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

fn get_user_name(uid: u32) -> String {
    let mut pwd = MaybeUninit::<passwd>::uninit();
    let mut buf: Vec<u8> = vec![0; 128];
    let buflen: size_t = 128;
    let mut _result = null_mut::<passwd>();

    unsafe {
        let _retval = getpwuid_r(uid, pwd.as_mut_ptr(), buf.as_mut_ptr() as *mut c_char, buflen, &mut _result);
    }

    let strlen = match buf.iter().position(|&x| 0 == x) {
        Some(n) => n,
        None => buf.len(),
    };

    buf.truncate(strlen);

    String::from_utf8(buf).unwrap()
}

fn get_group_name(gid: u32) -> String {
    let mut grp = MaybeUninit::<group>::uninit();
    let mut buf: Vec<u8> = vec![0; 128];
    let buflen: size_t = 128;
    let mut _result = null_mut::<group>();

    unsafe {
        let _retval = getgrgid_r(gid, grp.as_mut_ptr(), buf.as_mut_ptr() as *mut c_char, buflen, &mut _result);
    }

    let strlen = match buf.iter().position(|&x| 0 == x) {
        Some(n) => n,
        None => buf.len(),
    };

    buf.truncate(strlen);

    String::from_utf8(buf).unwrap()
}