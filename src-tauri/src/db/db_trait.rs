use super::config::DataBaseConfig;


pub(crate) trait DBAdaptorTrait: Sized {
    type Connection;
    type ConnectionError;

    async fn try_connect(config: DataBaseConfig) -> Result<Self::Connection, Self::ConnectionError>;

    async fn new_pool(config: DataBaseConfig) -> Result<Self, Self::ConnectionError>;
}