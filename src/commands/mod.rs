// Copyright 2023 myujiku (https://github.com/myuujiku)

mod clean;
mod edit;
mod load;
mod new;
mod pack;
mod profile;
mod sync;
mod unload;

use clap::{Parser, Subcommand};

pub fn parse() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sync(args) => args.eval(),
        Commands::Load(args) => args.eval(),
        Commands::Unload(args) => args.eval(),
        Commands::New(args) => args.eval(),
        Commands::Pack(args) => args.eval(),
        Commands::Edit(args) => args.eval(),
        Commands::Profile(args) => args.eval(),
        Commands::Clean(args) => args.eval(),
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sync(sync::Args),
    Load(load::Args),
    Unload(unload::Args),
    New(new::Args),
    Pack(pack::Args),
    Edit(edit::Args),
    Profile(profile::Args),
    Clean(clean::Args),
}
