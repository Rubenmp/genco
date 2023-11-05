use crate::java::annotation_usage::JavaAnnotationUsage;

// Tested in 5.8.2
pub(crate) fn _create_test_annotation_usage() -> JavaAnnotationUsage {
    JavaAnnotationUsage::builder()
        .import("org.junit.jupiter.api.Test")
        .build()
        .expect("Valid annotation")
}
