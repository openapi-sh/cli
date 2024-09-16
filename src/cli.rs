use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "OpenAPI Manager", version)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Push,
    Create {
        name: String,
    },
    Pull {
        name: String,
    },
    Run {
        #[arg(short, long)]
        schema: Option<String>,
        #[arg(short, long)]
        flavour: Option<String>,
    },
}

impl Command {
    pub fn call(self) {}
}
