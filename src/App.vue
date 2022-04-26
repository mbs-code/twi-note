<template>
  <n-config-provider :theme="isDark ? darkTheme : lightTheme">
    <n-message-provider placement="top-right">
      <n-dialog-provider>
        <n-layout position="absolute">
          <!-- Header Area -->
          <n-layout-header position="absolute" bordered style="height: 22px;">
            <n-space>
              <div>div</div>
              <n-switch v-model:value="isDark" />
              <router-link to="/"><n-button size="small">ホーム</n-button></router-link>
              <router-link to="/tag"><n-button size="small">タグ</n-button></router-link>
            </n-space>
          </n-layout-header>

          <!-- Contents Area -->
          <n-layout has-sider position="absolute" style="top: 22px;">
            <!-- Left Contents -->
            <n-layout-sider
              content-style="padding: 8px;"
              :native-scrollbar="false"
              :width="50"
              bordered
            >
              <SidePanel />
            </n-layout-sider>

            <!-- Main Contents -->
            <n-layout content-style="padding: 8px;" :native-scrollbar="false">
              <router-view :key="$route.fullPath" />
              <n-back-top bottom="20" right="20" />
            </n-layout>
          </n-layout>
        </n-layout>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { lightTheme, darkTheme } from 'naive-ui'

const isDark = ref<boolean>(true)
</script>
