<template>
  <Tabs :value="currentTab" size="small">
    <TabList>
      <Tab v-for="(tab, index) in tabs" :key="tab" :value="index">{{ t(`notification.${tab}`) }}</Tab>
    </TabList>
    <TabPanels v-if="isPageReady">
      <TabPanel :value="0"> <ForumThreadPostsNotifications /> </TabPanel>
      <TabPanel :value="1"> <TitleGroupCommentsNotifications /> </TabPanel>
      <TabPanel :value="2"> <TorrentRequestCommentsNotifications /> </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<script setup lang="ts">
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import ForumThreadPostsNotifications from '@/components/notification/ForumThreadPostsNotifications.vue'
import TitleGroupCommentsNotifications from '@/components/notification/TitleGroupCommentsNotifications.vue'
import TorrentRequestCommentsNotifications from '@/components/notification/TorrentRequestCommentsNotifications.vue'
import { useI18n } from 'vue-i18n'
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { ref } from 'vue'

const { t } = useI18n()
const route = useRoute()

const tabs = ['forum_thread_posts', 'title_group_comments', 'torrent_request_comments']
const isPageReady = ref(false)
const currentTab = ref(0)

onMounted(() => {
  if (route.query.tab) {
    currentTab.value = tabs.indexOf(route.query.tab as string)
  }
  isPageReady.value = true
})
</script>
