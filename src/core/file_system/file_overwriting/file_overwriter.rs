use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::core::file_system::file_creation::file_creator::{
    create_file_if_not_exist, remove_file_if_exists,
};
use crate::core::file_system::file_reader::get_number_of_bytes_of;

pub struct FileOverwriting {
    file_path: PathBuf,
    content_nodes: Vec<FileOverwritingItem>,
}

impl FileOverwriting {
    pub fn new(file_path: &Path) -> FileOverwriting {
        if !file_path.exists() || !file_path.is_file() {
            panic!(
                "Error creating FileOverwriting with invalid resource \"{}\"",
                file_path.to_string_lossy()
            );
        }

        FileOverwriting {
            file_path: PathBuf::from(file_path),
            content_nodes: Vec::new(),
        }
    }

    pub fn append(&mut self, content: &str) {
        self.append_internal(content, false);
    }

    pub fn append_with_previous_newline(&mut self, content: &str) {
        self.append_internal(content, true);
    }

    fn append_internal(&mut self, content: &str, previous_new_line: bool) {
        self.add_item(FileOverwritingItem::new(
            None,
            None,
            previous_new_line,
            true,
            content.to_string(),
        ));
    }

    pub fn insert_content_with_previous_newline_at(&mut self, byte: usize, content: &str) {
        self.add_item(FileOverwritingItem::new(
            Some(byte),
            Some(byte),
            true,
            false,
            content.to_string(),
        ));
    }

    pub fn replace(&mut self, start_byte: usize, end_byte: usize, content: &str) {
        self.add_item(FileOverwritingItem::new(
            Some(start_byte),
            Some(end_byte),
            false,
            false,
            content.to_string(),
        ));
    }

    fn add_item(&mut self, item: FileOverwritingItem) {
        self.content_nodes.push(item);
    }

    pub fn write_all_to_file(&mut self, output_file: &Path) {
        let input_file = &self.get_file_path().clone();
        self.write_all_internal(input_file, output_file);
    }

    pub fn write_all(&mut self) {
        let file_path = &self.get_file_path().clone();
        self.write_all_internal(file_path, file_path);
    }

    fn write_all_internal(&mut self, input_file: &Path, output_file: &Path) {
        let (internal_items, items_to_append) = self.prepare_to_overwrite();
        let file = self.open_file(input_file);
        let mut reader = BufReader::new(file);
        let mut file_byte_index: usize = 0;
        let mut buffer =
            vec![
                0;
                self.get_required_bytes_to_write(input_file, &internal_items, &items_to_append)
            ];
        let mut buffer_index = 0;
        let initial_file_number_of_bytes = get_number_of_bytes_of(input_file);

        for item in internal_items {
            if let (Some(start_byte), Some(end_byte)) = (item.get_start_byte(), item.get_end_byte())
            {
                let temporal_buffer =
                    Self::read_to_buffer(&mut reader, start_byte - file_byte_index);
                file_byte_index = Self::skip_until_end_byte(&mut reader, end_byte);

                let buffer_index_end = buffer_index + temporal_buffer.len();
                buffer[buffer_index..buffer_index_end].clone_from_slice(&temporal_buffer);
                buffer_index = buffer_index_end;

                let content_bytes = item.get_content_as_bytes();
                let buffer_index_end = buffer_index + content_bytes.len();
                buffer[buffer_index..buffer_index_end].clone_from_slice(&content_bytes);
                buffer_index = buffer_index_end;
            }
        }

        let buffer_until_end = self.read_until_end_to_buffer(
            &mut reader,
            &file_byte_index,
            &initial_file_number_of_bytes,
        );
        buffer[buffer_index..(buffer_index + buffer_until_end.len())]
            .clone_from_slice(&buffer_until_end);
        buffer_index += buffer_until_end.len();

        Self::clone_items_to_append_into_buffer(&mut buffer, buffer_index, items_to_append);

        remove_file_if_exists(output_file);
        create_file_if_not_exist(output_file);
        let mut file = self.open_file(output_file);
        file.seek(io::SeekFrom::Start(0))
            .expect("Seek resource to the beginning");
        file.write_all(&buffer).expect("Write resource failed.");
    }

