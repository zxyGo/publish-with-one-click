<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { dbService } from './utils/db'

const greetMsg = ref('')
const name = ref('')
const dbUsers = ref<any[]>([])

onMounted(async () => {
  // 初始化数据库
  await dbService.init()
  // 加载数据
  await loadUsers()
})

async function loadUsers() {
  try {
    const db = dbService.getDb()
    dbUsers.value = await db.select('SELECT * FROM users ORDER BY id DESC')
  } catch (e) {
    console.error('加载用户失败:', e)
  }
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke('greet', { name: name.value })

  // 同时保存到数据库演示
  if (name.value) {
    try {
      const db = dbService.getDb()
      await db.execute('INSERT INTO users (name) VALUES ($1)', [name.value])
      await loadUsers()
    } catch (e) {
      console.error('保存用户失败:', e)
    }
  }
}
</script>

<template>
  <main class="container">
    <h1 class="text-3xl font-bold text-blue-600">Welcome to Tauri + Vue</h1>
    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <t-button theme="primary" type="submit">Greet & Save</t-button>
    </form>
    <p>{{ greetMsg }}</p>

    <!-- 数据库演示区域 -->
    <div class="mt-8 text-left max-w-md mx-auto">
      <h2 class="text-xl font-bold mb-4">Local Database Users:</h2>
      <ul class="list-disc pl-5">
        <li v-for="user in dbUsers" :key="user.id">
          {{ user.name }} <span class="text-gray-500 text-sm">({{ user.created_at }})</span>
        </li>
      </ul>
      <p v-if="dbUsers.length === 0" class="text-gray-500">No users found in database.</p>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
