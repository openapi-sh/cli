use std::fs::{create_dir, File};

use colored::Colorize;

pub fn init() -> anyhow::Result<()> {
    let mut created_entities: u8 = 0;
    println!();
    if create_dir(".openapi").is_ok() {
        created_entities += 1;
        println!("Created directory {}", ".openapi".bold());
    }
    if create_dir(".openapi/flavours").is_ok() {
        created_entities += 1;
        println!("Created directory {}", ".openapi/flavours".bold());
    }
    if create_dir(".openapi/languages").is_ok() {
        created_entities += 1;
        println!("Created directory {}", ".openapi/languages".bold());
    }
    if File::create_new(".openapi/config.toml").is_ok() {
        created_entities += 1;
        println!("Created config file {}", ".openapi/config.toml".bold());
    }

    if created_entities > 0 {
        println!();
        println!("{}", "OpenAPI Manager is now setup 🎉".green());
        println!("Pull or create flavours by executing");
        println!();
        println!("oam pull {}", "<flavour>".blue());
        println!();
        println!("oam create {}", "<flavour>".blue());
    } else {
        println!("{}", "OpenAPI Manager is already setup ✨".green());
    }
    println!();
    Ok(())
}
