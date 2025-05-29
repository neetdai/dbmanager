<script setup lang="ts">
import { onBeforeUnmount, onMounted, Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import  NavContainer from "./db_nav/NavContainer.vue";

// const greetMsg = ref("");
// const name = ref("");


// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

const currentHeight: Ref<number> = ref(window.innerHeight);

function resizeHeight() {
  currentHeight.value = window.innerHeight;
}

onMounted(() => {
  window.addEventListener('resize', resizeHeight);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', resizeHeight);
});

</script>

<script lang="ts">
  export default {
    name: "HomePage",
  }
</script>

<template>
  <main>
    <v-layout class="rounded rounded-md border" :style="{height: currentHeight + 'px'}">
      <!-- <v-navigation-drawer :rail="rail" @click="toggleRail"> -->
      <NavContainer>
        <v-list nav>
          <v-list-item title="Navigation drawer" link></v-list-item>
        </v-list>
      </NavContainer>
      <!-- </v-navigation-drawer> -->

      <v-app-bar title="Application bar"></v-app-bar>

      <v-main class="d-flex align-center justify-center" height="300">
        <v-container>
          sdfsadfae
          <router-link to="/new_connection">new_connection</router-link>
          <v-sheet
            border="dashed md"
            color="surface-light"
            height="200"
            rounded="lg"
            width="100%"
          ></v-sheet>
        </v-container>
      </v-main>
    </v-layout>
  </main>
</template>

<style scoped>

</style>