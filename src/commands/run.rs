use colored::Colorize;

use crate::{config::AppConfig, flavour::get_flavour_config, schema::OpenAPI};

pub fn run(config: AppConfig) -> anyhow::Result<()> {
    println!();
    println!(
        "Will generate based on OpenAPI schema {} using flavour {}...",
        config.schema.bold(),
        config.flavour.blue()
    );
    println!();

    // Retrieve schema from file.
    let _schema = OpenAPI::from(&config.schema)?;

    // Retrieve flavour config.
    let _flavour = get_flavour_config(config.flavour)?;

    println!("{:?} {:?}", _schema, _flavour);

    Ok(())
}
