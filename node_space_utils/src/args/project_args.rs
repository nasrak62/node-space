use clap::Args;

#[derive(Args)]
pub struct ProjectArgs {
    pub name: Option<String>,

    #[arg(short, long)]
    pub output_dir: Option<String>,
}
