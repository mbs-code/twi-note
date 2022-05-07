<template>
  <n-data-table
    :columns="columns"
    :data="phrases"
    size="small"
    flex-height
    :bordered="false"
    :pagination="false"

    :style="{ height: 'calc(100vh - 34px)' }"
  />
</template>

<script setup lang="ts">
import { DataTableColumns, NButton, NIcon } from 'naive-ui'
import { h, reactive } from 'vue'
import { Phrase } from '../../composables/usePhraseAPI'
import {
  CreateOutline as EditIcon,
} from '@vicons/ionicons5'

defineProps<{ phrases: Phrase[] }>()
const emit = defineEmits<{
  (e: 'onEdit', phrases: Phrase): void,
}>()

const columns = reactive<DataTableColumns<Phrase>>([
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
    title: 'フレーズ',
    key: 'text',
    sorter: 'default',
  },
  {
    title: '色',
    key: 'color',
    align: 'center',
    sorter: 'default',
    render: (row: Phrase) => {
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
    render: (row: Phrase) =>  h(
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
