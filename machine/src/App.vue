<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import greet from "./components/Greet.vue";
import click from "./components/ScrollBar.vue";


// import click from "./components/ScrollBar.vue"
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>
    <p>Eudaimonia Machine</p>
    <greet />
    <click/>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
