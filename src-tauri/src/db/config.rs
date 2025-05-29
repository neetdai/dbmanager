use serde::{Deserialize, Serialize};
use tauri::{command, Runtime};


#[derive(Debug, Serialize, Deserialize)]
pub enum DataBaseType {
    Mysql,
    Postgres,
    Sqlite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SSHConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataBaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_type: DataBaseType,
    pub ssh_config: Option<SSHConfig>,
}

#[tauri::command]
pub async fn test_connection(config: DataBaseConfig) -> Result<(), String> {
    match config.database_type {
        DataBaseType::Mysql => {

        }
        _ => {

        }
    }

    Ok(())
}