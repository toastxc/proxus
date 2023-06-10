use crate::result::{Error, ErrorConvert};

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Conf {
    pub data: Vec<ConfData>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfData {
    pub a1: String,
    pub a2: String,
}

impl Conf {
    pub fn from_file(path: &str) -> Result<Self, Error> {
        let file = std::fs::read(path).res()?;

        toml::from_str(&String::from_utf8(file).unwrap()).res()
    }
}
