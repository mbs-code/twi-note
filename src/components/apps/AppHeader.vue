<template>
  <n-space>
    <div>side</div>
    <n-switch v-model:value="configStore.expand_side" />
    <div>dark</div>
    <n-switch v-model:value="configStore.is_dark" />
    <router-link to="/">
      <n-button size="small">
        ホーム
      </n-button>
    </router-link>
    <router-link to="/tag">
      <n-button size="small">
        タグ
      </n-button>
    </router-link>
  </n-space>
</template>

<script setup lang="ts">
import { useConfigStore } from '../../stores/config'

const configStore = useConfigStore()
</script>
