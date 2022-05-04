<template>
  <n-space>
    <n-tag
      v-for="(text, _) of value"
      :key="_"
      closable
      @close="onRemove(_)"
    >
      {{ text }}
    </n-tag>

    <n-auto-complete
      ref="inputRef"
      v-model:value="formText"
      :input-props="{ autocomplete: 'disabled' }"
      :options="options"
      :get-show="() => true"
      type="text"
      size="small"
      placeholder="Tag"
      @keyup.enter.exact="onInput"
    />
  </n-space>
</template>

<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, nextTick, onMounted, ref } from 'vue'
import { Tag, useTagAPI } from '../composables/useTagAPI'

const props = defineProps<{ value: string[] }>()
const emit = defineEmits<{ (e: 'update:value', value: string[]): void }>()

const message = useMessage()
const tagAPI = useTagAPI()

/// ////////////////////////////////////////////////////////////
/// フォーカス機能

const inputRef = ref<HTMLInputElement | null>(null)
const focusInputForm = () => {
  nextTick(() => { inputRef.value?.focus() })
}

/// ////////////////////////////////////////////////////////////
/// オートコンプリート機能

const tags = ref<Tag[]>([])
onMounted(async () => {
  tags.value = await tagAPI.getAll({
    hasPinned: false
  })
})

const options = computed(() => {
  return tags.value
    .map((tag: Tag) => tag.name)
    .filter((name: string) => name ? name.includes(formText.value ?? '') : true)
})

/// ////////////////////////////////////////////////////////////
/// フォームアクション

const formText = ref<string>()
const onInput = () => {
  if (formText.value) {
    // 既に追加されていないなら追加
    if (!props.value.includes(formText.value)) {
      emit('update:value', [...props.value, formText.value])
    } else {
      message.error('既に追加されています。')
    }

    // 入力を空にする
    formText.value = ''
    focusInputForm()
  }
}

const onRemove = (index: number) => {
  const copy = [...props.value]
  copy.splice(index, 1)

  emit('update:value', copy)
  focusInputForm()
}

const onClear = () => {
  // 入力を空にする
  formText.value = ''
}

/// ////////////////////////////////////////////////////////////

defineExpose({ onInput, onClear })
</script>
