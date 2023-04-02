mod file_info;

use itertools::Itertools;
use std::{env, fs, io, thread};
use std::fs::DirEntry;
use walkdir::WalkDir;
use file_info::FileInfo;
use chrono::{NaiveDateTime, NaiveDate, DateTime, Local, Datelike, Timelike};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <name>", args[0]);
        std::process::exit(1);
    }
    let path :&str = &args[1];

    if !(Path::new(&path)).exists()
    {
        eprintln!("Error: path {} does not exist", path);
        std::process::exit(1);
    }

    let mut file_info_vec = Vec::new();

    walkDirectory(path, &mut file_info_vec);

    // Spawn a vector of child threads
    let mut handles = Vec::new();

    println!("Found {} files:", file_info_vec.len());

    let groups = file_info_vec.into_iter().group_by(|file_info| {
        let creation_date = chrono::NaiveDateTime::from_timestamp(
            file_info.creation_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            0,
        );
        (creation_date.year(), creation_date.month())
    });
    let grouped_vecs: Vec<Vec<FileInfo>> = groups.into_iter().map(|(_, group)| group.collect()).collect();

    println!("Split files into {} groups:", grouped_vecs.len());
    for group in grouped_vecs {

        let local_time: DateTime<Local> = group[0].creation_time.into(); // convert to local time
        //let year_month:(i32, u32) = (local_time.year(), local_time.month());

        let year = local_time.year();
        let month = local_time.month();
        let folder_name = format!("{:02}-{:02}", year % 100, month);
        let mut new_folder_path = PathBuf::new();
        new_folder_path.push(path);
        new_folder_path.push(folder_name);

        if !new_folder_path.exists()
        {
            std::fs::create_dir(&new_folder_path).unwrap();
        }
        else
        {
            println!("{} already Exists!", new_folder_path.display());
        }

        // Spawn a new thread and increment the active_threads counter
        //let active_threads_clone = active_threads.clone();
        let handle = thread::spawn(move || {
            //let mut counter = active_threads_clone.lock().unwrap();
            //*counter += 1;

            for file_info in group {
                let mut new_path = PathBuf::new();
                new_path.push(&new_folder_path);
                //new_path.push(file_info.path.file_name().unwrap());

                match file_info.path.file_name() {
                    Some(file_name) => new_path.push(file_name),
                    None => {
                        // Handle the error case
                        eprintln!("Error: unable to get file name for {:?}", file_info.path);
                        continue;
                    }
                }

                println!("{:?}\nMoving: {}\n    TO: {}",thread::current().id(), file_info.path.display(),new_path.display());
                //println!("TO: {}",new_path.display());


                if let Some(path_str) = file_info.path.to_str()
                {
                    if path_str == "Thumbs.db"
                    {
                        continue;
                    }
                }
                else
                {
                    // Handle the error case
                    eprintln!("Error: unable to convert path to string for {:?}", file_info.path);
                    continue;
                }

                if !new_path.exists()
                {
                    std::fs::rename(file_info.path, new_path).unwrap();
                }
                else
                {
                    println!("{:?}\n{} Already Exists!\n", thread::current().id(), new_path.display());
                    std::fs::rename(file_info.path, new_path).unwrap();
                }

                //*counter -= 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all child threads to finish before exiting the main thread
    for handle in handles
    {
        //handle.join().unwrap();

        match handle.join() {
            Ok(_) => {
                // Handle the success case
                println!("Thread finished successfully");
            }
            Err(error) => {
                // Handle the error case
                eprintln!("Error: thread panicked or encountered an I/O error: {:?}", error);
            }
        }
    }

    println!("Deleting empty directories");
    delete_empty_directories(path).unwrap();
}

fn delete_empty_directories(path: &str) -> io::Result<()>
{
    let dir = fs::read_dir(path)?;

    for entry in dir
    {
        let entry = entry?;

        if entry.file_type()?.is_dir()
        {
            if !has_visible_files(entry.path().to_str().unwrap())?
            {
                println!("Removing: {}", entry.path().display());
                fs::remove_dir_all(entry.path().to_str().unwrap())?;
            }
        }
    }
    Ok(())
}

fn walkDirectory(path: &str, file_info_vec: &mut Vec<FileInfo>) {
    for entry in WalkDir::new(path)
    {
        if let Ok(entry) = entry
        {
            let path = entry.path().to_owned();
            if path.is_file()
            {
                let creation_time = fs::metadata(&path).unwrap().modified().unwrap();
                let file_info = FileInfo::new(path, creation_time);
                file_info_vec.push(file_info);
            }
        }
    }
}

fn is_thumbs_db(entry: &DirEntry) -> bool
{
    return entry.path().file_name().unwrap() == "Thumbs.db";
}

fn has_visible_files(path: &str) -> io::Result<bool> {
    let dir = fs::read_dir(path)?;

    for entry in dir {
        let entry = entry?;

        //println!("File: {}", entry.path().display());

        if !is_thumbs_db(&entry)
        {
            return Ok(true);
        }
    }

    Ok(false)
}


