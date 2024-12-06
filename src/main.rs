mod preferences;
mod commands;
mod utils;

use commands::{add::add, all::all, get_config::get_config, list::list, main::main as mainCommand, remove::remove, set_config::set_config};
use preferences::preferences::Preferences;
use clap::{command, ArgGroup, Parser};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "mounter",
    version = env!("CARGO_PKG_VERSION"),
    about = "Simple program for mounting your drives on Linux.",
    author = "Jakub Žitník",
    group = ArgGroup::new("command")
        .args(&["list", "add", "remove", "config_set"])
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
    config_file: Option<String>,

    #[arg(long = "no-filter", help = "Don't filter drives in the --all.", requires = "all")]
    no_filter: bool,

    #[arg(long = "config-set", help = "Set a configuration value.", value_names = &["KEY", "VALUE"])]
    config_set: Option<Vec<String>>,

    #[arg(long = "config-get", help = "Get the config value.", value_names = &["KEY"])]
    config_get: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let loaded_prefs = Preferences::load(&cli.config_file).await.unwrap_or_else(|err| {
        eprintln!("Error loading preferences: {}", err);
        process::exit(1);
    });

    if cli.list {
        list(loaded_prefs);
        return;
    }

    if cli.add.is_some() {
        add(cli.add.unwrap(), loaded_prefs, &cli.config_file).await;
        return;
    }

    if cli.remove.is_some() {
        remove(cli.remove.unwrap(), loaded_prefs, &cli.config_file).await;
        return;
    }

    if cli.all {
        all(cli.no_filter, loaded_prefs);
        return;
    }

    if cli.config_set.is_some() {
        set_config(loaded_prefs, cli.config_set.unwrap(), &cli.config_file).await;
        return;
    }

    if cli.config_get.is_some() {
        get_config(loaded_prefs, cli.config_get.unwrap()).await;
        return;
    }

    // Main
    mainCommand(loaded_prefs);
}
