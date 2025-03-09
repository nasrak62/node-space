use clap::Args;

#[derive(Args)]
pub struct CoordinatorStartArgs {}

#[derive(Args)]
pub struct CoordinatorLogArgs {
    #[arg(short, long)]
    pub watch: bool,
}
