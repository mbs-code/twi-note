<template>
  <n-drawer v-model:show="showDrawer" :width="500">
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

      <n-form-item label="Storage">
        <n-space vertical style="width: 100%">
          <n-input :value="storage?.path" readonly>
            <template #prefix>
              <n-icon :component="PathIcon" />
            </template>
          </n-input>

          <span>{{ filesize(storage?.size) }}</span>
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
} from '@vicons/ionicons5'
import { useConfigStore } from '../../stores/config'
import { StorageInfo, useStorageAPI } from '../../composables/useStorageAPI'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ (e: 'update:show', value: boolean): void }>()
const configStore = useConfigStore()

const showDrawer = computed({
  get: () => props.show,
  set: (val: boolean) => emit('update:show', val),
})

///

const storageAPI = useStorageAPI()
const storage = ref<StorageInfo>()
onMounted(async () => {
  storage.value = await storageAPI.load()
})
</script>
