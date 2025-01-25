use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub struct LinkArgs {
    pub name: Option<String>,

    #[arg(short, long)]
    pub show: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Link(LinkArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
