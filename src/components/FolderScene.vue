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
      <div v-for="node in nodes" class="folder-node">
        {{ node.name }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { FolderNode } from '../interfaces/FolderNode';

export default defineComponent({
  name: 'FolderScene',
  data() {
    return {
      scale: 1,
      offset: { x: 0, y: 0 },
      dragging: false,
      lastPosition: { x: 0, y: 0 }
    };
  },
  props: {
      nodes: {
      type: Array as () => FolderNode[],
      required: true
    }
  },
  computed: {
    workspaceStyle(): Record<string, string> {
      return {
        transform: `translate(${this.offset.x}px, ${this.offset.y}px) scale(${this.scale})`,
        transformOrigin: '0 0'
      };
    }
  },
  methods: {
    startDrag(e: MouseEvent) {
      this.dragging = true;
      this.lastPosition = { x: e.clientX, y: e.clientY };
    },
    onDrag(e: MouseEvent) {
      if (!this.dragging) return;

      this.offset.x += e.clientX - this.lastPosition.x;
      this.offset.y += e.clientY - this.lastPosition.y;

      this.lastPosition = { x: e.clientX, y: e.clientY };
    },
    stopDrag() {
      this.dragging = false;
    },
    onZoom(e: WheelEvent) {
      const zoomFactor = 0.03;
      this.scale += e.deltaY < 0 ? zoomFactor : -zoomFactor;
      this.scale = Math.max(0.3, Math.min(3, this.scale));
    }
  }
});
</script>

<style scoped>
.viewport {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  cursor: grab;
  background: #f8f8f8;
  display: flex;
  justify-content: center;
  align-items: center;
}

.workspace {
  position: relative;
  width: 5000px;
  height: 5000px;
  display: flex;
  justify-content: center;
  align-items:end;

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
  display: flex;           /* flex to center text inside node */
  align-items: center;
  justify-content: center;
  font-weight: bold;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
  margin: 0 10px;          /* spacing between nodes */
}
</style>