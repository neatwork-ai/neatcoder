use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    #[cfg(feature = "full")]
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[cfg(feature = "full")]
    #[error(transparent)]
    RustSyn(#[from] syn::Error),
    #[cfg(feature = "full")]
    #[error(transparent)]
    RustPython(#[from] rustpython_parser::error::ParseError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}
