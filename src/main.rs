use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use comfy_table::Table;
use std::path::{Path, PathBuf};

use achievement_hunter::commands::achievements::get_achievements;

use achievement_hunter::settings::{get_config_filepath, Settings};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(visible_alias = "get")]
    GetAchievements,
}

fn main() -> Result<()> {
    let settings_filepath: String =
        get_config_filepath().context("Failed to determine the config filepath")?;
    let settings_file_path = Path::new(&settings_filepath);
    let settings = Settings::new(&settings_file_path).context("Failed to initialise settings")?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::GetAchievements) => {
            let achievements = get_achievements(&settings).context("Failed to get achievements")?;

            let formatted_achievements: Vec<Vec<String>> = achievements
                .into_iter()
                .map(|achievement| {
                    vec![
                        achievement.display_name,
                        achievement.description.unwrap_or(String::from("Hidden")),
                        achievement.achieved.to_string(),
                        match achievement.achieved_at {
                            Some(value) => value.to_string(),
                            None => String::from("Not achieved... yet"),
                        },
                    ]
                })
                .collect();

            let mut table = Table::new();
            table
                .set_header(vec!["Name", "Description", "Achieved?", "Achieved at"])
                .add_rows(formatted_achievements);
            println!("{table}");
            Ok(())
        }
        None => {
            bail!("Invalid command chosen")
        }
    }
}
