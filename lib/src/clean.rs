use std::fs;
use std::path::PathBuf;

pub fn get_deployments_of_device(output_dir: &PathBuf) -> Vec<String> {
    match fs::read_dir(&output_dir) {
        Ok(dir) => {
            let mut entries = Vec::new();
            // Iterate over the entries in the directory
            for entry in dir {
                if let Ok(entry) = entry {
                    // Add the name of each entry to the Vector
                    entries.push(entry.file_name().into_string().unwrap());
                }
            }
            return entries;
        }
        Err(e) => {
            panic!("Error opening the directory {:?}: {}", output_dir, e);
        }
    }
}

pub fn clear_directory(output_dir: &PathBuf) {
    fs::remove_dir_all(&output_dir).expect(&*format!("Removing everything in directory {:?} failed!", &output_dir));
    fs::create_dir(&output_dir).expect(&*format!("Creating new empty output directory in {:?} failed!", &output_dir));
}
