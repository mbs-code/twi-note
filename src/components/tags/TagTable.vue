<template>
  <n-data-table
    :columns="columns"
    :pagination="false"
    :data="tags"
    :style="{ height: 'calc(100vh - 40px)' }"
    flex-height
  />
</template>

<script setup lang="ts">
import { DataTableColumns, NButton } from 'naive-ui'
import { h, reactive } from 'vue'
import { Tag } from '../../composables/useTagAPI'

const props = defineProps<{ tags: Tag[] }>()
const emit = defineEmits<{
  (e: 'onEdit', tags: Tag): void,
}>()

const columns = reactive<DataTableColumns<Tag>>([
  {
    title: 'ID',
    key: 'id',
    sorter: 'default',
  },
  {
    title: 'Name',
    key: 'name',
    sorter: 'default',
  },
  {
    title: 'Color',
    key: 'color',
    sorter: 'default',
  },
  {
    title: 'is_pinned',
    key: 'is_pinned',
    sorter: 'default',
  },
  {
    title: 'priority',
    key: 'priority',
    sorter: 'default',
  },
  {
    title: 'created_at',
    key: 'created_at',
    sorter: 'default',
  },
  {
    title: 'updated_at',
    key: 'updated_at',
    sorter: 'default',
  },
  {
    title: 'actions',
    key: 'actions',
    render: (row: Tag) =>  h(
      NButton,
      {
        strong: true,
        tertiary: true,
        size: 'small',
        onClick: () => emit('onEdit', row)
      },
      { default: () => 'Edit' }
    )
  },
])
</script>
