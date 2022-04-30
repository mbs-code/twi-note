<template>
  <n-space style="background-color: blue;">
    <n-button
      size="tiny"
      style="padding: 0px; margin: 0px;"
      @click="onExpandButton()"
    >
      <n-avatar class="no-select" :size="24">
        <n-icon v-if="configStore.expand_side">
          <RightIcon />
        </n-icon>
        <n-icon v-else>
          <LeftIcon />
        </n-icon>
      </n-avatar>
    </n-button>

    <n-button size="small" text>
      <template #icon>
        <n-icon>
          <ReportIcon />
        </n-icon>
      </template>
      メモ
    </n-button>

    <n-button size="small" text>
      <template #icon>
        <n-icon>
          <TagIcon />
        </n-icon>
      </template>
      タグ
    </n-button>

    <div>dark</div>
    <n-switch v-model:value="configStore.is_dark" />

    <router-link to="/">
      <n-button size="small">
        ホーム
      </n-button>
    </router-link>
    <router-link to="/tag">
      <n-button size="small">
        タグ
      </n-button>
    </router-link>
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
