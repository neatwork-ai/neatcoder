use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Files(pub VecDeque<String>);

impl AsRef<VecDeque<String>> for Files {
    fn as_ref(&self) -> &VecDeque<String> {
        &self.0
    }
}

impl Deref for Files {
    type Target = VecDeque<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Files {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
