use clap::Parser;
use git_derivative::args::{Cli, Subcommands};

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Init => {
            println!("init");
        }
        Subcommands::Install => {
            println!("install");
        }
        Subcommands::Update { force } => {
            println!("update");
            println!("force: {}", force);
        }
    }
}
