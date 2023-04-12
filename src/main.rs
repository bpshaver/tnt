mod cli;
mod io;
mod task;

use crate::cli::{Args, TntCommand};

fn main() {
    let args = Args::parse_args();
    match args.command {
        None => println!("No subcommand provided, showing current task..."),
        Some(command) => match command {
            TntCommand::Add {
                name,
                parent,
                switch,
            } => {
                println!(
                    "Name: {:#?}, parent: {:#?}, switch: {}",
                    name, parent, switch
                )
            }
            _ => todo!("Command not supported yet!"),
        },
    }
}
