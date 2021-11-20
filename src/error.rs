use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum CipherError {
    LangAlreadyExists,
    LangNotFound,
    NoLangSelected,
}

impl std::error::Error for CipherError {}
impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CipherError::LangAlreadyExists => "Lang name already exists",
                CipherError::LangNotFound => "Lang with specified name did not exist",
                CipherError::NoLangSelected =>
                    "No language was selected.\n\ttry `cipher lang set -n <name>`",
            }
        )
    }
}
