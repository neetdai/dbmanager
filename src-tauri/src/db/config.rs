use serde::{Deserialize, Serialize};
use tauri::{command, Runtime};
use std::path::PathBuf;
use crate::ssh_client;
use tokio::{fs::{File, OpenOptions}, io::{AsyncReadExt, AsyncWriteExt}};
use super::{db_trait::DBAdaptorTrait, mysql::MysqlAdapter};


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
    pub database_name: String,
    pub database_type: DataBaseType,
    pub ssh_config: Option<SSHConfig>,
}

#[tauri::command]
pub async fn test_connection(config: DataBaseConfig) -> Result<(), String> {
    if let Some(ssh_config) = &config.ssh_config {
        ssh_client::ClientSession::connect(ssh_config).await.map_err(|e| e.to_string())?;
    }

    let result= match config.database_type {
        DataBaseType::Mysql => {
            MysqlAdapter::try_connect(config).await
                .map(|_| ())
                .map_err(|e| e.to_string())
        },
        _ => {
            Ok(())
        }
    };

    result
}

#[tauri::command]
pub async fn save_config(config: DataBaseConfig) -> Result<(), String> {
    let config_path = PathBuf::from(format!("../db_configs/{}.json", config.database_name));
    dbg!(&config_path);
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .read(true)
        .open(config_path)
        .await.map_err(|e| e.to_string())?;
    let content = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes()).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_db_configs() -> Result<Vec<DataBaseConfig>, String> {
    let mut configs = Vec::new();
    let config_path = PathBuf::from("../db_configs");
    let mut entries = tokio::fs::read_dir(config_path).await.map_err(|e| e.to_string())?;
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        let mut file = File::open(path).await.map_err(|e| e.to_string())?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.map_err(|e| e.to_string())?;
        let config: DataBaseConfig = serde_json::from_reader(buf.as_slice()).map_err(|e| e.to_string())?;
        configs.push(config);
    }

    Ok(configs)
}