<template>
  <div
    class="viewport"
    @mousedown="startDrag"
    @mousemove="onDrag"
    @mouseup="stopDrag"
    @mouseleave="stopDrag"
    @wheel.prevent="onZoom"
  >
    <div
      class="workspace"
      :style="workspaceStyle"
    >
      <div class="folder-node" :style="{ margin: auto }">
        Home
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const scale = ref(1)
const offset = ref({ x: 0, y: 0 })
const dragging = ref(false)
const lastPos = ref({ x: 0, y: 0 })

const workspaceStyle = computed(() => ({
  transform: `
    translate(${offset.value.x}px, ${offset.value.y}px)
    scale(${scale.value})
  `,
  transformOrigin: '0 0'
}))

function startDrag(e) {
  dragging.value = true
  lastPos.value = { x: e.clientX, y: e.clientY }
}

function onDrag(e) {
  if (!dragging.value) return

  offset.value.x += e.clientX - lastPos.value.x
  offset.value.y += e.clientY - lastPos.value.y

  lastPos.value = { x: e.clientX, y: e.clientY }
}

function stopDrag() {
  dragging.value = false
}

function onZoom(e) {
  const zoomFactor = 0.1
  scale.value += e.deltaY < 0 ? zoomFactor : -zoomFactor
  scale.value = Math.max(0.3, Math.min(3, scale.value))
}
</script>

<style scoped>
.viewport {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  cursor: grab;
  background: #f8f8f8;
}

.workspace {
  position: relative;
  width: 5000px;
  height: 5000px;

  background-image:
    linear-gradient(to right, #e0e0e0 1px, transparent 1px),
    linear-gradient(to bottom, #e0e0e0 1px, transparent 1px);

  background-size: 40px 40px;
}

.folder-node {
  width: 120px;
  height: 80px;
  background: white;
  border: 1px solid #ccc;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
}
</style>