use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::fs;


#[derive(Deserialize, Serialize, Debug)]
pub struct Xkcd {
    month: String,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    transcript: String,
    alt: String,
    title: String,
    day: String,
    pub num: usize,
    pub  img: String,
}

impl Xkcd {
    pub fn save_image_file(&self, comic_dir: &str) -> bool { 
      let mut skipped= false;
      let comic_target_path: PathBuf = self.create_image_file_path( comic_dir).unwrap();
      if comic_target_path.try_exists().unwrap_or_default()
      {
          skipped = true;
      } else {
          match self.download_xkcd_image_to_dir(&comic_target_path) {
              Ok(_) => {
                  //if !already_updated {
                  //    sync_state.updated += 1;
                  //}
              }
              Err(error) => {
                  eprintln!(
                      "Error retrieving image for #{num}: {err}",
                      err = error.root_cause(), num=self.num
                  );
                  eprintln!("Note: Skipping #{num} as it will be retieved next time.",
                   num=self.num);
                  return false;  
              }
          }
        }
        return skipped;
    }

    fn create_image_file_path(&self,  comic_dir: &str) -> Result<PathBuf> {
      let num = self.num;
        let comic_file_name = self.img.split('/').last().context(format!(
            "extracting filename from image url {url}",
            url = comic_dir
        ))?;
        let mut comic_path_name = PathBuf::new();
        comic_path_name.push(comic_dir);
        comic_path_name.push(format!("{num:05}_{file}", file = comic_file_name));
        
        Ok(comic_path_name)
    }

    pub fn download_xkcd_image_to_dir(&self, target_file: &Path) -> Result<()> {
       
        let img_reader = ureq::get(&self.img)
            .call()
            .context(format!("fetching {url}", url = self.img))?
            .into_reader();
        let writer =
            fs::File::create(target_file).context(format!("open {target_file:?} for writing"))?;
    
        std::io::copy(&mut BufReader::new(img_reader), &mut BufWriter::new(writer)).context(
            format!(
                "stream data from {url} to {file:?}",
                url = self.img,
                file = target_file
            ),
        )?;
        Ok(())
    }
}

