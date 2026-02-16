<template>
  <div ref="containerRef" :style="{ minHeight: visible ? undefined : '80px' }">
    <TitleGroupPreviewTable
      v-if="visible"
      :title_group
      :hideSeriesName
      :showDeleteBtn
      :deleteBtnTooltip
      @delete="(titleGroupId) => emit('delete', titleGroupId)"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue'
import TitleGroupPreviewTable from './TitleGroupPreviewTable.vue'
import type { TitleGroupHierarchyLite } from '@/services/api-schema'

defineProps<{
  title_group: TitleGroupHierarchyLite
  hideSeriesName?: boolean
  showDeleteBtn?: boolean
  deleteBtnTooltip?: string
}>()

const emit = defineEmits<{
  delete: [titleGroupId: number]
}>()

const containerRef = ref<HTMLElement>()
const visible = ref(false)
let observer: IntersectionObserver | undefined

onMounted(() => {
  if (!containerRef.value) return
  observer = new IntersectionObserver(
    ([entry]) => {
      if (entry.isIntersecting) {
        visible.value = true
        observer?.disconnect()
      }
    },
    { rootMargin: '200px' },
  )
  observer.observe(containerRef.value)
})

onBeforeUnmount(() => {
  observer?.disconnect()
})
</script>
