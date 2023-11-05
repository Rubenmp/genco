use crate::java::annotation_usage::JavaAnnotationUsage;

pub(crate) fn _create_service_annotation_usage() -> JavaAnnotationUsage {
    JavaAnnotationUsage::builder()
        .import("org.springframework.stereotype.Service")
        .build()
        .expect("Valid annotation")
}
