<script setup lang="ts">
import { Ref, ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";

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
  database: string;
  database_type: DatabaseType;
  ssh_config: SSHConfig | null;

  constructor() {
    this.database_type = DatabaseType.Mysql;
    this.ssh_config = null;
    this.host = "";
    this.port = 0;
    this.username = "";
    this.password = "";
    this.database = "";
  }
}

const host: Ref<string> = ref("");
const port: Ref<number> = ref(0);
const username: Ref<string> = ref("");
const password: Ref<string> = ref("");
const database: Ref<string> = ref("");
const database_type: Ref<DatabaseType> = ref(DatabaseType.Mysql);
const display_ssh_config: Ref<boolean> = ref(false);
const ssh_host: Ref<string> = ref("");
const ssh_port: Ref<number> = ref(0);
const ssh_username: Ref<string> = ref("");
const ssh_password: Ref<string> = ref("");
const display_alert: Ref<boolean> = ref(false);
const alert_text: Ref<string> = ref("");
const alert_type: Ref<"error" | "success" | "info" | "warning"> = ref("success");

const test_connection = async () => {
  let config: ConnectionConfig = new ConnectionConfig();
  config.host = host.value;
  config.port = port.value;
  config.username = username.value;
  config.password = password.value;
  config.database = database.value;
  config.database_type = database_type.value;
  if (display_ssh_config.value) {
    config.ssh_config = {
      host: ssh_host.value,
      port: ssh_port.value,
      username: ssh_username.value,
      password: ssh_password.value,
    };
  } else {
    config.ssh_config = null;
  }
  try {
    await invoke("test_connection", {config});
    display_alert.value = true;
    alert_text.value = "连接成功";
    alert_type.value = "success";
  } catch (e) {
    display_alert.value = true;
    alert_text.value = e as string;
    alert_type.value = "error";
  }
}

</script>
<script lang="ts">
  export default {
    name: "NewConnection",
  };
</script>

<template>
  <main>
    <v-form>
      <v-container>
        <v-row>
          <v-col>
            <v-select v-model="database_type" :items="['Mysql']" label="数据库类型"></v-select>
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="12" md="4">
            <v-text-field v-model="host" label="主机地址"></v-text-field>
          </v-col>
          <v-col cols="12" md="4">
            <v-text-field v-model="port" label="主机端口"></v-text-field>
          </v-col>
          <v-col cols="12" md="4">
            <v-text-field v-model="database" label="数据库名称"></v-text-field>
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="6" md="3">
            <v-text-field v-model="username" label="用户名称"></v-text-field>
          </v-col>
          <v-col cols="6" md="3">
            <v-text-field v-model="password" label="密码"></v-text-field>
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="12" md="2">
            <v-checkbox v-model="display_ssh_config" label="设置ssh连接"></v-checkbox>
          </v-col>
          <v-col cols="12" md="3" v-if="display_ssh_config">
            <v-text-field v-model="ssh_host" label="ssh主机"></v-text-field>
          </v-col>
          <v-col cols="12" md="3" v-if="display_ssh_config">
            <v-text-field v-model="ssh_port" label="ssh端口"></v-text-field>
          </v-col>
          <v-col cols="12" md="3" v-if="display_ssh_config">
            <v-text-field v-model="ssh_host" label="ssh用户名称"></v-text-field>
          </v-col>
          <v-col cols="12" md="3" v-if="display_ssh_config">
            <v-text-field v-model="ssh_port" label="ssh密码"></v-text-field>
          </v-col>
        </v-row>
      </v-container>
    </v-form>
    <v-btn color="primary" @click="test_connection">测试连接</v-btn>
    <v-alert v-model="display_alert" :text="alert_text" :type="alert_type" closable></v-alert>
  </main>
</template>