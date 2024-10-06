use crate::cli_progress::ProgressBar;

mod cli_progress;

mod app_state;
mod xkcd;
use crate::xkcd::Xkcd;

use app_state::AppState;

use anyhow::{Context, Result};
//use serde::{Deserialize, Serialize};
use std::fs;

use clap::Parser;
use std::sync::{Arc, Mutex};
//use std::time::Duration;
use threadpool::ThreadPool;
#[derive(Parser)]
struct Cli {
    /// Optional name to operate on
    #[arg(short, long, default_value_t = String::from("comics"))]
    comic_dir: String,

    #[arg(short, long, default_value_t = String::from("xkcd_sync_state.json"))]
    sync_state_file: String,

    #[arg(short, long, default_value_t = 4)]
    num_threads: usize,
}

fn main() -> Result<()> {
    let progress_bar = ProgressBar {
        full_chars: Vec::from(cli_progress::UNICODE_BAR_FULL_CHARS),
        empty_char: ' ',
        ..ProgressBar::default()
    };
    let cli = Cli::parse();
    let comic_dir = cli.comic_dir.clone();
    let sync_state_file = &cli.sync_state_file;
    let num_threads = cli.num_threads;

    fs::create_dir_all(&comic_dir)
        .context(format!("create commic storage directory {comic_dir}"))?;

    // Create a thread pool with 4 threads.
    let pool = ThreadPool::new(num_threads);

    // Shared vector to store the results, wrapped in Arc and Mutex for thread-safe access.
    //let results = Arc::new(Mutex::new(Vec::new()));

    let sync_state = AppState::from_file(&sync_state_file.to_string()).unwrap();
    let last_num = sync_state.last_num;
    progress_bar.update(0f32, "Fetching latest comic information...")?;

    for num in 1..=last_num {
        // Clone Arc so that each thread has ownership of the reference.
        //let results = Arc::clone(&results);

        // Execute each task in the thread pool.
        
        
        let dir = format!("{}", comic_dir.clone());
        pool.execute(move || {
          
          let result1 = Xkcd::get_xkcd(num);
          let mut _skipped = false;
          //let results = Arc::clone(&results);
            match result1 {
                Ok(xkcd) => {
                    _skipped = xkcd.save_image_file(&dir);
                    
                }
                Err(error) => {
                    println!(
                        "Error retrieving metadata for #{num}: {err}",
                        err = error.root_cause()
                    );
                    println!("Note: Skipping #{num} as it will be retieved next time.");
                    //continue;
                }
            }

            // Store the result.
            //let mut results = results.lock().unwrap();
            //results.push((num, skipped));

            //if sync_state.updated > 0 && sync_state.updated % 5 == 0 {
                // progress_bar.update(
                //     num as f32 / last_num as f32 * 100f32,
                //     &format!("Saving sync state to {file}", file = sync_state_file),
                // );
            //    let _ = sync_state.save_progress();
            //}

            //let mut results = results.lock().unwrap();
            //results.push((num, square));
        });
    }

    // Wait for all tasks to finish by dropping the pool.
    pool.join();
    let _ = sync_state
        .save_progress()
        .context("Failed to save sync state");

    // println!(
    //     "Finished sync run: Updated {} comics, skipped {} comics.",
    //     sync_state.updated, sync_state.skipped
    // );
    Ok(())
}
