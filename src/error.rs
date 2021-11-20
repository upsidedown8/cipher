use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Lang name already exists")]
    LangAlreadyExists,
    #[error("Lang with specified name did not exist")]
    LangNotFound,
    #[error("The selected lang was not set.\n\ttry `cipher lang set -n <name>`")]
    SelectedLangNotSet,
}
