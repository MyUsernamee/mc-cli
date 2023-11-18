use std::error::Error;

use directories::ProjectDirs;
use log::*;

pub fn setup() -> Result<(), Box<dyn Error>> {

    info!("Setting up MC-CLI");

    if let Some(proj_dir) = ProjectDirs::from("com", "MyUsername", "MC-CLI") {

        let data_dir = proj_dir.data_dir();
        let config_dir = proj_dir.config_dir();
        let cache_dir = proj_dir.cache_dir();

        info!("Data directory: {}", data_dir.display());
        info!("Config directory: {}", config_dir.display());
        info!("Cache directory: {}", cache_dir.display());

        // We create the directories here
        std::fs::create_dir_all(data_dir)?;
        std::fs::create_dir_all(config_dir)?;
        std::fs::create_dir_all(cache_dir)?;

        // We populate the config file
        

        Ok(())
    }
    else {

        return Err("Could not find project directories".into());

    }

}