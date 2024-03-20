use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn flash(image_path: &PathBuf, device_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Open source file for reading
    let mut source_file = File::open(image_path).expect(&*format!("Failed to open image {:?}!", image_path));

    // Open destination file for writing
    let mut dest_file = File::open(device_path).expect(&*format!("Failed to open device file {:?}!", device_path));

    // Set buffer size for read and write operations
    let mut buffer = [0; 4096]; // Adjust the buffer size as needed

    // Read data from source and write to destination
    loop {
        let bytes_read = source_file.read(&mut buffer).expect("Failed to read bytes from image!");
        if bytes_read == 0 {
            // Reached end of file
            return Ok(());
        }
        dest_file.write_all(&buffer[..bytes_read]).expect("Failed to write bytes to device!");
    }
}
