<template>
  <div id="app">
    <FolderScene :nodes="nodes" />
  </div>
</template>

<script setup lang="ts">
import FolderScene from './components/FolderScene.vue';
import { invoke } from '@tauri-apps/api/core';
import { homeDir } from '@tauri-apps/api/path';
import { ref, onMounted } from "vue";
import { FolderNode } from "./interfaces/FolderNode"

invoke("start");

const nodes = ref<FolderNode[]>([]);

onMounted(async () => {
  const rootPath = await homeDir();
  nodes.value = await invoke<FolderNode[]>("load_children", { root: rootPath});
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