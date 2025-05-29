<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import  NavContainer from "./db_nav/NavContainer.vue";

// const greetMsg = ref("");
// const name = ref("");


// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
// let rail: Ref<boolean> = ref(true);
// const toggleRail = () => {
//   rail.value = !rail.value;
// };

</script>

<script lang="ts">
  export default {
    name: 'App',
  }
</script>

<template>
  <router-view></router-view>
</template>

<style scoped>

</style>