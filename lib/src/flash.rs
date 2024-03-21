use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;

pub fn flash(image_path: &PathBuf, device_path: &PathBuf) -> Result<()> {
    let mut source_file = File::open(image_path).expect(&*format!("Failed to open image {:?}!", image_path));
    let mut dest_file = File::create(device_path).expect(&*format!("Failed to open device file {:?}!", device_path));
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = source_file.read(&mut buffer).expect("Failed to read bytes from image!");
        if bytes_read == 0 {
            return Ok(());
        }
        dest_file.write_all(&buffer[..bytes_read]).expect("Failed to write bytes to device!");
    }
}
