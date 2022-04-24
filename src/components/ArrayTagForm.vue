<template>
  <n-space>
    <n-tag
      v-for="(text, _) of value"
      closable
      @close="onRemove(_)"
    >
     {{ text }}
    </n-tag>

    <n-input
      ref="inputRef"
      v-model:value="formText"
      type="text"
      size="small"
      placeholder="Tag"
      @keyup.enter.exact="onInput"
    />
  </n-space>
</template>

<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { nextTick, ref } from 'vue'

const props = defineProps<{ value: string[] }>()
const emit = defineEmits<{ (e: 'update:value', value: string[]): void }>()
const message = useMessage()

// フォーカス処理
const inputRef = ref<HTMLInputElement | null>(null)
const focusInputForm = () => {
  nextTick(() => { inputRef.value?.focus() })
}

// 更新処理
const formText = ref<string>()
const onInput = () => {
  if (formText.value) {
    // 既に追加されていないなら追加
    if (!props.value.includes(formText.value)) {
      emit("update:value", [...props.value, formText.value])
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

  emit("update:value", copy)
  focusInputForm()
}
</script>
