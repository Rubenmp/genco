use crate::core::file_system::file_overwriting::file_overwriter::FileOverwriting;
use crate::core::file_system::path_helper::try_to_absolute_path;
use crate::core::observability::logger;
use crate::java::dto::java_import::JavaImport;

#[derive(Debug)]
pub(crate) struct JavaFileImports {
    explicit_imports: Vec<JavaFileImport>,
    wildcard_imports: Vec<JavaFileImport>,
}

#[derive(Debug)]
pub(crate) struct JavaFileImport {
    import: JavaImport,
    file_end_byte: usize,
}

impl JavaFileImport {
    fn new(import: JavaImport, file_end_byte: usize) -> Self {
        Self {
            import,
            file_end_byte,
        }
    }
}

impl JavaFileImports {
    pub(crate) fn new() -> Self {
        JavaFileImports {
            explicit_imports: Vec::new(),
            wildcard_imports: Vec::new(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.explicit_imports.is_empty() && self.wildcard_imports.is_empty()
    }

    fn get_wildcard_imports(&self) -> Vec<JavaImport> {
        self.wildcard_imports
            .iter()
            .map(|file_import| file_import.import.to_owned())
            .collect()
    }

    fn get_explicit_imports(&self) -> Vec<JavaImport> {
        self.explicit_imports
            .iter()
            .map(|file_import| file_import.import.to_owned())
            .collect()
    }

    pub(crate) fn get_explicit_import(&self, type_id: &str) -> Result<JavaImport, String> {
        // TODO: optimize this
        for explicit_import in &self.get_explicit_imports() {
            if explicit_import.match_type_id(type_id) {
                return Ok(explicit_import.clone());
            }
        }

        let error = format!("Import for \"{}\" not found.", type_id);
        Err(error)
    }

    pub(crate) fn insert(&mut self, import: JavaImport, import_end_byte: usize) {
        if import.is_explicit_import() {
            self.explicit_imports
                .push(JavaFileImport::new(import, import_end_byte));
        } else if import.is_wildcard_import() {
            logger::log_unrecoverable_error(
                format!("Wildcard imports are not supported yet\n\"{}\"", import).as_str(),
            );
            self.wildcard_imports
                .push(JavaFileImport::new(import, import_end_byte));
        } else {
            logger::log_unrecoverable_error(&format!(
                "Invalid java import:\n\"{:?}\"",
                import.to_string()
            ));
        }
    }

    pub(crate) fn add_missing_imports(
        &self,
        to_overwrite: &mut FileOverwriting,
        imports_to_add: Vec<JavaImport>,
        byte_to_insert_first_import_opt: Option<usize>,
    ) -> Result<(), String> {
        let sorted_imports_to_add = get_sorted_asc(imports_to_add);
        let last_import_end_byte_opt = self.get_last_import_end_byte();

        for import_to_add in sorted_imports_to_add {
            if let Some(last_import_end_byte) = last_import_end_byte_opt {
                to_overwrite.insert_content_with_previous_newline_at(
                    last_import_end_byte,
                    &import_to_add.to_string(),
                )
            } else if let Some(byte_to_insert_first_import) = byte_to_insert_first_import_opt {
                to_overwrite
                    .insert_content_with_previous_newline_at(byte_to_insert_first_import, "");
                to_overwrite.insert_content_with_previous_newline_at(
                    byte_to_insert_first_import,
                    &import_to_add.to_string(),
                );
            } else {
                return Err(format!(
                    "It was not possible to add import to file:\n{}\n",
                    try_to_absolute_path(to_overwrite.get_file_path())
                ));
            }
        }
        Ok(())
    }

    fn get_last_import_end_byte(&self) -> Option<usize> {
        let mut result = None;

        for import in self
            .explicit_imports
            .iter()
            .chain(self.wildcard_imports.iter())
        {
            if let Some(current_result) = result {
                if current_result < import.file_end_byte {
                    result = Some(import.file_end_byte);
                }
            } else {
                result = Some(import.file_end_byte);
            }
        }

        result
    }
}

/// TODO: sort in alphabetically ascending order
pub(crate) fn get_sorted_asc(result: Vec<JavaImport>) -> Vec<JavaImport> {
    result.to_vec()
}

impl JavaFileImports {
    // Test specific methods
    #[cfg(test)]
    pub(crate) fn count(&self) -> usize {
        self.explicit_imports.len() + self.wildcard_imports.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::testing::test_assert::assert_fail;
    use crate::java::dto::java_import::JavaImport;
    use crate::java::scanner::file::java_file_imports::JavaFileImports;

    #[test]
    fn get_explicit_import_matches() {
        let imports = get_java_imports_scan_with("org.test.Class");

        let result = imports.get_explicit_import("Class");
        match result {
            Ok(import) => assert!(import.is_explicit_import()),
            Err(error_msg) => assert_fail(&error_msg),
        }
    }

    #[test]
    fn get_explicit_import_error() {
        let imports = get_java_imports_scan_with("org.test.Class");

        let result = imports.get_explicit_import("NotFoundClass");
        match result {
            Ok(_) => assert_fail("It should not return any JavaImport"),
            Err(err) => assert_eq!("Import for \"NotFoundClass\" not found.", err),
        }
    }

    fn get_java_imports_scan_with(input_route: &str) -> JavaFileImports {
        let imports_vec = vec![
            JavaImport::new_explicit_import_requiring_m2_repo_scan(input_route)
                .expect("Java explicit import is valid"),
        ];

        let mut imports = JavaFileImports::new();
        assert!(imports.is_empty());
        insert_imports(&mut imports, &imports_vec);
        assert!(!imports.is_empty());
        assert_eq!(imports_vec.len(), imports.count());

        imports
    }

    fn insert_imports(import_scan: &mut JavaFileImports, imports: &Vec<JavaImport>) {
        for import in imports {
            let irrelevant_import_end_byte_for_test_stub = 0;
            import_scan.insert(import.to_owned(), irrelevant_import_end_byte_for_test_stub);
        }
    }
}
