<template>
  <n-config-provider :theme="isDark ? darkTheme : lightTheme">
    <n-dialog-provider>
      <n-layout position="absolute">
        <n-layout-header position="absolute" bordered style="height: 22px;">
          <n-space>
            <div>div</div>
            <n-switch v-model:value="isDark" />
          </n-space>
        </n-layout-header>

        <n-layout has-sider position="absolute" style="top: 22px;">
          <n-layout content-style="padding: 8px;" :native-scrollbar="false">
            <router-view />
            <n-back-top bottom="20" right="20" />
          </n-layout>
        </n-layout>
      </n-layout>
    </n-dialog-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { lightTheme, darkTheme } from 'naive-ui'

const isDark = ref<boolean>(true)
</script>
