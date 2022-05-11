<template>
  <n-config-provider
    :theme="configStore.is_dark ? darkTheme : lightTheme"
    :locale="jaJP"
    :date-locale="dateJaJP"
  >
    <n-message-provider placement="top-right">
      <n-dialog-provider>
        <n-layout position="absolute">
          <!-- Fixed Header -->
          <n-layout-header bordered position="absolute">
            <AppHeader />
          </n-layout-header>

          <!-- Contents -->
          <n-layout has-sider position="absolute" style="top: 32px">
            <n-layout-sider
              bordered
              collapse-mode="width"
              :width="configStore.expand_side ? 160 : 50"
              :native-scrollbar="false"
            >
              <SidePanel :expand="configStore.expand_side" />
            </n-layout-sider>

            <n-layout>
              <router-view :key="$route.fullPath" />
            </n-layout>
          </n-layout>
        </n-layout>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { lightTheme, darkTheme, jaJP, dateJaJP } from 'naive-ui'
import { onMounted, provide, watch } from 'vue'
import { reportQueryKey, useReportQuery } from './composables/timelines/useReportQuery'
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

///

// 検索クエリ管理 composable
const reportQuery = useReportQuery()
provide(reportQueryKey, reportQuery)

</script>
