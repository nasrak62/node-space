use clap::Args;

#[derive(Args)]
pub struct AddDependencyArgs {
    pub name: String,

    #[arg(short, long)]
    pub group: Option<String>,
}

#[derive(Args)]
pub struct UpdateDependencyArgs {
    pub name: String,

    #[arg(short, long)]
    pub group: Option<String>,
}
