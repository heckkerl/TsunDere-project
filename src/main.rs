use clap::{Parser, Subcommand};
// use cliux::Boxed;
// use console::style;
// use indicatif::{ProgressBar, ProgressStyle};
// use promptuity::{PromptInput, Promptuity, Term, prompts::Input, themes::FancyTheme};
use std::{
    // error::Error,
    // fs::{self, File},
    path::Path,
    // time::Duration,
};

mod beatmap_downloader;
mod beatmap_loader;
mod beatmap_types;
mod beatmap_writer;

#[derive(Parser)]
#[command(name = "tsundere-cli")]
#[command(about = "Music on, jerk off", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Exporting beatmaps, Help push for more infomations
    Push {
        /// path to your, Beatmaps folder
        path: String,
    },
    /// Importing beatmaps Help pull for more infomations
    Pull {
        /// the file to be import
        file: String,
        delay: Option<usize>,
    },
    /// In Beta, this command has no use
    Init {},
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Push { path } => {
            beatmap_writer::import(Path::new(&path)).await?;
        }
        Commands::Pull { file, delay } => {
            let bms = beatmap_loader::load(file).await?;
            beatmap_downloader::batch_download(&bms, delay.unwrap_or(3000) as u64).await?;
        }
        Commands::Init {  } => {

        }
    }
    Ok(())
}
