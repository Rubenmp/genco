use std::fmt;

#[derive(Debug)]
pub(crate) struct SemVer {
    major: usize,
    minor: usize,
    patch: usize,
}

impl SemVer {
    pub(crate) fn is_greater_than_program_version(&self) -> bool {
        // TODO get genco program version somehow
        false
    }
}

impl SemVer {
    pub(crate) fn new(semver_str: &str) -> Result<Self, String> {
        let versions: Vec<&str> = semver_str.split('.').collect();
        match versions.len() {
            1 => {
                if let Some(major) = versions.get(0) {
                    return Self::from_major(major);
                }
                Err("Unexpected SemVer error (major)".to_string())
            }
            2 => {
                if let (Some(major), Some(minor)) = (versions.get(0), versions.get(1)) {
                    return Self::from_major_minor(major, minor);
                }
                Err("Unexpected SemVer error (major, minor)".to_string())
            }
            3 => {
                if let (Some(major), Some(minor), Some(patch)) =
                    (versions.get(0), versions.get(1), versions.get(2))
                {
                    return Self::from_major_minor_patch(major, minor, patch);
                }
                Err("Unexpected SemVer error (major, minor, patch)".to_string())
            }
            _ => Err(format!("Invalid SemVer \"{}\"", semver_str).to_string()),
        }
    }

    fn from_major(major_str: &str) -> Result<Self, String> {
        let major = to_usize(major_str)?;
        Ok(Self {
            major,
            minor: 0,
            patch: 0,
        })
    }

    fn from_major_minor(major_str: &str, minor_str: &str) -> Result<SemVer, String> {
        let major = to_usize(major_str)?;
        let minor = to_usize(minor_str)?;
        Ok(Self {
            major,
            minor,
            patch: 0,
        })
    }

    fn from_major_minor_patch(
        major_str: &str,
        minor_str: &str,
        patch_str: &str,
    ) -> Result<SemVer, String> {
        let major = to_usize(major_str)?;
        let minor = to_usize(minor_str)?;
        let patch = to_usize(patch_str)?;
        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

fn to_usize(major_str: &str) -> Result<usize, String> {
    match major_str.parse::<usize>() {
        Ok(major) => Ok(major),
        Err(_) => {
            Err(format!("can not parse \"{}\" to non-negative number", major_str).to_string())
        }
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}.{:?}.{:?}", self.major, self.minor, self.patch)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::script::semver::SemVer;

    #[test]
    fn new_positive() {
        assert_eq!(
            "0.1.2",
            SemVer::new("0.1.2").expect("Valid semver").to_string()
        );
        assert_eq!(
            "1.2.0",
            SemVer::new("1.2").expect("Valid semver").to_string()
        );
        assert_eq!("2.0.0", SemVer::new("2").expect("Valid semver").to_string());
    }
}
