use std::fmt::{Display, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::core::file_system::file_cache::FileCache;
use crate::core::file_system::file_reader;
use crate::core::parser::string_helper;
use crate::core::parser::string_helper::escape_str_for_json;

pub(crate) trait ParserNode<NodeType: Display + PartialEq + Copy>: Sized {
    fn from_path(file_path: &Path) -> Result<Self, String>;

    fn get_start_byte(&self) -> usize;
    fn get_end_byte(&self) -> usize;
    fn get_file_path(&self) -> &Path;
    fn get_children(&self) -> &Vec<Self>;
    fn get_node_type(&self) -> Option<NodeType>;

    #[allow(unused)]
    fn print_tree_and_panic(&self) {
        println!("{}", self.get_tree_str());
        panic!("Finished");
    }

    fn get_tree_str(&self) -> String {
        self.get_tree_str_internal(0, 1, false)
    }

    fn get_tree_str_internal(
        &self,
        depth: usize,
        current_child_index: usize,
        show_bytes: bool,
    ) -> String {
        let mut tree_str: String = "".to_string();
        if depth == 0 {
            tree_str.push('{');
            writeln!(&mut tree_str).expect("Error in method ParserNode.get_tree_str");
        }
        tree_str.push_str(&"  ".repeat(depth + 1));

        let type_str: String;
        if let Some(node_type_str) = self.get_node_type() {
            type_str = node_type_str.to_string();
        } else {
            type_str = "UnknownType".to_string();
        }

        if show_bytes {
            write!(
                tree_str,
                "\"{}. {} [{}, {}]\"",
                current_child_index,
                type_str,
                self.get_start_byte(),
                self.get_end_byte()
            )
            .expect("Writing parser node showing bytes");
        } else {
            write!(tree_str, "\"{}. {}\"", current_child_index, type_str)
                .expect("Writing parser node");
        }

        let children = self.get_children();
        if self.is_composed_node_printable() || children.is_empty() {
            tree_str.push_str(": ");
            tree_str.push_str(&format!(
                "\"{}\"",
                escape_str_for_json(self.get_content()).as_str()
            ));
        } else {
            tree_str.push_str(": {");
            writeln!(&mut tree_str).expect("Error in method ParserNode.get_tree_str");

            for (index, child) in children.iter().enumerate() {
                tree_str.push_str(
                    child
                        .get_tree_str_internal(depth + 1, index + 1, show_bytes)
                        .as_str(),
                );
                if index < (children.len() - 1) {
                    tree_str.push(',');
                }
                writeln!(&mut tree_str).expect("Error in method ParserNode.get_tree_str");
            }

            tree_str.push_str("  ".repeat(depth + 1).as_str());
            tree_str.push('}');
        }

        if depth == 0 {
            writeln!(&mut tree_str).expect("Error in method ParserNode.get_tree_str");
            tree_str.push('}');
        }
        tree_str
    }

    fn is_composed_node_printable(&self) -> bool;

    /// WARN: this method is really slow because it gets the content from a file.
    /// It is mostly used during file parsing analysis, an improvement for this
    /// would read the whole file into a buffer, get_content_from_buffer and then
    /// free the buffer, to do only one read to file, not hundred/thousand smaller ones.
    ///
    /// Migrate to get_content_from_cache method
    #[deprecated(note = "please use `get_content_from_cache` instead")]
    fn get_content(&self) -> String {
        let buffer = file_reader::read_bytes(
            self.get_file_path(),
            self.get_start_byte(),
            self.get_end_byte(),
        );

        string_helper::to_str(&buffer)
    }

    fn get_content_from_cache(&self, file_cache: &FileCache) -> String {
        file_cache.get_content(self.get_start_byte(), self.get_end_byte())
    }

    fn get_content_bytes_with_previous_empty_space(&self) -> String {
        let file = File::open(self.get_file_path()).expect("Parser node resource must open.");
        let mut reader = BufReader::new(&file);
        let mut current_start_byte = 0;

        let mut buf = String::new();
        while let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            } // eof
            let next_current_byte = current_start_byte + n;
            if next_current_byte > self.get_start_byte() {
                break;
            }
            current_start_byte += n;
            buf.clear();
        }

        if current_start_byte == 0 {
            current_start_byte = self.get_start_byte();
        }

        let buffer = file_reader::read_bytes(
            self.get_file_path(),
            current_start_byte,
            self.get_end_byte(),
        );

        string_helper::to_str(&buffer)
    }

    /// Depth First Search
    fn depth_first_search_bytes(&self, node_type: NodeType) -> Option<(usize, usize)> {
        if Some(node_type) == self.get_node_type() {
            return Some((self.get_start_byte(), self.get_end_byte()));
        }

        for child in self.get_children() {
            if let Some(sub_node_search_bytes) = child.depth_first_search_bytes(node_type) {
                return Some(sub_node_search_bytes);
            }
        }

        None
    }
}
