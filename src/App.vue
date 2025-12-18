<script setup lang="ts">
// import { ref, onMounted } from 'vue'
// import { invoke } from '@tauri-apps/api/core'
// import { dbService } from './utils/db'

// const greetMsg = ref('')
// const name = ref('')
// const dbUsers = ref<any[]>([])

// onMounted(async () => {
//   // 初始化数据库
//   await dbService.init()
//   // 加载数据
//   await loadUsers()
// })

// async function loadUsers() {
//   try {
//     const db = dbService.getDb()
//     dbUsers.value = await db.select('SELECT * FROM users ORDER BY id DESC')
//   } catch (e) {
//     console.error('加载用户失败:', e)
//   }
// }

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke('greet', { name: name.value })

//   // 同时保存到数据库演示
//   if (name.value) {
//     try {
//       const db = dbService.getDb()
//       await db.execute('INSERT INTO users (name) VALUES ($1)', [name.value])
//       await loadUsers()
//     } catch (e) {
//       console.error('保存用户失败:', e)
//     }
//   }
// }
</script>

<template>
  <router-view></router-view>
</template>
