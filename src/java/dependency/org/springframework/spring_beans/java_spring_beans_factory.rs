use crate::java::annotation_usage::JavaAnnotationUsage;

pub(crate) fn _create_autowired_annotation_usage() -> JavaAnnotationUsage {
    JavaAnnotationUsage::builder()
        .import("org.springframework.beans.factory.annotation.Autowired")
        .build()
        .expect("Valid annotation")
}
