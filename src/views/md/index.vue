<script setup lang="ts">
import { computed } from 'vue'

// const isTauri = computed(() => Boolean((window as any).__TAURI__?.core))

const mdUrl = computed(() => {
  if (import.meta.env.DEV) return `http://localhost:1422/`
  return new URL(`md-editor/index.html`, window.location.href).toString()
})

// async function openEditorWindow() {
//   if (!isTauri.value) {
//     window.open(mdUrl.value, `_blank`)
//     return
//   }

//   const { WebviewWindow } = await import('@tauri-apps/api/webviewWindow')
//   const existing = await WebviewWindow.getByLabel(`md`)
//   if (existing) {
//     await existing.setFocus()
//     return
//   }

//   const win = new WebviewWindow(`md`, {
//     title: `Markdown 编辑器`,
//     url: mdUrl.value,
//     width: 1280,
//     height: 800
//   })

//   win.once(`tauri://error`, () => {})
// }
</script>

<template>
  <div class="h-full w-full flex flex-col gap-3">
    <!-- <div class="flex items-center gap-2">
      <t-button theme="primary" @click="openEditorWindow">打开独立编辑器窗口</t-button>
      <div class="text-sm opacity-70">
        <span v-if="isTauri">Tauri 模式：支持原生打开/保存对话框</span>
        <span v-else>Web 模式：将以新标签页打开</span>
      </div>
    </div> -->
    <div class="flex-1 min-h-0 rounded overflow-hidden border border-solid border-gray-200">
      <iframe class="h-full w-full" :src="mdUrl" />
    </div>
  </div>
</template>
