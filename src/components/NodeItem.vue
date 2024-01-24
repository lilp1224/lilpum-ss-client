<script setup lang="ts">
import { SsNode } from '../types/SsNode';

const props = defineProps({
  node: {
    type: Object as () => SsNode,
    required: true
  },
  isUsed: Boolean
});

const emit = defineEmits(['selectNode']);

function useNode() {
  emit('selectNode', props.node);
}
</script>

<template>
  <div class="node-item bg-gray-800 rounded-lg p-4 my-2 shadow-md">
    <span class="text-white text-lg">{{ node.server }}:{{ node.server_port }}</span>
    <button
        @click="useNode"
        :disabled="isUsed"
        :class="isUsed ? 'bg-gray-500' : 'bg-blue-500 hover:bg-blue-700'"
        class="ml-4 px-4 py-2 rounded text-white focus:outline-none focus:ring-2 focus:ring-blue-300">
      {{ isUsed ? '使用中' : '使用' }}
    </button>
  </div>
</template>


<style scoped>
.node-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #ddd;
}
</style>
