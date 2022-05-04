<template>
  <n-drawer v-model:show="showDrawer" :style="{ width: 'calc(100vw)', maxWidth: '320px' }">
    <n-drawer-content title="Config" closable>
      <n-form-item label="Dark Mode">
        <n-space>
          <n-icon size="24">
            <LightIcon />
          </n-icon>
          <n-switch v-model:value="configStore.is_dark" />
          <n-icon size="24">
            <DarkIcon />
          </n-icon>
        </n-space>
      </n-form-item>

      <n-form-item label="Timestamp">
        <n-select v-model:value="configStore.timestamp_mode" :options="timestapOptions" />
      </n-form-item>

      <n-form-item label="Storage">
        <n-space vertical style="width: 100%">
          <n-input :value="storage?.path" readonly>
            <template #prefix>
              <n-button text>
                <n-icon :component="PathIcon" />
              </n-button>
            </template>
          </n-input>

          <n-space align="center" justify="space-between">
            <span>{{ fileSize }}</span>

            <n-button type="info" ghost @click="onOpenDirectory">
              <template #icon>
                <n-icon :component="FolderIcon" />
              </template>
              Open
            </n-button>
          </n-space>
        </n-space>
      </n-form-item>
    </n-drawer-content>
  </n-drawer>
</template>

<script setup lang="ts">
import { onMounted, computed, ref } from 'vue'
import filesize from 'filesize'
import {
  Sunny as LightIcon,
  Moon as DarkIcon,
  Save as PathIcon,
  FolderOpen as FolderIcon,
} from '@vicons/ionicons5'
import { useConfigStore } from '../../stores/config'
import { StorageInfo, useStorageAPI } from '../../composables/useStorageAPI'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ (e: 'update:show', value: boolean): void }>()

const configStore = useConfigStore()
const storageAPI = useStorageAPI()

/// ////////////////////////////////////////////////////////////
/// ドロワー表示管理

const showDrawer = computed({
  get: () => props.show,
  set: (val: boolean) => emit('update:show', val),
})

/// ////////////////////////////////////////////////////////////
/// フォーム管理

const storage = ref<StorageInfo>()
onMounted(async () => {
  storage.value = await storageAPI.load()
})

const timestapOptions = ref([
  { label: '相対時間: 5日前', value: 'relative' },
  { label: '絶対時間: 2022-04-01 10:00:00', value: 'absolute' },
])

const fileSize = computed(() => {
  return filesize(storage.value?.size ?? 0)
})

const onOpenDirectory = async () => {
  await storageAPI.open()
}
</script>
