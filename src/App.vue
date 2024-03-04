<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import NodeItem from "./components/NodeItem.vue";
import {SsNode} from "./types/SsNode.ts";

const subscribe_link = ref('');
const local_port = ref(1087);

const nodes = ref<SsNode[]>([]);
const used_node = ref<SsNode | null>(null);

// 获取节点列表
async function get_nodes() {
  nodes.value = await invoke('get_nodes_from_url', {link: subscribe_link.value});
}

// 从文件中获取订阅地址
async function get_cached_url() {
  subscribe_link.value = await invoke('get_subscription_url_from_file');
}

get_cached_url();

function handle_select_node(node: SsNode) {
  node.local_address = '127.0.0.1';
  node.local_port = local_port.value;
  // 启动sslocal
  invoke('start_sslocal', {node: node});
  used_node.value = node;
}
</script>

<template>
  <div class="flex flex-col h-screen mx-auto p-4 bg-gray-800 rounded-lg shadow-md">
    <!-- 固定顶部的设置区域 -->
    <div class="flex-none">
      <!-- 本地端口设置和订阅地址设置并排 -->
      <div class="flex flex-wrap -mx-2 mb-4">
        <!-- 本地端口设置 -->
        <div class="flex-initial px-2" style="flex-basis: 20%;">
          <label for="local-port-input" class="block text-sm font-medium text-gray-300">本地端口</label>
          <input id="local-port-input" v-model.number="local_port" type="number" placeholder="端口"
                 class="mt-1 block w-full rounded-md border-gray-600 bg-gray-700 text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"/>
        </div>
        <!-- 输入订阅地址 -->
        <div class="flex-auto px-2" style="flex-basis: 75%;">
          <label for="subscribe-link-input" class="block text-sm font-medium text-gray-300">订阅地址</label>
          <div class="mt-1 flex rounded-md shadow-sm">
            <input id="subscribe-link-input" v-model="subscribe_link" placeholder="请输入订阅地址"
                   class="flex-grow block w-full rounded-none rounded-l-md border-gray-600 bg-gray-700 text-white focus:border-indigo-500 focus:ring focus:ring-indigo-500 focus:ring-opacity-50"/>
            <button @click="get_nodes"
                    class="ml-2 inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md text-white bg-gray-600 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 min-w-0 whitespace-nowrap text-xs px-4">
              获取节点
            </button>
          </div>
        </div>
      </div>

      <!-- 本地代理url -->
      <div class="mt-4 px-2 text-center">
        <label class="block text-sm font-medium text-gray-300">
          本地代理状态:
          <span class="text-white">{{ used_node === null ? '关' : '开' }}</span>
        </label>
        <label class="block text-sm font-medium text-gray-300">
          URL: <code class="text-white bg-gray-800 rounded px-2">socks://127.0.0.1:{{ local_port }}</code>
        </label>
      </div>

      <hr class="my-4 border-gray-600"/>

    </div>

    <!-- 滚动的节点列表 -->
    <div class="flex-grow overflow-auto">
      <NodeItem
          v-for="(node, index) in nodes"
          :key="index"
          :node="node"
          :isUsed="used_node === node"
          @selectNode="handle_select_node"
          class="mb-2"
      />
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

/* 隐藏滚动区域的滚动条，但仍然可以滚动 */
.flex-grow::-webkit-scrollbar {
  width: 0; /* 将滚动条宽度设置为0，从而隐藏滚动条 */
  height: 0; /* 如果有水平滚动条，也将其隐藏 */
}
</style>
