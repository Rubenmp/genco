use crate::core::observability::logger::logger::log_unrecoverable_error;
use crate::java::scanner::file::dto::java_import::JavaImport;

#[allow(unused)]
pub struct JavaImports {
    explicit_imports: Vec<JavaImport>,
    wildcard_imports: Vec<JavaImport>,
}

impl<'a> JavaImports {
    pub fn new() -> JavaImports {
        JavaImports {
            explicit_imports: Vec::new(),
            wildcard_imports: Vec::new(),
        }
    }

    pub fn insert_import(&mut self, import: JavaImport) {
        if import.is_explicit_import() {
            self.explicit_imports.push(import);
        } else if import.is_wildcard_import() {
            self.wildcard_imports.push(import);
        }
    }

    pub fn count(&self) -> usize {
        self.explicit_imports.len() + self.wildcard_imports.len()
    }
}
