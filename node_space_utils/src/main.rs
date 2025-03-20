use node_space_utils::handle_cli::handle_cli;

#[tokio::main]
async fn main() {
    match handle_cli().await {
        Ok(value) => println!("{value}"),
        Err(err) => println!("{err}"),
    }
}
