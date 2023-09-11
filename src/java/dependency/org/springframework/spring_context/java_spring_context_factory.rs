use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
use crate::java::dto::java_import::JavaImport;

pub fn create_service_annotation_usage() -> JavaAnnotationUsage {
    JavaAnnotationUsage::builder()
        .import(
            JavaImport::new_explicit_import_requiring_m2_repo_scan(
                "org.springframework.stereotype.Service",
            )
            .expect("Java explicit import is valid"),
        )
        .build()
}
