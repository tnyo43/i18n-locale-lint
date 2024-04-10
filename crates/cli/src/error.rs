use std::fmt::Display;

pub enum CliError<'a> {
    UnknownExtension(&'a str, Option<&'a str>),
    FileReadError,
}

impl<'a> Display for CliError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::UnknownExtension(path, e) => {
                if let Some(ext) = e {
                    write!(f, "Unknown file extension: {} for {}", ext, path)
                } else {
                    write!(f, "Failed to extension for {}", path)
                }
            }
            CliError::FileReadError => todo!(),
        }
    }
}
