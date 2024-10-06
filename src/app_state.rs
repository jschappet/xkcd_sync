use std::collections::HashMap;
use anyhow::{Context, Result};
use std::collections::hash_map::Entry;

use crate::xkcd::*;
use std::io::{BufReader, BufWriter};
use std::fs;

type SyncState = HashMap<usize, Xkcd>;

#[derive(Debug)]
pub struct AppState {
    
    pub state: SyncState,
    //pub progress_bar: ProgressBar, 
    pub sync_state_file: String, 
    pub last_num: usize,
    pub updated: i32 ,
    pub skipped: i32 ,
}


impl AppState {

    

    pub fn save_progress(&self) -> Result<()> {
        let file = fs::File::create(&self.sync_state_file)
                .context(format!("open {file} for writing", file = self.sync_state_file))?;
            serde_json::to_writer(BufWriter::new(file), &self.state)
                .context("serialize sync state")?;
        Ok(())
    }

    pub fn from_file(sync_state_file: &String) -> Result<Self> {
        println!("Opening {file} as sync state", file = sync_state_file);
        let cur_state = match fs::File::open(sync_state_file) {
            Ok(file) => serde_json::from_reader(BufReader::new(file)).context(format!(
                "deserializing sync state from {file}",
                file = sync_state_file
            ))?,
            Err(_) => SyncState::new(),
        };

        Ok(Self {
            state: cur_state,
            sync_state_file: sync_state_file.clone(),
            last_num: {
                let lastest_url = "https://xkcd.com/info.0.json";
                let latest = fetch_json(lastest_url)?;
                latest.num
            },
            updated: 0 ,
            skipped: 0,
           
        })
    }
}
