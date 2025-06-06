use std::{net::SocketAddr, sync::Arc};

use russh::client::{connect, Config, Handle, Handler};

use crate::db::config::{DataBaseConfig, SSHConfig};

pub(crate) struct ClientSession {
    inner: Handle<SSHClient>,
    ssh_host: String,
    ssh_port: u16,
}

impl ClientSession {
    pub(crate) async fn connect(ssh_config: &SSHConfig) -> Result<Self, russh::Error> {
        let client = SSHClient {};
        let config = Arc::new(Config::default());
        let mut inner = connect(config, format!("{}:{}", ssh_config.host, ssh_config.port), client).await?;
        inner.authenticate_password(&ssh_config.username, &ssh_config.password).await?;
        Ok(Self { inner, ssh_host: ssh_config.host.clone(), ssh_port: ssh_config.port })
    }

    pub(crate) async fn keep_alive(&mut self) -> Result<(), russh::Error> {
        self.inner.send_keepalive(true).await?;
        Ok(())
    }

    pub(crate) async fn forward(self, target_host: &str, target_port: u16) -> Result<(), russh::Error> {
        
        self.inner.channel_open_direct_tcpip(
            self.ssh_host.clone(),
             self.ssh_port as _,
              target_host,
               target_port as _
        ).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct SSHClient {

}

impl Handler for SSHClient {
    type Error = russh::Error;

    async  fn check_server_key(&mut self,server_public_key: &russh::keys::ssh_key::PublicKey,) -> Result<bool,Self::Error> {
        Ok(true)
    }
}