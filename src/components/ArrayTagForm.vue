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
      v-model:value="formText"
      type="text"
      size="small"
      placeholder="Tag"
      @keyup.enter.exact="onInput"
    />
  </n-space>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'

const props = defineProps<{ value: string[] }>()
const emit = defineEmits<{ (e: 'update:value', value: string[]): void }>()

const formText = ref<string>()
const onInput = () => {
  if (formText.value) {
    emit("update:value", [...props.value, formText.value])
    formText.value = ''
  }
}
const onRemove = (index: number) => {
  const copy = [...props.value]
  copy.splice(index, 1)
  emit("update:value", copy)
}
</script>
