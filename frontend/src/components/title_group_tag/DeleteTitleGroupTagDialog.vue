<template>
  <div class="delete-tag">
    <div>
      {{ t('title_group.confirm_delete_tag') }}
      <br />
      <br />
      <span class="bold">name:</span> {{ tag.name }}
      <br />
      <span class="bold">synonyms:</span> {{ tag.synonyms.join(', ') }}
      <br />
    </div>

    <FloatLabel>
      <InputText v-model="deletionReason" size="small" fluid />
      <label>{{ t('title_group.deletion_reason') }}</label>
    </FloatLabel>

    <Button :label="t('general.delete')" size="small" :loading="loading" :disabled="!deletionReason.trim()" @click="sendDeletion()" />
  </div>
</template>

<script setup lang="ts">
import { deleteTitleGroupTag, type EditedTitleGroupTag } from '@/services/api-schema'
import Button from 'primevue/button'
import FloatLabel from 'primevue/floatlabel'
import InputText from 'primevue/inputtext'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const loading = ref(false)
const deletionReason = ref('')

const props = defineProps<{
  tag: EditedTitleGroupTag
}>()

const emit = defineEmits<{
  deleted: []
}>()

const sendDeletion = () => {
  loading.value = true
  deleteTitleGroupTag({ id: props.tag.id, deletion_reason: deletionReason.value.trim() }).then(() => {
    loading.value = false
    emit('deleted')
  })
}
</script>

<style scoped>
.delete-tag {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 1rem;
  width: 40em;
}
</style>
