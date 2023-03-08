use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand, Debug)]
enum Command {
    #[command(arg_required_else_help = true)]
    Get(Get),
    #[command(arg_required_else_help = true)]
    Post(Post),
}
#[derive(Debug, Args)]
struct Get {
    url: String,
}
#[derive(Debug, Args)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn main() {
    let opts = Cli::parse();
    println!("Hello, world! {:?}", opts);
}
