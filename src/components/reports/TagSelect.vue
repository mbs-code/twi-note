<template>
  <n-select
    placeholder="Tags"
    filterable
    multiple
    clearable
    tag
    :options="options"
    :render-tag="renderTag"
    :render-label="renderLabel"
  />
</template>

<script setup lang="ts">
import { computed, h, onMounted, ref, VNodeChild } from 'vue'
import { Tag, useTagAPI } from '../../composables/useTagAPI'
import { NTag, SelectOption, SelectRenderTag } from 'naive-ui'
import TagPanel from './TagPanel.vue'

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

const renderTag: SelectRenderTag = ({ option, handleClose }) => {
  const findTag = tags.value.find((tag) => option.label === tag.name)

  return h(NTag,
    {},
    {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore // TODO: バグ？
      default: () => h(TagPanel, {
        tag: findTag,
        text: option.label,
        closable: true,
        onClose: (e: MouseEvent) => {
          e.stopPropagation()
          handleClose()
        },
      }, {})
    },
  )
}

const renderLabel = (option: SelectOption): VNodeChild => {
  const findTag = tags.value.find((tag) => option.label === tag.name)

  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore // TODO: バグ？
  return h(TagPanel, {
    tag: findTag,
    text: option.label,
  }, {})
}
</script>
