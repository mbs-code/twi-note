<template>
  <div class="d-flex" style="margin: 0 12px">
    <n-button
      size="small"
      text
      @click="onExpandButton"
    >
      <template #icon>
        <n-icon v-if="configStore.expand_side" :component="RightIcon" />
        <n-icon v-else :component="LeftIcon" />
      </template>
    </n-button>

    <template v-for="(link, _) of links" :key="_">
      <router-link :to="link.to" class="no-link">
        <n-button size="small" text>
          <template #icon>
            <n-icon :component="link.icon" />
          </template>
          {{ link.name }}
        </n-button>
      </router-link>
    </template>

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
  SettingsSharp as DrawerIcon,
  DocumentText as ReportIcon,
  Pricetag as TagIcon,
} from '@vicons/ionicons5'
import { ref } from 'vue'

const configStore = useConfigStore()

const links = [
  { name: 'ホーム', icon: ReportIcon, to: { name: 'home' } },
  { name: 'タイムライン', icon: ReportIcon, to: { name: 'timeline' } },
  { name: 'タグ', icon: TagIcon, to: { name: 'tag' } },
]

const onExpandButton = () => {
  configStore.expand_side = !configStore.expand_side
}

const showDrawer = ref(false)
const openDrawer = () => {
  showDrawer.value = true
}
</script>
