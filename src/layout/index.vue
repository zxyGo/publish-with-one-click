<script setup lang="ts">
import Menu from './components/Menu/index.vue'
</script>
<template>
  <t-layout class="h-full w-full">
    <t-aside>
      <Menu />
    </t-aside>
    <t-layout class="h-full">
      <t-header>
        <div>发布助手</div>
      </t-header>
      <t-content class="h-full">
        <router-view></router-view>
      </t-content>
    </t-layout>
  </t-layout>
</template>
