mod preferences;
mod commands;
mod utils;

use commands::{add::add, all::all, list::list, main::main as mainCommand, remove::remove};
use preferences::preferences::Preferences;
use clap::{command, ArgGroup, Parser};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "mounter",
    version = "0.1.0",
    about = "Simple program for mounting your drives on Linux.",
    group = ArgGroup::new("command")
        .args(&["list", "add", "remove"])
        .multiple(false)
)]
struct Cli {
    #[arg(short = 'l', long = "list", help = "Lists all the saved mount points.")]
    list: bool,

    #[arg(short = 'a', long = "add", help = "Adds new mount point to the list.", value_name = "NAME")]
    add: Option<String>,

    #[arg(short = 'r', long = "remove", help = "Remove a mount point from the list.", value_name = "NAME")]
    remove: Option<String>,

    #[arg(long = "all", help = "Mount any connected drive.")]
    all: bool,

    #[arg(long = "config", help = "Path to the configuration file.", value_name = "FILE")]
    config: Option<String>,

    #[arg(long = "sudo", help = "Specify whether to use sudo when mounting.", value_name = "OPTION", default_value = "false")]
    sudo: bool,

    #[arg(long = "no-filter", help = "Don't filter drives in the --all.", requires = "all")]
    no_filter: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let loaded_prefs = Preferences::load(&cli.config).await.unwrap_or_else(|err| {
        eprintln!("Error loading preferences: {}", err);
        process::exit(1);
    });

    if cli.list {
        list(loaded_prefs);
        return;
    }

    if cli.add.is_some() {
        add(cli.add.unwrap(), loaded_prefs, &cli.config).await;
        return;
    }

    if cli.remove.is_some() {
        remove(cli.remove.unwrap(), loaded_prefs, &cli.config).await;
        return;
    }

    if cli.all {
        all(cli.no_filter, cli.sudo);
        return;
    }

    // Main
    mainCommand(loaded_prefs, cli.sudo);
}