    fn clone_items_to_append_into_buffer(
        buffer: &mut [u8],
        buffer_index: usize,
        items_to_append: Vec<FileOverwritingItem>,
    ) {
        let mut current_buffer_index = buffer_index;
        for item in items_to_append {
            let content_bytes = item.get_content_as_bytes();
            if !content_bytes.is_empty() {
                let buffer_index_end = current_buffer_index + content_bytes.len();
                buffer[current_buffer_index..buffer_index_end].clone_from_slice(&content_bytes);
                current_buffer_index = buffer_index_end;
            }
        }
    }

    fn read_until_end_to_buffer(
        &mut self,
        reader: &mut BufReader<File>,
        file_byte_index: &usize,
        file_number_of_bytes: &usize,
    ) -> Vec<u8> {
        let remaining_bytes = file_number_of_bytes - file_byte_index;
        let mut buffer: Vec<u8> = Vec::with_capacity(remaining_bytes);
        reader
            .read_to_end(&mut buffer)
            .expect("File read_to_end failed");

        buffer
    }

    fn skip_until_end_byte(reader: &mut BufReader<File>, end_byte: usize) -> usize {
        reader
            .seek(SeekFrom::Start(end_byte as u64))
            .expect("Try to seek parser node offset to resource");
        end_byte
    }

    fn read_to_buffer(reader: &mut BufReader<File>, size: usize) -> Vec<u8> {
        let mut temporal_buffer = vec![0; size];
        reader
            .read_exact(&mut temporal_buffer)
            .expect("File read_exact failed");
        temporal_buffer
    }

    fn open_file(&mut self, output_file: &Path) -> File {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(output_file)
            .expect("File can not be opened to write")
    }

    fn prepare_to_overwrite(&mut self) -> (Vec<FileOverwritingItem>, Vec<FileOverwritingItem>) {
        let to_write = self.get_sorted_intermediate_writes();
        self.check_replacements(&to_write);

        let to_append = self
            .content_nodes
            .clone()
            .into_iter()
            .filter(|node| node.is_to_append())
            .collect::<Vec<FileOverwritingItem>>();

        if (to_write.len() + to_append.len()) != self.content_nodes.len() {
            panic!("Not all the elements from FileOverwriting.content_nodes are valid writes");
        }

        (to_write, to_append)
    }

    fn get_sorted_intermediate_writes(&mut self) -> Vec<FileOverwritingItem> {
        let mut to_write = self
            .content_nodes
            .clone()
            .into_iter()
            .filter(|node| {
                !node.is_to_append() && node.start_byte.is_some() && node.end_byte.is_some()
            })
            .collect::<Vec<FileOverwritingItem>>();
        to_write.sort_by(|x, y| x.start_byte.unwrap().cmp(&y.start_byte.unwrap()));

        to_write
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }

    fn check_replacements(&self, items: &Vec<FileOverwritingItem>) {
        let bytes_in_file = self.get_file_number_of_bytes();
        let max_end_byte = get_max_end_byte(items);
        if max_end_byte > bytes_in_file {
            panic!(
                "Invalid FileOverwritingItem with end_byte {} to resource with {} bytes: \"{}\"",
                max_end_byte,
                bytes_in_file,
                self.get_file_path().to_string_lossy()
            );
        }

        if !items.is_empty() {
            let mut current_start_byte = items.get(0).unwrap().get_start_byte().unwrap();
            let mut current_end_byte = items.get(0).unwrap().get_end_byte().unwrap();
            for item in items.iter().skip(1) {
                let start_byte = item.get_start_byte().unwrap();
                let end_byte = item.get_end_byte().unwrap();
                if start_byte < current_end_byte {
                    panic!("Error: can not overwrite resource, bytes [{}, {}] intersect with [{}, {}] at resource \"{}\"", start_byte, end_byte, current_start_byte, current_end_byte, self.get_file_path().to_string_lossy());
                }
                current_start_byte = start_byte;
                current_end_byte = end_byte;
            }
        }
    }

