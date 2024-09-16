use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConfig {
    pub schema: String,
    pub flavour: String,
}

impl AppConfig {
    pub fn new(schema: Option<String>, flavour: Option<String>) -> Self {
        AppConfig {
            schema: schema.unwrap_or(String::from("openapi.yaml")),
            flavour: flavour.unwrap_or(String::from("default")),
        }
    }

    pub fn load() {}
}
