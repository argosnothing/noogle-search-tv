mod cache;
mod commands;
mod data;
mod format;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::io::{self, ErrorKind};

#[derive(Debug, Clone, ValueEnum)]
enum Namespace {
    Lib,
    Builtins,
    Pkgs,
}

impl Namespace {
    fn as_str(&self) -> &str {
        match self {
            Namespace::Lib => "lib",
            Namespace::Builtins => "builtins",
            Namespace::Pkgs => "pkgs",
        }
    }
}

#[derive(Parser)]
#[command(name = "noogle-search")]
#[command(about = "Search Noogle functions for television/fzf")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Filter {
        namespace: Namespace,
    },
    Print {
        #[arg(long)]
        filter: Option<String>,
    },
    Preview {
        name: String,
    },
    OpenSource {
        name: String,
    },
    OpenNoogle {
        name: String,
    },
}

fn main() -> Result<()> {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let result = run();

    if let Err(e) = &result {
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            if io_err.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
        }
    }

    result
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Filter { namespace }) => {
            commands::search::execute(Some(namespace.as_str().to_string()))?;
        }
        Some(Commands::Print { filter }) => {
            let response = cache::load_data()?;
            commands::print::execute(&response, filter.as_deref());
        }
        Some(Commands::Preview { name }) => {
            let response = cache::load_data()?;
            commands::preview::execute(&response, &name)?;
        }
        Some(Commands::OpenSource { name }) => {
            let response = cache::load_data()?;
            commands::open_source::execute(&response, &name)?;
        }
        Some(Commands::OpenNoogle { name }) => {
            let response = cache::load_data()?;
            commands::open_noogle::execute(&response, &name)?;
        }
        None => {
            commands::search::execute(None)?;
        }
    }

    Ok(())
}
