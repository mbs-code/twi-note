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
    </n-drawer-content>
  </n-drawer>
</template>

<script setup lang="ts">
import { computed } from '@vue/reactivity'
import { useConfigStore } from '../../stores/config'
import {
  Sunny as LightIcon,
  Moon as DarkIcon,
} from '@vicons/ionicons5'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ (e: 'update:show', value: boolean): void }>()
const configStore = useConfigStore()

const showDrawer = computed({
  get: () => props.show,
  set: (val: boolean) => emit('update:show', val),
})
</script>
