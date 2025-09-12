use std::{env, fmt};

#[derive(Debug)]
pub(crate) struct UsageError {
    program_name: String,
}

impl UsageError {
    pub(crate) fn new() -> Self {
        let program_name = env::args().next().unwrap_or("tile-smith".into());
        Self { program_name }
    }
}

impl fmt::Display for UsageError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "Usage: {} <input image file> <output base name>",
            self.program_name
        )
    }
}

impl std::error::Error for UsageError {}
