<template>
  <div id="app">
    <FolderScene />
  </div>
</template>

<script setup>
import FolderScene from './components/FolderScene.vue';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted } from "vue";

invoke("start");

const fileTree = ref(null);

onMounted(async () => {
  // fileTree.value = await invoke("load_file_tree");
});
</script>

<style>
html, body, #app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>