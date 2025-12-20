<script setup lang="ts">
import { menuRouter } from '@/routers/modules/staticRouter'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()

const menuItems = menuRouter.map(item => ({
  path: item.path ? `/${item.path}` : `/`,
  name: item.name,
  meta: item.meta
}))

function handleMenuChange(value: string) {
  if (value && value !== route.path) {
    router.push(value)
  }
}
</script>
<template>
  <t-menu :value="route.path" @change="handleMenuChange">
    <template #logo>
      <div>logo</div>
    </template>
    <t-menu-item
      v-for="item in menuItems"
      :key="item.path"
      :value="item.path"
    >
      {{ item.meta.title }}
    </t-menu-item>
  </t-menu>
</template>
