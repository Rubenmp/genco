use std::path::Path;

pub fn get_package(dir_path: &Path) -> String {
    if !dir_path.exists() || !dir_path.is_dir() {
        panic!(
            "Error: get_package method must be called with a directory path, found: {:?}",
            dir_path
        );
    }

    for ancestor in dir_path.ancestors() {
        if ancestor.ends_with("java") {
            if let Some(second_ancestor) = ancestor.parent() {
                if second_ancestor.ends_with("main") {
                    if let Some(third_ancestor) = second_ancestor.parent() {
                        if third_ancestor.ends_with("src") {
                            let bytes = ancestor.to_string_lossy().as_bytes().len();
                            let mut package_route = dir_path.to_string_lossy().to_string()[bytes..]
                                .to_owned()
                                .replace("/", ".")
                                .replace("\\", ".");
                            package_route.remove(0); // Remove first "."
                            return package_route;
                        }
                    }
                }
            }
        }
    }

    "".to_string()
}
