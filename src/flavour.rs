use anyhow::anyhow;
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize, Debug)]
pub struct Flavour {
    pub version: Option<String>,
    pub language: String,
    #[serde(alias = "template")]
    pub templates: Vec<Template>,
}

#[derive(Deserialize, Debug)]
pub struct Template {
    pub input: String,
    pub output: String,
    pub iteration: Option<String>,
}

pub fn get_flavour_config(name: String) -> anyhow::Result<Flavour> {
    let config_contents = read_to_string(format!(".openapi/flavours/{name}/config.toml"))?;

    toml::from_str(&config_contents).map_err(|error| anyhow!(error))
}
