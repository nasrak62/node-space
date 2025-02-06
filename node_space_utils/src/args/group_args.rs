use clap::Args;

#[derive(Args)]
pub struct GroupsArgs {
    pub name: String,
}

#[derive(Args)]
pub struct GroupShowArgs {
    pub name: Option<String>,
}
