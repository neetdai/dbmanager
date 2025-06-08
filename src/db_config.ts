enum DatabaseType {
  Mysql = "Mysql",
}

class SSHConfig {
  host: string;
  port: number;
  username: string;
  password: string;

  constructor() {
      this.port = 22;
      this.host = "";
      this.username = "";
      this.password = "";
  }
}

class ConnectionConfig {
  host: string;
  port: number;
  username: string;
  password: string;
  database_name: string;
  database_type: DatabaseType;
  ssh_config: SSHConfig | null;

  constructor() {
      this.database_type = DatabaseType.Mysql;
      this.ssh_config = null;
      this.host = "";
      this.port = 0;
      this.username = "";
      this.password = "";
      this.database_name = "";
  }
}

export {
  DatabaseType,
  ConnectionConfig,
  SSHConfig,
};