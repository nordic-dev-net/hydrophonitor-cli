use std::{fs, io};
use std::path::PathBuf;
use log::{error, info};

pub fn clean(device: &PathBuf) {
    info!("Cleaning device at {:?}", &device);
    let device = device.clone().push("output");

    // Attempt to open the directory
    match fs::read_dir(&device) {
        Ok(entries) => {
            // Iterate over the entries in the directory
            let mut entries_found = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    // Print the name of each entry
                    println!("{}", entry.file_name().to_string_lossy());
                    entries_found = true;
                }
            }
            if entries_found {
                println!("Do you really want to delete these entries? (y/n)");
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                if !(user_input.contains("y") || user_input.contains("Y")) {
                    println!("Aborting!");
                    return;
                }
            } else {
                println!("The directory is already empty!");
                return;
            }
        }
        Err(e) => {
            error!("Error opening the directory {:?}: {}", &device, e);
            return;
        }
    }

    match fs::remove_dir_all(&device) {
        Ok(_) => {}
        Err(e) => {
            error!("Removing everything in directory {:?} failed: {}", &device, e);
            return;
        }
    };
    match fs::create_dir(&device) {
        Ok(_) => {}
        Err(e) => {
            error!("Creating new output directory in {:?} failed: {}", &device, e);
            return;
        }
    };

    println!("Successfully cleaned directory!")
}
