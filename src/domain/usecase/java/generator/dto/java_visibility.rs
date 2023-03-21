use std::fmt;

#[derive(Debug)]
pub enum JavaVisibility {
    Public,
    Private,
    Package,
    Protected,
}

impl fmt::Display for JavaVisibility {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)?;

        Ok(())
    }
}
