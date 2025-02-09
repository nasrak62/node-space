use clap::Args;

#[derive(Args)]
pub struct BuildArgs {
    #[arg(short, long)]
    pub watch: bool,

    #[arg(short, long)]
    pub deamon: bool,

    #[arg(short, long)]
    pub start: bool,
}
