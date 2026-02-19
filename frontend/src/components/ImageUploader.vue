<template>
  <FileUpload
    accept="image/*"
    multiple
    customUpload
    :auto="true"
    :showUploadButton="false"
    :showCancelButton="false"
    :chooseLabel="t('general.select_or_drop_images')"
    chooseIcon="pi pi-image"
    @uploader="onUpload"
    v-tooltip.top="t('general.select_or_drop_images_hint')"
  >
    <template #content="{ files, removeFileCallback }">
      <div v-if="files.length > 0 || uploadedFiles.length > 0" class="image-preview-list">
        <div v-for="(file, index) in files" :key="file.name + file.lastModified" class="image-preview-item">
          <img :src="getObjectUrl(file)" :alt="file.name" class="preview-thumbnail" />
          <ProgressBar :value="progress[file.name]" :showValue="false" class="preview-progress" />
          <Button icon="pi pi-times" severity="danger" size="small" text rounded @click="removeFileCallback(index)" />
        </div>
        <div v-for="(uploaded, index) in uploadedFiles" :key="index" class="image-preview-item uploaded" :class="{ 'fade-out': uploaded.fading }">
          <img :src="uploaded.url" :alt="uploaded.name" class="preview-thumbnail" />
          <i class="pi pi-check" />
        </div>
      </div>
    </template>
  </FileUpload>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import FileUpload, { type FileUploadUploaderEvent } from 'primevue/fileupload'
import ProgressBar from 'primevue/progressbar'
import Button from 'primevue/button'
import api from '@/services/api/api'
import type { UploadImage200Response } from '@/services/api-schema'

const { t } = useI18n()

const emit = defineEmits<{
  uploaded: [url: string]
}>()

const progress = ref<Record<string, number>>({})
const uploadedFiles = ref<{ name: string; url: string; fading: boolean }[]>([])
const objectUrls = new Map<string, string>()

const getObjectUrl = (file: File) => {
  const key = file.name + file.lastModified
  if (!objectUrls.has(key)) {
    objectUrls.set(key, URL.createObjectURL(file))
  }
  return objectUrls.get(key)!
}

const onUpload = (event: FileUploadUploaderEvent) => {
  const files = Array.isArray(event.files) ? event.files : [event.files]
  for (const file of files) {
    const formData = new FormData()
    formData.append('image', file)
    progress.value[file.name] = 0
    api
      .post<UploadImage200Response>('/api/image-host/upload', formData, {
        headers: { 'Content-Type': 'multipart/form-data' },
        onUploadProgress: (e) => {
          if (e.total) {
            progress.value[file.name] = Math.round((e.loaded * 100) / e.total)
          }
        },
      })
      .then((response) => {
        const entry = { name: file.name, url: response.data.data.url, fading: false }
        uploadedFiles.value.push(entry)
        emit('uploaded', response.data.data.url)
        entry.fading = true
        setTimeout(() => {
          const idx = uploadedFiles.value.indexOf(entry)
          if (idx !== -1) uploadedFiles.value.splice(idx, 1)
        }, 500)
      })
      .finally(() => {
        delete progress.value[file.name]
        const key = file.name + file.lastModified
        const objUrl = objectUrls.get(key)
        if (objUrl) {
          URL.revokeObjectURL(objUrl)
          objectUrls.delete(key)
        }
      })
  }
}
</script>

<style scoped>
.empty-label {
  color: var(--p-text-muted-color);
  font-size: 0.85rem;
}

.image-preview-list {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.image-preview-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  width: 80px;
}

.preview-thumbnail {
  width: 80px;
  height: 80px;
  object-fit: cover;
  border-radius: 6px;
}

.preview-progress {
  width: 100%;
  height: 6px;
}

.image-preview-item.uploaded .pi-check {
  color: var(--p-green-500);
}

.image-preview-item.fade-out {
  opacity: 0;
  transition: opacity 0.5s ease;
}
</style>
