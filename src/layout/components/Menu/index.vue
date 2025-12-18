<script setup lang="ts">
import { menuRouter } from '@/routers/modules/staticRouter'
console.log(menuRouter)
const menuItems = menuRouter.map(item => ({
  path: item.path,
  name: item.name,
  meta: item.meta
}))
</script>
<template>
  <t-menu>
    <template #logo>
      <div>logo</div>
    </template>
    <t-menu-item
      v-for="item in menuItems"
      :key="item.path"
      :path="item.path"
      :name="item.name"
      :meta="item.meta"
    >
      {{ item.meta.title }}
    </t-menu-item>
  </t-menu>
</template>
