use serde::{Serialize, Deserialize};
use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use dirs::config_dir;
use std::collections::HashMap;
use std::path::PathBuf;
use super::config::{is_valid, ValidationResult};
use super::mount_point::MountPoint;

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub saved_mount_points: Vec<MountPoint>,
    pub config: HashMap<String, String>
}

impl Preferences {
    fn get_config_file_path(config: &Option<String>) -> PathBuf {
        match config {
            Some(config_path) => PathBuf::from(config_path),
            None => {
                let mut config_path = config_dir().expect("Unable to determine config directory");
                config_path.push(env!("CARGO_PKG_NAME"));
                config_path.push("preferences.json");
                config_path
            },
        }
    }

    pub async fn load(config: &Option<String>) -> io::Result<Preferences> {
        let path = Self::get_config_file_path(&config);

        if !path.exists() {
            // Default preference here
            return Ok(Preferences {
                saved_mount_points: vec![],
                config: HashMap::new()
            });
        }
        let mut file = File::open(&path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let preferences: Preferences = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Check if config is valid
        for (key, value) in preferences.config.iter() {
            match is_valid(key, value) {
                ValidationResult::ValueError => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Invalid value in \"{}\" reading \"{}\".", key, value),
                    ));
                }
                ValidationResult::KeyError => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unknown key \"{}\" in config.", key),
                    ));
                }
                ValidationResult::Correct => {}
            }
        }

        Ok(preferences)
    }

    pub async fn save(&self, config: &Option<String>) -> io::Result<()> {
        let path = Self::get_config_file_path(config);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let contents = serde_json::to_string_pretty(&self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut file = File::create(path).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }

    // Configuring preferences
    pub async fn add_mount_point(&mut self, mount_point: MountPoint, config_file: &Option<String>) -> io::Result<()> {
        self.saved_mount_points.push(mount_point);
        self.save(config_file).await
    }

    pub async fn remove_mount_point(&mut self, mount_name: String, config_file: &Option<String>) -> io::Result<()> {
        self.saved_mount_points.retain(|mount_point| mount_point.name != mount_name);
        self.save(config_file).await
    }

    pub async fn update_config(&mut self, name: &String, value: &String, config_file: &Option<String>) -> io::Result<()> {
        self.config.insert(String::from(name), String::from(value));
        self.save(config_file).await
    }
}
