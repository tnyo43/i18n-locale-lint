use std::fmt::Display;

pub enum CliError {
    UnknownExtension,
    FileReadError,
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
