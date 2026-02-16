<template>
  <div id="torrent-stats-filters">
    <FloatLabel>
      <Select
        v-model="timeRange"
        :options="timeRangeOptions"
        optionLabel="label"
        optionValue="value"
        size="small"
        input-id="timeRangeSelect"
        style="min-width: 10em"
      />
      <label for="timeRangeSelect">{{ t('stats.time_range') }}</label>
    </FloatLabel>
    <FloatLabel>
      <Select
        v-model="interval"
        :options="intervalOptions"
        optionLabel="label"
        optionValue="value"
        size="small"
        input-id="intervalSelect"
        style="min-width: 10em"
      />
      <label for="intervalSelect">{{ t('stats.interval') }}</label>
    </FloatLabel>
    <FloatLabel>
      <MultiSelect
        v-model="selectedGroupBys"
        :options="groupByOptions"
        optionLabel="label"
        optionValue="value"
        size="small"
        input-id="groupBySelect"
        style="min-width: 15em"
        :maxSelectedLabels="2"
      />
      <label for="groupBySelect">{{ t('stats.group_by') }}</label>
    </FloatLabel>
  </div>
  <ProgressSpinner v-if="loading" />
  <div v-else-if="overallTorrentStats">
    <div id="torrent-stats-summary">
      <ContentContainer :containerTitle="t('stats.unique_uploaders')">
        {{ overallTorrentStats.unique_uploaders }}
      </ContentContainer>
      <ContentContainer :containerTitle="t('stats.total_size')">
        {{ bytesToReadable(totalSize) }}
      </ContentContainer>
      <ContentContainer :containerTitle="t('stats.total_torrents')">
        {{ totalCount }}
      </ContentContainer>
    </div>
    <h3>{{ t('stats.overall_uploads') }}</h3>
    <Chart class="chart" type="line" :data="chartData" :options="chartOptions" />
    <div v-for="groupBy in selectedGroupBys" :key="groupBy" class="grouped-chart">
      <h3>{{ groupByLabel(groupBy) }}</h3>
      <ProgressSpinner v-if="!groupedStats[groupBy]" />
      <template v-else>
        <div class="grouped-legend">
          <span v-for="(attr, i) in groupedData[groupBy].attributes" :key="attr" class="legend-item">
            <span class="legend-color" :style="{ backgroundColor: CHART_COLORS[i % CHART_COLORS.length] }" />
            {{ attr }}
          </span>
        </div>
        <div class="grouped-charts-row">
          <Chart class="chart grouped-line-chart" type="line" :data="groupedData[groupBy].line" :options="groupedLineOptions" />
          <Chart class="chart grouped-pie-chart" type="pie" :data="groupedData[groupBy].pie" :options="groupedPieOptions" :plugins="[pieOuterLabelsPlugin]" />
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import ContentContainer from '@/components/ContentContainer.vue'
import Chart from 'primevue/chart'
import MultiSelect from 'primevue/multiselect'
import ProgressSpinner from 'primevue/progressspinner'
import Select from 'primevue/select'
import FloatLabel from 'primevue/floatlabel'
import { useI18n } from 'vue-i18n'
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { getTorrentStats, StatsInterval, TorrentStatsGroupBy, type TorrentStatsResponse } from '@/services/api-schema'
import { bytesToReadable, formatDateToLocalString, formatDateTimeLabel } from '@/services/helpers'
import { pieOuterLabelsPlugin } from '@/services/pieOuterLabelsPlugin'

const { t } = useI18n()

type TimeRange = 'this_week' | 'this_month' | 'this_year' | 'all_time'

const timeRange = ref<TimeRange>('this_year')
const interval = ref<StatsInterval>(StatsInterval.Month)
const selectedGroupBys = ref<TorrentStatsGroupBy[]>([TorrentStatsGroupBy.ContentType, TorrentStatsGroupBy.Source])

const timeRangeOptions = [
  { label: t('stats.this_week'), value: 'this_week' },
  { label: t('stats.this_month'), value: 'this_month' },
  { label: t('stats.this_year'), value: 'this_year' },
  { label: t('stats.all_time'), value: 'all_time' },
]