    fn get_required_bytes_to_write(
        &self,
        file: &Path,
        internal_items: &Vec<FileOverwritingItem>,
        to_append_items: &Vec<FileOverwritingItem>,
    ) -> usize {
        let mut file_bytes: i64 = get_number_of_bytes_of(file) as i64;
        file_bytes += get_bytes_required(internal_items);
        file_bytes += get_bytes_required(to_append_items);

        file_bytes as usize
    }

    fn get_file_number_of_bytes(&self) -> usize {
        fs::metadata(self.get_file_path())
            .expect("File to override not opened")
            .len() as usize
    }
}

fn get_bytes_required(items: &Vec<FileOverwritingItem>) -> i64 {
    let mut sum: i64 = 0;
    for item in items {
        sum += item.get_content_as_bytes().len() as i64;
        if let (Some(start_byte), Some(end_byte)) = (item.get_start_byte(), item.get_end_byte()) {
            sum -= (end_byte - start_byte) as i64;
        }
    }
    sum
}

fn get_max_end_byte(items: &[FileOverwritingItem]) -> usize {
    items
        .iter()
        .map(|item| item.get_end_byte().unwrap_or(0))
        .max()
        .unwrap_or(0)
}

#[derive(Debug, Clone)]
struct FileOverwritingItem {
    /// If applicable, start byte from resource to write into
    start_byte: Option<usize>,
    /// If applicable, end byte from resource to write into
    end_byte: Option<usize>,
    /// Setting to include a newline before the content
    previous_new_line: bool,
    /// Setting to write the content at the end of the resource, instead of using start_byte & end_byte
    to_append: bool,
    /// Actual content to write
    content: String,
}

impl FileOverwritingItem {
    pub fn new(
        start_byte: Option<usize>,
        end_byte: Option<usize>,
        previous_new_line: bool,
        to_append: bool,
        content: String,
    ) -> Self {
        check_valid_bytes_to_overwrite(start_byte, end_byte, to_append, &content);
        FileOverwritingItem {
            start_byte,
            end_byte,
            previous_new_line,
            to_append,
            content,
        }
    }

    pub fn get_start_byte(&self) -> Option<usize> {
        self.start_byte
    }

    pub fn get_end_byte(&self) -> Option<usize> {
        self.end_byte
    }

    pub fn get_content_as_bytes(&self) -> Vec<u8> {
        let content_bytes = self.content.as_bytes().to_vec();
        if !self.previous_new_line {
            return content_bytes;
        }

        let new_line_bytes = get_new_line_number_of_bytes();
        let mut buffer: Vec<u8> = vec![0; content_bytes.len() + new_line_bytes];

        write_initial_new_line(&mut buffer);
        buffer[new_line_bytes..new_line_bytes + content_bytes.len()]
            .clone_from_slice(&content_bytes);

        buffer
    }

    pub fn is_to_append(&self) -> bool {
        self.to_append
    }
}

fn write_initial_new_line(buffer: &mut [u8]) {
    buffer[0..get_new_line_number_of_bytes()].clone_from_slice(&get_new_line_as_bytes());
}

fn get_new_line_as_bytes() -> Vec<u8> {
    b'\n'.to_be_bytes().to_vec()
}

fn get_new_line_number_of_bytes() -> usize {
    get_new_line_as_bytes().len()
}

