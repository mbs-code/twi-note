<template>
  <div class="d-flex">
    <n-button
      size="small"
      text
      @click="onExpandButton()"
    >
      <template #icon>
        <n-icon v-if="configStore.expand_side">
          <RightIcon />
        </n-icon>
        <n-icon v-else>
          <LeftIcon />
        </n-icon>
      </template>
    </n-button>

    <router-link to="/" class="no-link flex-shrink-0">
      <n-button size="small" text>
        <template #icon>
          <n-icon>
            <ReportIcon />
          </n-icon>
        </template>
        メモ
      </n-button>
    </router-link>

    <router-link to="/tag" class="no-link flex-shrink-0">
      <n-button size="small" text>
        <template #icon>
          <n-icon>
            <TagIcon />
          </n-icon>
        </template>
        タグ
      </n-button>
    </router-link>

    <div name="spacer" class="flex-grow-1" />

    <n-button
      size="small"
      text
      @click="openDrawer"
    >
      <template #icon>
        <n-icon>
          <DrawerIcon />
        </n-icon>
      </template>
    </n-button>
  </div>

  <ConfigDrawer v-model:show="showDrawer" />
</template>

<script setup lang="ts">
import { useConfigStore } from '../../stores/config'
import {
  ChevronForward as LeftIcon,
  ChevronBack as RightIcon,
  DocumentText as ReportIcon,
  Pricetag as TagIcon,
  SettingsSharp as DrawerIcon,
} from '@vicons/ionicons5'
import { ref } from 'vue'

const configStore = useConfigStore()

const onExpandButton = () => {
  console.log('click')
  configStore.expand_side = !configStore.expand_side
}

const showDrawer = ref(false)
const openDrawer = () => {
  showDrawer.value = true
}
</script>

<style scoped lang="scss">
.d-flex {
  display: flex;

  > * {
    margin: 0 12px;
  }
}

.flex-grow-1 {
  flex-grow: 1;
}
</style>