const dateRangeFromSelection = computed(() => {
  const now = new Date()
  const to = now
  let from: Date
  switch (timeRange.value) {
    case 'this_week':
      from = new Date(now.getFullYear(), now.getMonth(), now.getDate() - 7)
      break
    case 'this_month':
      from = new Date(now.getFullYear(), now.getMonth(), now.getDate() - 30)
      break
    case 'this_year':
      from = new Date(now.getFullYear() - 1, now.getMonth(), now.getDate())
      break
    case 'all_time':
      // arcadia wasn't possibly used before this date :)
      from = new Date(2025, 0, 1)
      break
  }
  return { from, to }
})

const intervalOptions = [
  { label: t('stats.hour'), value: StatsInterval.Hour },
  { label: t('stats.day'), value: StatsInterval.Day },
  { label: t('stats.week'), value: StatsInterval.Week },
  { label: t('stats.month'), value: StatsInterval.Month },
  { label: t('stats.year'), value: StatsInterval.Year },
]

const groupByOptions = [
  { label: t('stats.group_by_content_type'), value: TorrentStatsGroupBy.ContentType },
  { label: t('stats.group_by_source'), value: TorrentStatsGroupBy.Source },
  { label: t('stats.group_by_video_resolution'), value: TorrentStatsGroupBy.VideoResolution },
  { label: t('stats.group_by_video_codec'), value: TorrentStatsGroupBy.VideoCodec },
  { label: t('stats.group_by_audio_codec'), value: TorrentStatsGroupBy.AudioCodec },
  { label: t('stats.group_by_audio_channels'), value: TorrentStatsGroupBy.AudioChannels },
  { label: t('stats.group_by_audio_bitrate_sampling'), value: TorrentStatsGroupBy.AudioBitrateSampling },
  { label: t('stats.group_by_container'), value: TorrentStatsGroupBy.Container },
  { label: t('stats.group_by_category'), value: TorrentStatsGroupBy.Category },
  { label: t('stats.group_by_platform'), value: TorrentStatsGroupBy.Platform },
  { label: t('stats.group_by_original_language'), value: TorrentStatsGroupBy.OriginalLanguage },
  { label: t('stats.group_by_country_from'), value: TorrentStatsGroupBy.CountryFrom },
]

const groupByLabelMap: Record<string, string> = Object.fromEntries(groupByOptions.map((o) => [o.value, o.label]))

const groupByLabel = (groupBy: TorrentStatsGroupBy) => groupByLabelMap[groupBy] ?? groupBy

const loading = ref(false)
const overallTorrentStats = ref<TorrentStatsResponse>()
const groupedStats = reactive<Record<string, TorrentStatsResponse>>({})

const totalSize = computed(() => overallTorrentStats.value?.data.reduce((sum, d) => sum + d.total_size, 0) ?? 0)
const totalCount = computed(() => overallTorrentStats.value?.data.reduce((sum, d) => sum + d.count, 0) ?? 0)

const CHART_COLORS = [
  '#3B82F6',
  '#EF4444',
  '#10B981',
  '#F59E0B',
  '#8B5CF6',
  '#EC4899',
  '#06B6D4',
  '#F97316',
  '#84CC16',
  '#6366F1',
  '#14B8A6',
  '#E11D48',
  '#A855F7',
  '#0EA5E9',
  '#D946EF',
  '#65A30D',
]

const chartData = computed(() => {
  if (!overallTorrentStats.value) return {}
  return {
    labels: overallTorrentStats.value.data.map((d) => formatDateTimeLabel(d.period, interval.value)),
    datasets: [
      {
        label: t('stats.count'),
        data: overallTorrentStats.value.data.map((d) => d.count),
        totalSizes: overallTorrentStats.value.data.map((d) => d.total_size),
        pointRadius: 0,
        pointHoverRadius: 5,
      },
    ],
  }
})

const chartOptions = computed(() => ({
  maintainAspectRatio: false,
  interaction: {
    mode: 'index' as const,
    intersect: false,
  },
  plugins: {
    legend: {
      display: false,
    },
    tooltip: {
      callbacks: {
        afterBody: (items: { dataIndex: number }[]) => {
          if (!items.length) return ''
          const idx = items[0].dataIndex
          const size = chartData.value.datasets?.[0]?.totalSizes?.[idx]
          if (size === undefined) return ''
          return `${t('stats.total_size')}: ${bytesToReadable(size)}`
        },
      },
    },
  },
}))

