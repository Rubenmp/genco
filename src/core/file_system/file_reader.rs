use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::str;

use crate::core::file_system::path_helper::try_to_absolute_path;

pub(crate) fn read_all_bytes(file_path: &Path) -> Result<Vec<u8>, String> {
    let mut file = open_file_to_read(file_path)?;
    let file_bytes = get_number_of_bytes(file_path)?;

    let mut input_buffer = Vec::with_capacity(file_bytes);
    file.read_to_end(&mut input_buffer).map_err(|e| {
        format!(
            "Can not read file ({}):\n{}\n",
            e,
            try_to_absolute_path(file_path)
        )
    })?;
    Ok(input_buffer)
}

pub(crate) fn open_file_to_read(file: &Path) -> Result<File, String> {
    match OpenOptions::new().read(true).write(false).open(file) {
        Ok(file) => Ok(file),
        Err(err) => Err(format!(
            "File can not be opened to read ({}):\n\"{}\"\n",
            err,
            try_to_absolute_path(file)
        )),
    }
}

pub(crate) fn read_to_string(file: &Path) -> String {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Unable to read file:\n{}\n", try_to_absolute_path(file)))
}

pub(crate) fn read_string(file: &Path, start_byte: usize, end_byte: usize) -> String {
    let bytes = read_bytes(file, start_byte, end_byte);
    return match str::from_utf8(&bytes) {
        Ok(str) => str.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}

pub(crate) fn read_bytes(file: &Path, start_byte: usize, end_byte: usize) -> Vec<u8> {
    let file = OpenOptions::new()
        .read(true)
        .write(false)
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

pub(crate) fn get_number_of_bytes(file: &Path) -> Result<usize, String> {
    match fs::metadata(file) {
        Ok(file) => Ok(file.len() as usize),
        Err(err) => Err(format!(
            "Can not get bytes from file ({}):\n{}\n",
            err,
            try_to_absolute_path(file)
        )),
    }
}

pub(crate) fn get_number_of_bytes_of(file: &Path) -> usize {
    get_number_of_bytes(file).unwrap_or_else(|_| {
        panic!(
            "Can not get bytes from file:\n{}\n",
            try_to_absolute_path(file)
        )
    })
}
