<template>
  <n-card
    class="card-dense"
    embedded
    :bordered="false"
    size="small"
    @keydown.ctrl.enter.exact="onSave"
  >
    <template #action>
      <div class="d-flex flex-align">
        <n-input
          v-model:value="formBody"
          type="textarea"
          placeholder="Text"
          clearable
          :autosize="{ minRows: 1, maxRows: 10 }"
        />

        <div>
          <div class="d-flex flex-column flex-align-stretch" style="height: 100%">
            <div class="flex-space" />

            <div class="d-flex" style="min-height: 36px">
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
                  <n-icon :component="TopIcon" />
                </template>
              </n-button>
            </div>
          </div>
        </div>
      </div>
    </template>
  </n-card>
</template>

<script setup lang="ts">
import { computed } from 'vue'

import {
  FileTrayOutline as CreateIcon,
  ChevronUp as TopIcon,
} from '@vicons/ionicons5'

const props = defineProps<{
  body: string,
  isEdit: boolean,
  showExpand: boolean,
  isValid: boolean,
}>()
const emit = defineEmits<{
  (e: 'update:body', val: string): void,
  (e: 'save'): void,
  (e: 'expand'): void,
}>()

const formBody = computed({
  get: () => props.body,
  set: (val: string) => emit('update:body', val),
})

///

const onSave = () => emit('save')
const onExpandButton = () => emit('expand')
</script>
