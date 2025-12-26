<template>
  <div>管理</div>
</template>
<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

const list = ref([])

const getList = async () => {
  try {
    list.value = await invoke('get_article_list')
    console.log(list.value)
  } catch (e) {
    console.error('加载失败:', e)
  }
}

onMounted(() => {
  getList()
})
</script>
