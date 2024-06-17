use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::process;

mod folder_organizer;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Organizes the files in the folder.
    Organize(OrganizeArgs),
}

#[derive(Args)]
struct OrganizeArgs {
    folder_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Organize(args) => {
            if let Err(e) = validate_args(args) {
                exit_with_error(&e);
            };

            if let Err(e) = folder_organizer::organize_folder(&args.folder_path) {
                exit_with_error(&e.to_string());
            }
        }
    }
}

fn validate_args(args: &OrganizeArgs) -> Result<(), String> {
    if args.folder_path.exists() {
        Ok(())
    } else {
        Err(String::from("Folder does not exist."))
    }
}

fn exit_with_error(error: &str) {
    eprintln!("Application error: {}", error);
    process::exit(1);
}
