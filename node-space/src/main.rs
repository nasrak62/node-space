use node_space_utils::handle_cli::handle_cli;

fn main() {
    match handle_cli() {
        Ok(value) => println!("{value}"),
        Err(err) => println!("{err}"),
    }
}