fn check_valid_bytes_to_overwrite(
    start_byte_opt: Option<usize>,
    end_byte_opt: Option<usize>,
    to_append: bool,
    content: &String,
) {
    if to_append && (start_byte_opt.is_some() || end_byte_opt.is_some()) {
        panic!("Can not create FileOverwritingItem type 'to_append' and set intermediate bytes at the same time.");
    } else if !to_append {
        if start_byte_opt.is_none() || end_byte_opt.is_none() {
            panic!("Can not create FileOverwritingItem without start & end bytes selected.");
        }

        if let (Some(start_byte), Some(end_byte)) = (start_byte_opt, end_byte_opt) {
            if start_byte > end_byte {
                panic!("Can not create FileOverwritingItem with start byte after end_byte.");
            }
        }
    }

    if content.is_empty() {
        //panic!("Can not create FileOverwritingItem with empty content.");
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_creation::file_creator::{
        create_file_with_content, remove_file_if_exists,
    };
    use crate::core::file_system::file_overwriting::file_overwriter::FileOverwriting;
    use crate::core::testing::test_assert::assert_same_file;
    use crate::core::testing::test_path::{get_non_existing_test_file, get_test_file};

    #[test]
    fn file_overwriting_new_nonexistent_file_panics() {
        let file_path = get_non_existing_test_file(get_current_file_path(), "nonexistent_file.txt");

        let result = std::panic::catch_unwind(|| FileOverwriting::new(&file_path));
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn file_overwriting_empty_file_error_writing_invalid_bytes() {
        let file_path = get_test_file(get_current_file_path(), "empty_file.txt");

        let mut overwriting = FileOverwriting::new(&file_path);
        overwriting.replace(10, 20, "content");

        overwriting.write_all();
    }

    #[test]
    #[should_panic(
        expected = "Error: can not overwrite resource, bytes [15, 50] intersect with [10, 20] at resource"
    )]
    fn file_overwriting_error_writing_overlapping_intervals() {
        let file_path = get_test_file(get_current_file_path(), "non_empty_file.txt");

        let mut overwriting = FileOverwriting::new(&file_path);
        overwriting.replace(15, 50, "content");
        overwriting.replace(10, 20, "content");

        overwriting.write_all();
    }

    #[test]
    fn file_overwriting_valid_scenario_non_empty_file() {
        let file_path = get_test_file(get_current_file_path(), "non_empty_file.txt");
        let file_path_copy =
            get_non_existing_test_file(get_current_file_path(), "non_empty_file_copy.txt");
        let expected_file_path =
            get_test_file(get_current_file_path(), "non_empty_file_expected.txt");
        create_file_with_content(&file_path_copy, &file_path);

        let mut overwriting = FileOverwriting::new(&file_path);
        overwriting.insert_content_with_previous_newline_at(5, "content1");
        overwriting.replace(10, 20, "content2");
        overwriting.replace(30, 50, "content3");
        overwriting.append_with_previous_newline("content4");

        overwriting.write_all();
        assert_same_file(&expected_file_path, &file_path);
        create_file_with_content(&file_path, &file_path_copy);
        remove_file_if_exists(&file_path_copy);
    }

    #[test]
    fn file_overwriting_valid_scenario_file_reduction() {
        let file_path = get_test_file(get_current_file_path(), "file_reduction.txt");
        let file_path_output =
            get_non_existing_test_file(get_current_file_path(), "file_reduction_output.txt");
        let expected_file_path =
            get_test_file(get_current_file_path(), "file_reduction_expected.txt");

        let mut overwriting = FileOverwriting::new(&file_path);
        overwriting.replace(41, 52, "<>");

        println!(
            "first part: {}",
            "The resource overwriting process will remove "
                .as_bytes()
                .len()
        );
        println!("second part: {}", "<this part>".as_bytes().len());
        println!("File length: {}", "The resource overwriting process will remove <this part> resource without trash at the end of the resource.".as_bytes().len());
        overwriting.write_all_to_file(&file_path_output);
        assert_same_file(&expected_file_path, &file_path_output);
        remove_file_if_exists(&file_path_output);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
