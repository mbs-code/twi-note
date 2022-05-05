import { onMounted, onUnmounted, Ref, ref } from 'vue'

export const useHeights = (headerRef: Ref<HTMLDivElement | undefined>, footerRef: Ref<HTMLDivElement | undefined>) => {
  const headerHeight = ref(0)
  const footerHeight = ref(0)
  const backtoHeight = ref(0)

  const sizeObserver = ref<ResizeObserver>()

  onMounted(() => {
    const observer = new ResizeObserver(() => {
      headerHeight.value = (headerRef.value?.clientHeight ?? 0) + 1
      footerHeight.value = (footerRef.value?.clientHeight ?? 0) + 1
      backtoHeight.value = (footerRef.value?.clientHeight ?? 0) + 6
      console.log(footerRef.value?.clientHeight)
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

  return {
    headerHeight,
    footerHeight,
    backtoHeight,
  }
}
