//use std::io::{BufReader, BufWriter};
use crate::cli_progress::ProgressBar;

mod cli_progress;

mod app_state;
mod xkcd;
use app_state::AppState;

use anyhow::{Context, Result};
//use serde::{Deserialize, Serialize};
use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long, default_value_t = String::from("comics"))]
    comic_dir: String,

    #[arg(short, long, default_value_t = String::from("xkcd_sync_state.json"))]
    sync_state_file: String,
}

fn main() -> Result<()> {
    let progress_bar = ProgressBar {
        full_chars: Vec::from(cli_progress::UNICODE_BAR_FULL_CHARS),
        empty_char: ' ',
        ..ProgressBar::default()
    };

    let cli = Cli::parse();
    let comic_dir = &cli.comic_dir;
    let sync_state_file = &cli.sync_state_file;

    fs::create_dir_all(comic_dir)
        .context(format!("create commic storage directory {comic_dir}"))?;

    let mut sync_state = AppState::from_file(&sync_state_file.to_string()).unwrap();
    let last_num = sync_state.last_num;
    progress_bar.update(0f32, "Fetching latest comic information...")?;

    for num in 1..=last_num {
        //let mut already_updated = false;

        let result = sync_state.get_xkcd(num);
        match result {
            Ok(xkcd) => {
                let skipped = xkcd.save_image_file(&comic_dir);
                if skipped {
                    sync_state.skipped += 1;
                } else {
                    sync_state.updated += 1;
                }
            }
            Err(error) => {
                println!(
                    "Error retrieving metadata for #{num}: {err}",
                    err = error.root_cause()
                );
                println!("Note: Skipping #{num} as it will be retieved next time.");
                continue;
            }
        }
        
        if sync_state.updated > 0 && sync_state.updated % 5 == 0 {
            progress_bar.update(
                num as f32 / last_num as f32 * 100f32,
                &format!("Saving sync state to {file}", file = sync_state_file),
            )?;
            let _ = sync_state.save_progress();
        }
    }

    let _ = sync_state
        .save_progress()
        .context("Failed to save sync state");

    println!(
        "Finished sync run: Updated {} comics, skipped {} comics.",
        sync_state.updated, sync_state.skipped
    );

    Ok(())
}
