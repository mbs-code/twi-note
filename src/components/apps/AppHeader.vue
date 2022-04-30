<template>
  <n-space>
    <n-button size="small" text style="font-size: 24px; margin: 3px;" @click="onExpandButton()">
      <n-icon v-if="configStore.expand_side">
        <RightIcon />
      </n-icon>
      <n-icon v-else>
        <LeftIcon />
      </n-icon>
    </n-button>

    <router-link to="/" class="no-link">
      <n-button size="small" text>
        <template #icon>
          <n-icon>
            <ReportIcon />
          </n-icon>
        </template>
        メモ
      </n-button>
    </router-link>

    <router-link to="/tag" class="no-link">
      <n-button size="small" text>
        <template #icon>
          <n-icon>
            <TagIcon />
          </n-icon>
        </template>
        タグ
      </n-button>
    </router-link>

    <div>dark</div>
    <n-switch v-model:value="configStore.is_dark" />
  </n-space>
</template>

<script setup lang="ts">
import { useConfigStore } from '../../stores/config'
import {
  ChevronForward as LeftIcon,
  ChevronBack as RightIcon,
  DocumentText as ReportIcon,
  Pricetag as TagIcon,
} from '@vicons/ionicons5'

const configStore = useConfigStore()

const onExpandButton = () => {
  console.log('click')
  configStore.expand_side = !configStore.expand_side
}
</script>
