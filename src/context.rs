use std::env;
use std::path::PathBuf;
use anyhow::{Result, Context as AnyhowContext};

pub struct Context {
    pub commands_dir: PathBuf,
    pub tracks_dir: PathBuf,
    pub writeups_dir: PathBuf,
    pub track_name_file_path: PathBuf,
}

impl Context {
    pub fn new() -> Result<Self> {
        let records_dir = env::var("ORS_RECORDS_DIR")
            .context("ORS_RECORDS_DIR is not set")?;

        let records_dir = PathBuf::from(records_dir);

        let commands_dir = records_dir.join("commands");
        let tracks_dir = records_dir.join("tracks");
        let writeups_dir = records_dir.join("writeups");
        let track_name_file_path = tracks_dir.join(".track");

        Ok(Self {
            commands_dir,
            tracks_dir,
            writeups_dir,
            track_name_file_path,
        })
    }
}
