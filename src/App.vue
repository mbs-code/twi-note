<template>
  <n-config-provider :theme="configStore.is_dark ? darkTheme : lightTheme">
    <n-message-provider placement="top-right">
      <n-dialog-provider>
        <n-layout position="absolute">
          <!-- Header Area -->
          <n-layout-header
            position="absolute"
            bordered
            style="height: 28px;"
          >
            <AppHeader />
          </n-layout-header>

          <!-- Contents Area -->
          <n-layout
            has-sider
            position="absolute"
            style="top: 34px;"
          >
            <!-- Left Contents -->
            <n-layout-sider
              :native-scrollbar="false"
              :width="configStore.expand_side ? 160 : 50"
              collapse-mode="width"
              bordered
            >
              <SidePanel :expand="configStore.expand_side" />
            </n-layout-sider>

            <!-- Main Contents -->
            <n-layout
              content-style="padding: 8px;"
              :native-scrollbar="false"
            >
              <router-view :key="$route.fullPath" />
              <n-back-top
                bottom="20"
                right="20"
              />
            </n-layout>
          </n-layout>
        </n-layout>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { lightTheme, darkTheme } from 'naive-ui'
import { onMounted, watch } from 'vue'
import { useAppConfigAPI } from './composables/useAppConfigAPI'
import { useConfigStore } from './stores/config'

const configStore = useConfigStore()

const appConfigAPI = useAppConfigAPI()
onMounted(async () => {
  const config = await appConfigAPI.load()
  configStore.$state = config
})

watch(configStore.$state, async () => {
  await appConfigAPI.save(configStore.$state)
})
</script>
