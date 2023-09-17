use crate::core::observability::logger;
use crate::java::dto::java_import::JavaImport;

#[derive(Debug)]
pub struct JavaFileImports {
    explicit_imports: Vec<JavaImport>,
    wildcard_imports: Vec<JavaImport>,
}

impl JavaFileImports {
    pub(crate) fn from(input_imports: Vec<JavaImport>) -> Self {
        let mut result = Self::new();
        for input_import in input_imports {
            result.insert(input_import);
        }

        result
    }

    pub(crate) fn new() -> Self {
        JavaFileImports {
            explicit_imports: Vec::new(),
            wildcard_imports: Vec::new(),
        }
    }

    pub(crate) fn get_explicit_import(&self, type_id: &str) -> Result<JavaImport, String> {
        // TODO: optimize this
        for explicit_import in &self.explicit_imports {
            if explicit_import.match_type_id(type_id) {
                return Ok(explicit_import.clone());
            }
        }

        let error = format!("Import for \"{}\" not found.", type_id);
        Err(error)
    }

    pub(crate) fn insert(&mut self, import: JavaImport) {
        if import.is_explicit_import() {
            self.explicit_imports.push(import);
        } else if import.is_wildcard_import() {
            logger::log_unrecoverable_error(
                format!("Wildcard imports are not supported yet\n\"{}\"", import).as_str(),
            );
            self.wildcard_imports.push(import);
        } else {
            logger::log_unrecoverable_error(&format!(
                "Invalid java import:\n\"{:?}\"",
                import.to_string()
            ));
        }
    }

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
            Ok(import) => {
                assert!(import.is_explicit_import());
            }
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
        insert_imports(&mut imports, imports_vec);
        imports
    }

    fn insert_imports(import_scan: &mut JavaFileImports, imports: Vec<JavaImport>) {
        for import in imports {
            import_scan.insert(import)
        }
    }
}
