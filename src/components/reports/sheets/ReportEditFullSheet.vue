<template>
  <n-card
    class="card-dense"
    embedded
    :bordered="false"
    size="small"
    @keydown.ctrl.enter.exact="onSave"
  >
    <n-space vertical>
      <n-input
        v-model:value="formTitle"
        placeholder="(Title)"
        clearable
      />

      <n-input
        v-model:value="formBody"
        type="textarea"
        placeholder="Text"
        clearable
        :autosize="{ minRows: 3, maxRows: 10 }"
      />

      <n-select
        v-model:value="formTagNames"
        placeholder="Tags"
        filterable
        multiple
        clearable
        tag
        :options="options"
      />
    </n-space>

    <template #action>
      <div class="d-flex flex-align-stretch" style="min-height: 36px">
        <n-button type="default" @click="onReset">
          リセット
        </n-button>

        <div class="flex-grow-1" />

        <n-button
          :type="isEdit ? 'warning' : 'primary'"
          :disabled="!isValid"
          @click="onSave"
        >
          <template #icon>
            <n-icon :component="CreateIcon" />
          </template>
          作成
        </n-button>

        <n-button
          v-if="showExpand"
          text
          @click="onExpandButton"
        >
          <template #icon>
            <n-icon :component="BottomIcon" />
          </template>
        </n-button>
      </div>
    </template>
  </n-card>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { Tag, useTagAPI } from '../../../composables/useTagAPI'
import {
  FileTrayOutline as CreateIcon,
  ChevronDown as BottomIcon,
} from '@vicons/ionicons5'

const props = defineProps<{
  title: string,
  body: string,
  tagNames: string[],
  isEdit: boolean,
  showExpand: boolean,
  isValid: boolean,
}>()
const emit = defineEmits<{
  (e: 'update:title', val: string): void,
  (e: 'update:body', val: string): void,
  (e: 'update:tagNames', val: string[]): void,
  (e: 'save'): void,
  (e: 'reset'): void,
  (e: 'expand'): void,
}>()

const tagAPI = useTagAPI()

/// ////////////////////////////////////////////////////////////
/// タグ管理

const tags = ref<Tag[]>([])
onMounted(async () => {
  tags.value = await tagAPI.getAll({
    hasPinned: false,
  })
})

const options = computed(() => {
  return tags.value
    .map((tag: Tag) => {
      return {
        label: tag.name,
        value: tag.name,
      }
    })
})

/// ////////////////////////////////////////////////////////////

const formTitle = computed({
  get: () => props.title,
  set: (val: string) => emit('update:title', val),
})

const formBody = computed({
  get: () => props.body,
  set: (val: string) => emit('update:body', val),
})

const formTagNames = computed({
  get: () => props.tagNames,
  set: (val: string[]) => emit('update:tagNames', val),
})

///

const onSave = () => emit('save')
const onReset = () => emit('reset')
const onExpandButton = () => emit('expand')
</script>
