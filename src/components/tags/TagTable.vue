<template>
  <n-data-table
    :columns="columns"
    :pagination="false"
    :data="tags"
    :style="{ height: 'calc(100vh - 48px)' }"
    flex-height
  />
</template>

<script setup lang="ts">
import { DataTableColumns, NButton, NIcon } from 'naive-ui'
import { h, reactive } from 'vue'
import { Tag } from '../../composables/useTagAPI'
import {
  CreateOutline as EditIcon,
} from '@vicons/ionicons5'

defineProps<{ tags: Tag[] }>()
const emit = defineEmits<{
  (e: 'onEdit', tags: Tag): void,
}>()

const columns = reactive<DataTableColumns<Tag>>([
  {
    title: 'ID',
    key: 'id',
    width: 80,
    align: 'center',
    sorter: 'default',
  },
  {
    title: '並び順',
    key: 'priority',
    width: 100,
    align: 'center',
    sorter: 'default',
    defaultSortOrder: 'descend', // 初期値降順
  },
  {
    title: 'タグ名',
    key: 'name',
    sorter: 'default',
  },
  {
    title: '色',
    key: 'color',
    align: 'center',
    sorter: 'default',
    render: (row: Tag) => {
      return h(
        'div',
        {},
        {
          default: () => [
            row.color ? h(
              'span',
              { style: `color: ${row.color}; padding-right: 4px; user-select: 'none';`},
              { default: () => '●'},
            ) : '-',
            row.color,
          ],
        },
      )
    }
  },
  {
    title: 'ピン留め',
    key: 'is_pinned',
    align: 'center',
    sorter: 'default',
    render: (row: Tag) => {
      return h(
        'span',
        {},
        { default: () => row.is_pinned ? '○' : '-' },
      )
    },
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
    title: '',
    key: 'actions',
    width: 60,
    render: (row: Tag) =>  h(
      NButton,
      {
        quaternary: true,
        circle: true,
        type: 'primary',
        onClick: () => emit('onEdit', row)
      },
      {
        icon: () => h(
          NIcon,
          {},
          { default: h(EditIcon) },
        ),
      },
    )
  },
])
</script>
