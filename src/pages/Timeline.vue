<template>
  <n-layout-header bordered position="absolute">
    <div ref="headerRef" style="padding: 4px">
      <SearchPanel />
    </div>
  </n-layout-header>

  <n-layout
    position="absolute"
    :style="{ top: headerHeight + 'px', bottom: footerHeight + 'px' }"
    :native-scrollbar="false"
  >
    <div v-for="(k, _) of Array.from({ length: 50 })" :key="_">コンテンツ</div>
  </n-layout>

  <n-layout-footer bordered position="absolute">
    <div ref="footerRef">
      <n-card class="card-dense">
        <ReportEditBox />
        <!-- @saved="handleCreated"  -->
      </n-card>
    </div>
  </n-layout-footer>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'

const headerHeight = ref(0)
const footerHeight = ref(0)

const headerRef = ref<HTMLDivElement>()
const footerRef = ref<HTMLDivElement>()
const sizeObserver = ref<ResizeObserver>()

onMounted(() => {
  const observer = new ResizeObserver(() => {
    headerHeight.value = (headerRef.value?.clientHeight ?? 0) + 1
    footerHeight.value = (footerRef.value?.clientHeight ?? 0) + 1
  })

  if (headerRef.value) observer.observe(headerRef.value)
  if (footerRef.value) observer.observe(footerRef.value)
  sizeObserver.value = observer
})

onUnmounted(() => {
  const observer = sizeObserver.value
  if (observer) {
    if (headerRef.value) observer.unobserve(headerRef.value)
    if (footerRef.value) observer.unobserve(footerRef.value)
  }
})
</script>
