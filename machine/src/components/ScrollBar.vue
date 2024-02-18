<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const resultMsg = ref("");

async function click() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  resultMsg.value = await invoke("click");
}
</script>

<template>
  <form class="row" @submit.prevent="click">
    <button type="submit">Scroll</button>
  </form>

  <p>{{ resultMsg }}</p>
</template>