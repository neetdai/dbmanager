use std::time::Duration;

use mysql_async::{Conn, Error as MysqlError, Opts, OptsBuilder, Pool, PoolOpts};

use super::db_trait::DBAdaptorTrait;
use super::config::DataBaseConfig;

#[derive(Debug)]
pub(crate) struct MysqlAdapter {
    pool: Pool,
}

impl MysqlAdapter {
    // pub(crate) fn
}

impl DBAdaptorTrait for MysqlAdapter {
    type Connection = Conn;
    type ConnectionError = MysqlError;

    async fn try_connect(config: DataBaseConfig) -> Result<Self::Connection, Self::ConnectionError> {
        let opts_builder = OptsBuilder::default()
            .ip_or_hostname(config.host)
            .tcp_port(config.port)
            .user(Some(&config.username))
            .pass(Some(&config.password))
            .db_name(Some(&config.database_name))
            .wait_timeout(Some(30));

        let opts = Opts::from(opts_builder);

        Conn::new(opts).await
    }

    async fn new_pool(config: DataBaseConfig) -> Result<Self, Self::ConnectionError> {
        let pool_opts = PoolOpts::new()
            .with_abs_conn_ttl(Some(Duration::from_secs(60)))
            .with_reset_connection(true);

        let conn_opts = OptsBuilder::default()
            .ip_or_hostname(config.host)
            .tcp_port(config.port)
            .user(Some(&config.username))
            .pass(Some(&config.password))
            .db_name(Some(&config.database_name))
            .wait_timeout(Some(30))
            .pool_opts(pool_opts);

        let pool = Pool::new(conn_opts);

        Ok(Self {
            pool,
        })
    }
}