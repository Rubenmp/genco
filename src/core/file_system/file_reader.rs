use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::str;

use crate::core::file_system::path_helper::to_absolute_path_str;

pub fn read_to_string(file: &Path) -> String {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Unable to read file:\n{}\n", to_absolute_path_str(file)))
}

pub fn read_string(file: &Path, start_byte: usize, end_byte: usize) -> String {
    let bytes = read_bytes(file, start_byte, end_byte);
    return match str::from_utf8(&bytes) {
        Ok(str) => str.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}

pub fn read_bytes(file: &Path, start_byte: usize, end_byte: usize) -> Vec<u8> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file)
        .unwrap_or_else(|_| panic!("File can not be opened to write \"{:?}\"", file.to_str()));

    let mut reader = BufReader::new(file);

    let mut temporal_buffer = vec![0; end_byte - start_byte];
    reader
        .seek(SeekFrom::Start(start_byte as u64))
        .expect("Try to seek parser node offset to resource");
    reader
        .read_exact(&mut temporal_buffer)
        .expect("File read_exact failed");
    temporal_buffer
}

pub fn get_number_of_bytes_of(file: &Path) -> usize {
    fs::metadata(file)
        .unwrap_or_else(|_| panic!("Can not get bytes from file:\n{}\n",
                to_absolute_path_str(file)))
        .len() as usize
}
