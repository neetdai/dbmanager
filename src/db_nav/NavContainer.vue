<script setup lang="ts">
import { Ref, ref } from 'vue';

const currentWidth: Ref<number> = ref(100);
const rail: Ref<boolean> = ref(false);

const toggleRail = () => {
  rail.value = !rail.value;
}

</script>
<script lang="ts">
  export default {
    name: 'NavContainer',
  }
</script>
<template>
  <v-layout class="rounded rounded-md border">
    <v-navigation-drawer class="border border-e-md" :style="{maxWidth: currentWidth + 'px', overflow: 'scroll'}" :rail="rail" @click="toggleRail" permanent >
      <slot></slot>
    </v-navigation-drawer>
  </v-layout>
</template>

<style scoped>

</style>