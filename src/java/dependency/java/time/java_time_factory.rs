use crate::java::data_type::JavaDataType;
use crate::java::import::JavaImport;

pub(crate) fn _create_offset_date_time() -> JavaDataType {
    let import = JavaImport::new_explicit_import_without_m2_repo_scan("java.time.OffsetDateTime")
        .expect("OffsetDateTime java type is in the standard library");
    JavaDataType::from_import(import)
}
