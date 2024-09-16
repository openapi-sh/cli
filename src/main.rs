use clap::Parser;
use oam::{
    cli::{Arguments, Command},
    commands,
    config::AppConfig,
};

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::parse();

    match arguments.command {
        Command::Init => commands::init(),
        Command::Run { schema, flavour } => commands::run(AppConfig::new(schema, flavour)),
        Command::Create { name } => commands::create(name),
        Command::Pull { name } => commands::pull(name),
        Command::Push => todo!(),
    }?;

    Ok(())
}
