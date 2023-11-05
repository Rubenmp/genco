use std::path::{Path, PathBuf};

use crate::core::file_system::file_edition::file_editor;
use crate::core::file_system::file_reader;
use crate::core::file_system::path_helper::try_to_absolute_path;

pub(crate) struct FileOverwriting {
    input_file: PathBuf,
    content_nodes: Vec<FileOverwritingItem>,
}

// Public crate methods
impl FileOverwriting {
    pub(crate) fn from_path(file_path: &Path) -> Result<FileOverwriting, String> {
        Self::check_file_exists(file_path)?;

        Ok(Self::from_unchecked_path(file_path))
    }

    pub(crate) fn from_unchecked_path(file_path: &Path) -> FileOverwriting {
        FileOverwriting {
            input_file: PathBuf::from(file_path),
            content_nodes: Vec::new(),
        }
    }

    pub(crate) fn append_with_previous_newline(&mut self, content: &str) {
        self.append_internal(content, true)
            .expect("append_with_previous_newline must succeed");
    }

    pub(crate) fn insert_content_at(&mut self, byte: usize, content: &str) -> Result<(), String> {
        let item = FileOverwritingItem::new(Some(byte), Some(byte), false, false, content)?;

        self.add_item(item);
        Ok(())
    }

    pub(crate) fn insert_content_with_previous_newline_at(
        &mut self,
        byte: usize,
        content: &str,
    ) -> Result<(), String> {
        let item = FileOverwritingItem::new(Some(byte), Some(byte), true, false, content)?;
        self.add_item(item);
        Ok(())
    }

    /// Add new edition changing slice [start_byte, end_byte] with input content
    pub(crate) fn replace(
        &mut self,
        start_byte: usize,
        end_byte: usize,
        content: &str,
    ) -> Result<(), String> {
        let item =
            FileOverwritingItem::new(Some(start_byte), Some(end_byte), false, false, content)?;
        self.add_item(item);
        Ok(())
    }

    pub(crate) fn get_input_file(&self) -> &PathBuf {
        &self.input_file
    }

    /// Write all the content creating the output file it if it does not exist.
    pub(crate) fn write_all_to_file(&mut self, output_file: &Path) -> Result<(), String> {
        let buffer = self.get_written_buffer()?;

        file_editor::create_or_replace_file_with_bytes(output_file, &buffer)
    }

    /// Write all the content creating the output file it if it does not exist.
    pub(crate) fn write_all(&mut self) -> Result<(), String> {
        let file_path_opt = self.get_input_file();
        let buffer = self.get_written_buffer()?;

        file_editor::create_or_replace_file_with_bytes(file_path_opt, &buffer)
    }
}

// Private methods
impl FileOverwriting {
    fn get_input_bytes(&self) -> Result<Vec<u8>, String> {
        file_reader::read_all_bytes(self.get_input_file())
    }

    fn check_file_exists(file_path: &Path) -> Result<(), String> {
        if !file_path.exists() || !file_path.is_file() {
            let err = format!(
                "Error creating FileOverwriting with invalid input file path:\n\"{}\"\n",
                try_to_absolute_path(file_path)
            );
            return Err(err);
        }
        Ok(())
    }

    fn add_item(&mut self, item: FileOverwritingItem) {
        self.content_nodes.push(item);
    }

    fn append_internal(&mut self, content: &str, previous_new_line: bool) -> Result<(), String> {
        let item = FileOverwritingItem::new(None, None, previous_new_line, true, content)?;
        self.add_item(item);

        Ok(())
    }

    pub(in crate::core::file_system) fn get_written_buffer(&self) -> Result<Vec<u8>, String> {
        let bytes = self.get_input_bytes()?;

        self.get_written_buffer_from_input(&bytes)
    }

