use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;


pub fn read_bytes(file: &PathBuf, start_byte: usize, end_byte: usize) -> Vec<u8> {
    let file = OpenOptions::new()
        .read(true).write(true)
        .open(file)
        .expect("File can not be opened to write");

    let mut reader = BufReader::new(file);

    let mut temporal_buffer = vec![0; end_byte-start_byte];
    reader.seek(SeekFrom::Start(start_byte as u64)).expect("Try to seek parser node offset to file");
    reader.read_exact(&mut temporal_buffer).expect("File read_exact failed");
    temporal_buffer
}

pub fn get_number_of_bytes_of(file: &PathBuf) -> usize {
    fs::metadata(file).expect("File to get number of bytes from can not be opened").len() as usize
}