const groupedData = computed(() => {
  const result: Record<string, { attributes: string[]; line: object; pie: object }> = {}
  for (const groupBy of selectedGroupBys.value) {
    const stats = groupedStats[groupBy]
    if (!stats) continue

    const periods = [...new Set(stats.data.map((d) => d.period))].sort()
    const byAttr = new Map<string, Map<string, { count: number; totalSize: number }>>()
    for (const point of stats.data) {
      const attr = point.attribute_value!
      if (!byAttr.has(attr)) byAttr.set(attr, new Map())
      byAttr.get(attr)!.set(point.period, { count: point.count, totalSize: point.total_size })
    }

    const attributes = [...byAttr.keys()]
    const colors = attributes.map((_, i) => CHART_COLORS[i % CHART_COLORS.length])

    result[groupBy] = {
      attributes,
      line: {
        labels: periods.map((p) => formatDateTimeLabel(p, interval.value)),
        datasets: attributes.map((attr, i) => ({
          label: attr,
          data: periods.map((p) => byAttr.get(attr)?.get(p)?.count ?? 0),
          borderColor: colors[i],
          backgroundColor: colors[i],
          pointRadius: 0,
          pointHoverRadius: 5,
        })),
      },
      pie: {
        labels: attributes,
        datasets: [
          {
            data: attributes.map((attr) => {
              let sum = 0
              for (const v of byAttr.get(attr)!.values()) sum += v.count
              return sum
            }),
            totalSizes: attributes.map((attr) => {
              let sum = 0
              for (const v of byAttr.get(attr)!.values()) sum += v.totalSize
              return sum
            }),
            backgroundColor: colors,
          },
        ],
      },
    }
  }
  return result
})

const groupedLineOptions = {
  maintainAspectRatio: false,
  interaction: { mode: 'index' as const, intersect: false },
  plugins: { legend: { display: false } },
}

const groupedPieOptions = computed(() => ({
  maintainAspectRatio: false,
  layout: { padding: 40 },
  plugins: {
    legend: { display: false },
    tooltip: {
      callbacks: {
        afterLabel: (item: { dataIndex: number; chart: { data: { datasets: { totalSizes: number[] }[] } } }) => {
          const size = item.chart.data.datasets[0]?.totalSizes?.[item.dataIndex]
          if (size === undefined) return ''
          return `${t('stats.total_size')}: ${bytesToReadable(size)}`
        },
      },
    },
  },
}))

const fetchTorrentStats = () => {
  const { from, to } = dateRangeFromSelection.value

  loading.value = true
  getTorrentStats({
    from: formatDateToLocalString(from),
    to: formatDateToLocalString(to),
    interval: interval.value,
    group_by: TorrentStatsGroupBy.None,
  })
    .then((data) => {
      overallTorrentStats.value = data
    })
    .finally(() => {
      loading.value = false
    })
}

const fetchGroupedStats = () => {
  const { from, to } = dateRangeFromSelection.value

  for (const groupBy of selectedGroupBys.value) {
    if (groupedStats[groupBy]) continue
    getTorrentStats({
      from: formatDateToLocalString(from),
      to: formatDateToLocalString(to),
      interval: interval.value,
      group_by: groupBy,
    }).then((data) => {
      groupedStats[groupBy] = data
    })
  }

  for (const key of Object.keys(groupedStats)) {
    if (!selectedGroupBys.value.includes(key as TorrentStatsGroupBy)) {
      delete groupedStats[key]
    }
  }
}

onMounted(() => {
  fetchTorrentStats()
  fetchGroupedStats()
})

watch([timeRange, interval], () => {
  for (const key of Object.keys(groupedStats)) delete groupedStats[key]
  fetchTorrentStats()
  fetchGroupedStats()
})

watch(selectedGroupBys, () => {
  fetchGroupedStats()
})
</script>

<style scoped>
#torrent-stats-filters {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin-bottom: 15px;
}

#torrent-stats-summary {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin-bottom: 25px;
  :deep(.content-body) {
    font-size: 1.2em;
    font-weight: bold;
    text-align: center;
  }
}

.chart {
  height: 30vh;
}

.grouped-chart {
  margin-top: 30px;
}

.grouped-legend {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 8px 16px;
  margin-bottom: 10px;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 0.85em;
}

.legend-color {
  display: inline-block;
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.grouped-charts-row {
  display: flex;
  gap: 15px;
}

.grouped-line-chart {
  flex: 2;
}

.grouped-pie-chart {
  flex: 1;
}
h3 {
  text-align: center;
  margin-bottom: 10px;
  font-weight: bold;
}
</style>
