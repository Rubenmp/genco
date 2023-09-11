use crate::java::dto::java_annotation_usage::JavaAnnotationUsage;
use crate::java::dto::java_import::JavaImport;

pub fn _create_autowired_annotation_usage() -> JavaAnnotationUsage {
    JavaAnnotationUsage::builder()
        .import(
            JavaImport::new_explicit_import_requiring_m2_repo_scan(
                "org.springframework.beans.factory.annotation.Autowired",
            )
            .expect("Java explicit import is valid"),
        )
        .build()
}
