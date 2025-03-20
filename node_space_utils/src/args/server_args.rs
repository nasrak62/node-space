use clap::Args;

#[derive(Args)]
pub struct StartServerArgs {
    pub name: Option<String>,

    #[arg(short, long)]
    pub port: Option<String>,
}

#[derive(Args)]
pub struct ConfigServerArgs {
    /// config name
    pub name: String,
    /// routes should have the format "route_name => project_name, route_name2 => project_name2"
    /// project_name must exist
    pub routes: String,

    #[arg(short, long)]
    /// port number default is 3000
    pub port: Option<String>,

    #[arg(short, long)]
    /// main route to look for index.html if not specified each route will look for its own
    /// index.html
    pub main_route: Option<String>,
}
