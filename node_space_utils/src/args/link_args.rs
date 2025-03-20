use clap::Args;

#[derive(Args)]
pub struct LinkArgs {
    pub name: Option<String>,

    #[arg(short, long)]
    pub show: bool,

    #[arg(short, long)]
    pub graph: bool,

    #[arg(short, long)]
    pub output_dir: Option<String>,
}
