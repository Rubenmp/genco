use crate::java::annotation_usage::JavaAnnotationUsage;
use crate::java::import::JavaImport;

// Tested in 5.8.2
pub(crate) fn _create_test_annotation_usage() -> JavaAnnotationUsage {
    JavaAnnotationUsage::builder()
        .import(
            JavaImport::new_explicit_import_requiring_m2_repo_scan("org.junit.jupiter.api.Test")
                .expect("Java explicit import is valid"),
        )
        .build()
}
