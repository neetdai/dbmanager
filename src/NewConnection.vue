<script setup lang="ts">
import { Ref, ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { ConnectionConfig, DatabaseType, SSHConfig } from "./db_config";

const host: Ref<string> = ref("");
const port: Ref<number> = ref(0);
const username: Ref<string> = ref("");
const password: Ref<string> = ref("");
const database_name: Ref<string> = ref("");
const database_type: Ref<DatabaseType> = ref(DatabaseType.Mysql);
const display_ssh_config: Ref<boolean> = ref(false);
const ssh_host: Ref<string> = ref("");
const ssh_port: Ref<number> = ref(22);
const ssh_username: Ref<string> = ref("");
const ssh_password: Ref<string> = ref("");
const display_alert: Ref<boolean> = ref(false);
const alert_text: Ref<string> = ref("");
const alert_type: Ref<"error" | "success" | "info" | "warning"> = ref("success");
const showPassword: Ref<boolean> = ref(false);
const showSSHPassword: Ref<boolean> = ref(false);

const build_config = (): ConnectionConfig => {
  let config: ConnectionConfig = new ConnectionConfig();
  config.host = host.value;
  config.port = Number(port.value);
  config.username = username.value;
  config.password = password.value;
  config.database_name = database_name.value;
  config.database_type = database_type.value;
  if (display_ssh_config.value) {
    config.ssh_config = {
      host: ssh_host.value,
      port: Number(ssh_port.value),
      username: ssh_username.value,
      password: ssh_password.value,
    };
  } else {
    config.ssh_config = null;
  }

  return config;
};

const test_connection = async () => {
  let config = build_config();
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

const save_config = async() => {
  let config = build_config();
  try {
    await invoke("save_config", {config});
    display_alert.value = true;
    alert_text.value = "保存成功";
    alert_type.value = "success";
  } catch (e) {
    display_alert.value = true;
    alert_text.value = e as string;
    alert_type.value = "error";
  }
};

</script>

<template>
  <v-card class="mx-auto" max-width="800" elevation="8">
    <v-card-title class="primary white--text">
      <v-icon left color="white">mdi-database</v-icon>
      数据库连接配置
    </v-card-title>
    
    <v-card-text class="pa-6">
      <v-form>
        <v-select
          v-model="database_type"
          :items="['MySQL']"
          label="数据库类型"
          prepend-icon="mdi-database"
          outlined
          dense
          class="mb-4"
        ></v-select>

        <v-row>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="host"
              label="主机地址"
              prepend-icon="mdi-server"
              outlined
              dense
              clearable
            ></v-text-field>
          </v-col>
          <v-col cols="12" md="3">
            <v-text-field
              v-model="port"
              label="端口"
              prepend-icon="mdi-ethernet"
              outlined
              dense
              type="number"
            ></v-text-field>
          </v-col>
          <v-col cols="12" md="3">
            <v-text-field
              v-model="database_name"
              label="数据库名称"
              prepend-icon="mdi-database"
              outlined
              dense
            ></v-text-field>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="username"
              label="用户名"
              prepend-icon="mdi-account"
              outlined
              dense
            ></v-text-field>
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="password"
              label="密码"
              prepend-icon="mdi-lock"
              outlined
              dense
              :type="showPassword == true ?'text' : 'password'"
              :append-icon="showPassword ? 'mdi-eye-off' : 'mdi-eye'"
              @click:append="showPassword = !showPassword"
            ></v-text-field>
          </v-col>
        </v-row>

        <v-expand-transition>
          <div v-if="display_ssh_config">
            <v-divider class="my-4"></v-divider>
            
            <v-row>
              <v-col cols="12">
                <v-subheader class="pl-0">
                  <v-icon left color="primary">mdi-ssh</v-icon>
                  SSH 隧道配置
                </v-subheader>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="ssh_host"
                  label="SSH主机"
                  prepend-icon="mdi-server-network"
                  outlined
                  dense
                ></v-text-field>
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="ssh_port"
                  label="SSH端口"
                  prepend-icon="mdi-network"
                  outlined
                  dense
                  type="number"
                ></v-text-field>
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="ssh_username"
                  label="SSH用户名"
                  prepend-icon="mdi-account-key"
                  outlined
                  dense
                ></v-text-field>
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="ssh_password"
                  label="SSH密码"
                  prepend-icon="mdi-key"
                  outlined
                  dense
                  :type="showSSHPassword == true ?'text' : 'password'"
                  :append-icon="showSSHPassword ? 'mdi-eye-off' : 'mdi-eye'"
                  @click:append="showSSHPassword = !showSSHPassword"
                ></v-text-field>
              </v-col>
            </v-row>
          </div>
        </v-expand-transition>
      </v-form>

      <v-alert
        v-model="display_alert"
        :type="alert_type"
        dismissible
        class="mt-4"
        transition="scale-transition"
      >
        {{ alert_text }}
      </v-alert>

      <v-switch
        v-model="display_ssh_config"
        label="启用SSH隧道"
        color="primary"
        class="mt-2"
      ></v-switch>
    </v-card-text>

    <v-card-actions class="px-6 pb-4">
      <v-spacer></v-spacer>
      <v-btn
        color="primary"
        @click="test_connection"
        depressed
        large
      >
        <v-icon left>mdi-connection</v-icon>
        测试连接
      </v-btn>
      <v-btn
        color="success"
        @click="save_config"
        depressed
        large
      >
        <v-icon left></v-icon>
        保存连接
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<style scoped>
.v-card {
  margin-top: 24px;
  border-radius: 8px;
}
.v-subheader {
  height: auto;
  padding-left: 0;
  font-size: 1rem;
}
</style>