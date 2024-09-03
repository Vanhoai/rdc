use clap::{Args, Parser, Subcommand};
use std::process::Command;

mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "stringer - a simple CLI to transform and inspect strings",
    long_about = "stringer is a super fancy CLI (kidding) One can use stringer to modify or inspect strings straight from the terminal"
)]
pub struct RdcCli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/**
 * Commands:
 * - New project: cargo rdc new <project-name>
 * - Add primary adapters: cargo rdc adapter <feature-name>
 */
#[derive(Subcommand)]
enum Commands {
    Reverse(Reverse),
    Inspect(Inspect),
    New(New),
}

#[derive(Args)]
struct New {
    #[arg(value_name = "PROJECT_NAME")]
    project_name: String,
}

#[derive(Args)]
struct Reverse {
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

fn main() {
    let cli = RdcCli::parse();

    match &cli.command {
        Some(Commands::Reverse(name)) => match name.string {
            Some(ref _name) => {
                let reverse = api::stringer::reverse(_name);
                println!("{}", reverse);
            },
            None => {
                println!("Please provide a string to reverse");
            },
        },
        Some(Commands::Inspect(name)) => match name.string {
            Some(ref _name) => {
                let (res, kind) = api::stringer::inspect(_name, name.only_digits);

                let mut plural_s = "s";
                if res == 1 {
                    plural_s = "";
                }

                println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
            },
            None => {
                println!("Please provide a string to inspect");
            },
        },
        Some(Commands::New(name)) => {
            // if let Err(e) = Command::new("mkdir").arg(&name.project_name).status() {
            //     eprintln!("Failed to create directory: {}", e);
            //     return;
            // }

            // run cargo new <name>
            if let Err(e) = Command::new("cargo").args(["new", &name.project_name]).status() {
                eprintln!("Failed to create new project: {}", e);
                return;
            }

            // Running additional commands inside the new project directory
            // if let Err(e) = Command::new("sh")
            //     .arg("-c")
            //     .arg(format!("echo 'Initialized project: {}' > {}/README.md", name.project_name, name.project_name))
            //     .status()
            // {
            //     eprintln!("Failed to initialize project files: {}", e);
            // }
            println!("Creating new project: {}", name.project_name);
        },
        None => {},
    }
}
