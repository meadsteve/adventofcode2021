use std::error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseSubmarineCommandsError(pub String);

impl error::Error for ParseSubmarineCommandsError {}

impl Display for ParseSubmarineCommandsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