    fn get_written_buffer_from_input(&self, input_buffer: &[u8]) -> Result<Vec<u8>, String> {
        let (internal_items, items_to_append) = self.prepare_to_overwrite()?;
        let input_number_of_bytes = input_buffer.len();
        let buffer_size = self.get_required_bytes_to_write(
            input_number_of_bytes,
            &internal_items,
            &items_to_append,
        );
        let mut result_buffer = vec![0; buffer_size];
        let mut input_buffer_index = 0;
        let mut result_buffer_index = 0;

        for item in internal_items {
            if let (Some(start_byte), Some(end_byte)) = (item.get_start_byte(), item.get_end_byte())
            {
                let intermediate_file_content = &input_buffer[input_buffer_index..start_byte];
                let buffer_index_end = result_buffer_index + intermediate_file_content.len();
                result_buffer[result_buffer_index..buffer_index_end]
                    .clone_from_slice(intermediate_file_content);
                result_buffer_index = buffer_index_end;

                let content_bytes = item.get_content_as_bytes();
                let buffer_index_end = result_buffer_index + content_bytes.len();
                result_buffer[result_buffer_index..buffer_index_end]
                    .clone_from_slice(&content_bytes);
                result_buffer_index = buffer_index_end;

                input_buffer_index = end_byte;
            }
        }

        let buffer_until_end = &input_buffer[input_buffer_index..input_number_of_bytes];

        result_buffer[result_buffer_index..(result_buffer_index + buffer_until_end.len())]
            .clone_from_slice(buffer_until_end);
        result_buffer_index += buffer_until_end.len();

        Self::clone_items_to_append_into_buffer(
            &mut result_buffer,
            result_buffer_index,
            items_to_append,
        );
        Ok(result_buffer)
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

    fn prepare_to_overwrite(
        &self,
    ) -> Result<(Vec<FileOverwritingItem>, Vec<FileOverwritingItem>), String> {
        let to_write = self.get_sorted_intermediate_writes();
        self.check_replacements(&to_write)?;

        let to_append = self
            .content_nodes
            .iter()
            .filter(|&node| node.is_to_append())
            .cloned()
            .collect::<Vec<FileOverwritingItem>>();

        if (to_write.len() + to_append.len()) != self.content_nodes.len() {
            return Err(
                "Not all the elements from FileOverwriting.content_nodes are valid writes"
                    .to_string(),
            );
        }

        Ok((to_write, to_append))
    }

    fn get_sorted_intermediate_writes(&self) -> Vec<FileOverwritingItem> {
        let mut to_write = self
            .content_nodes
            .iter()
            .filter(|node| {
                !node.is_to_append() && node.start_byte.is_some() && node.end_byte.is_some()
            })
            .cloned()
            .collect::<Vec<FileOverwritingItem>>();

        to_write.sort_by(|x, y| {
            x.start_byte
                .expect("Start byte checked")
                .cmp(&y.start_byte.expect("End byte checked"))
        });

        to_write
    }

    fn check_replacements(&self, items: &Vec<FileOverwritingItem>) -> Result<(), String> {
        let input_content_bytes = self.get_input_bytes()?.len();
        let max_end_byte = get_max_end_byte(items);
        if max_end_byte > input_content_bytes {
            return Err(format!(
                "Invalid FileOverwritingItem with end_byte {} to resource with {} bytes: \"{}\"",
                max_end_byte,
                input_content_bytes,
                self.get_input_file().to_string_lossy()
            ));
        }

        if !items.is_empty() {
            let first_item = items.get(0).expect("First item exists");
            let mut current_start_byte = first_item
                .get_start_byte()
                .expect("First item start byte exists");
            let mut current_end_byte = first_item
                .get_end_byte()
                .expect("First item end byte exists");
            for item in items.iter().skip(1) {
                let start_byte = item
                    .get_start_byte()
                    .expect("Second item start byte exists");
                let end_byte = item.get_end_byte().expect("Second item end byte exists");
                if start_byte < current_end_byte {
                    let err = format!("Error: can not overwrite resource, bytes [{}, {}] intersect with [{}, {}] at resource \"{}\"", start_byte, end_byte, current_start_byte, current_end_byte, self.get_input_file().to_string_lossy());
                    return Err(err);
                }
                current_start_byte = start_byte;
                current_end_byte = end_byte;
            }
        }

        Ok(())
    }

    fn get_required_bytes_to_write(
        &self,
        initial_number_of_bytes: usize,
        internal_items: &Vec<FileOverwritingItem>,
        to_append_items: &Vec<FileOverwritingItem>,
    ) -> usize {
        let mut result: i64 = initial_number_of_bytes as i64;
        result += get_bytes_required(internal_items);
        result += get_bytes_required(to_append_items);

        result as usize
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
        content: &str,
    ) -> Result<Self, String> {
        check_valid_bytes_to_overwrite(start_byte, end_byte, to_append)?;
        Ok(FileOverwritingItem {
            start_byte,
            end_byte,
            previous_new_line,
            to_append,
            content: content.to_string(),
        })
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
) -> Result<(), String> {
    if to_append && (start_byte_opt.is_some() || end_byte_opt.is_some()) {
        return Err("Can not create FileOverwritingItem type 'to_append' and set intermediate bytes at the same time.".to_string());
    } else if !to_append {
        if start_byte_opt.is_none() || end_byte_opt.is_none() {
            return Err(
                "Can not create FileOverwritingItem without start & end bytes selected."
                    .to_string(),
            );
        }

        if let (Some(start_byte), Some(end_byte)) = (start_byte_opt, end_byte_opt) {
            if start_byte > end_byte {
                return Err(
                    "Can not create FileOverwritingItem with start byte after end_byte."
                        .to_string(),
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::file_system::file_overwriting::file_overwriter::FileOverwriting;
    use crate::core::testing::test_assert::{assert_fail, assert_same_bytes_than_file};
    use crate::core::testing::test_path::{get_non_existing_test_file, get_test_file};

    #[test]
    fn file_overwriting_new_nonexistent_file_panics() {
        let file_path =
            get_non_existing_test_file(&get_current_file_path(), "nonexistent_file.txt");

        let result = FileOverwriting::from_path(&file_path);
        assert!(result.is_err());
        let err = "Error creating FileOverwriting with invalid input file path:\n\"src/core/file_system/file_overwriting/test/nonexistent_file.txt\"\n";
        assert_eq!(Some(err.to_string()), result.err())
    }

    #[test]
    fn file_overwriting_empty_file_error_writing_invalid_bytes() {
        let file_path = get_test_file(&get_current_file_path(), "empty_file.txt");

        let mut overwriting =
            FileOverwriting::from_path(&file_path).expect("FileOverwriting must be created");
        overwriting
            .replace(10, 20, "content")
            .expect("It must admit replacing bytes [10,20]");

        match overwriting.get_written_buffer() {
            Ok(_) => assert_fail("Method must fail with message containing \"Invalid FileOverwritingItem with end_byte 20 to resource with 0 bytes\""),
            Err(err) => assert!(err.contains("Invalid FileOverwritingItem with end_byte 20 to resource with 0 bytes"))
        };
    }

    #[test]
    fn file_overwriting_error_writing_overlapping_intervals() {
        let file_path = get_test_file(&get_current_file_path(), "non_empty_file.txt");

        let mut overwriting =
            FileOverwriting::from_path(&file_path).expect("FileOverwriting must be created");
        overwriting
            .replace(15, 50, "content")
            .expect("It must admit replacing bytes [15,50]");
        overwriting
            .replace(10, 20, "content")
            .expect("It must admit replacing bytes [10,20]");

        match overwriting.get_written_buffer() {
            Ok(_) => assert_fail("Method must fail with message containing \"Invalid FileOverwritingItem with end_byte 20 to resource with 0 bytes\""),
            Err(err) => assert!(err.contains("Error: can not overwrite resource, bytes [15, 50] intersect with [10, 20] at resource"))
        };
    }

    #[test]
    fn file_overwriting_valid_scenario_non_empty_file() {
        let file_path = get_test_file(&get_current_file_path(), "non_empty_file.txt");
        let expected_file_path =
            get_test_file(&get_current_file_path(), "non_empty_file_expected.txt");

        let mut overwriting =
            FileOverwriting::from_path(&file_path).expect("FileOverwriting must be created");
        overwriting
            .insert_content_with_previous_newline_at(5, "content1")
            .expect("It must admit creating content with new line");
        overwriting
            .replace(10, 20, "content2")
            .expect("It must admit replacing bytes [10,20]");
        overwriting
            .replace(30, 50, "content3")
            .expect("It must admit replacing bytes [30,50]");
        overwriting.append_with_previous_newline("content4");

        let buffer = overwriting
            .get_written_buffer()
            .expect("Result file should be written");
        assert_same_bytes_than_file(&expected_file_path, &buffer);
    }

    #[test]
    fn file_overwriting_valid_scenario_file_reduction() {
        let file_path = get_test_file(&get_current_file_path(), "file_reduction.txt");
        let expected_file_path =
            get_test_file(&get_current_file_path(), "file_reduction_expected.txt");

        let mut overwriting =
            FileOverwriting::from_path(&file_path).expect("FileOverwriting must be created");
        overwriting
            .replace(41, 52, "<>")
            .expect("It must admit replacing bytes [41,52]");

        let result_buffer = overwriting
            .get_written_buffer()
            .expect("get_written_buffer method must succeed");

        assert_same_bytes_than_file(&expected_file_path, &result_buffer);
    }

    fn get_current_file_path() -> PathBuf {
        PathBuf::from(file!())
    }
}
