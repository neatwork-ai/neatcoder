use thiserror::Error;

#[derive(Error, Debug)]
pub enum GluonError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    RustSyn(#[from] syn::Error),
    #[error(transparent)]
    RustPython(#[from] rustpython_parser::error::ParseError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}
