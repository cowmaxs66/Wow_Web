<script setup lang="ts">
import { Activity, History, LineChart } from "@lucide/vue";
import { computed } from "vue";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatRelativeAge, formatTimestamp } from "../types/protocol";

const props = defineProps<{
  history: ClientStatusEnvelope[];
  limit: number;
}>();

interface TrendPoint {
  x: number;
  y: number;
}

const latest = computed(() => props.history[props.history.length - 1] ?? null);
const first = computed(() => props.history[0] ?? null);

const onlineSamples = computed(() => {
  return props.history.filter((item) => item.data.online).length;
});

const scriptSwitchCount = computed(() => {
  let changes = 0;
  let previous: string | null = null;

  for (const item of props.history) {
    const script = item.data.current_script ?? "無腳本";
    if (previous !== null && previous !== script) {
      changes += 1;
    }
    previous = script;
  }

  return changes;
});

const trendPoints = computed<TrendPoint[]>(() => {
  if (props.history.length === 0) {
    return [];
  }

  if (props.history.length === 1) {
    return [{ x: 50, y: props.history[0].data.online ? 20 : 80 }];
  }

  const maxIndex = props.history.length - 1;
  return props.history.map((item, index) => ({
    x: Math.round((index / maxIndex) * 100),
    y: item.data.online ? 20 : 80,
  }));
});

const polylinePoints = computed(() => {
  return trendPoints.value.map((point) => `${point.x},${point.y}`).join(" ");
});

const recentRows = computed(() => {
  return [...props.history].reverse().slice(0, 6);
});
</script>

<template>
  <section class="history-panel">
    <header>
      <div>
        <h2>歷史趨勢</h2>
        <p>基於 Server 進程內保存的真實上報記錄。</p>
      </div>
      <LineChart :size="20" />
    </header>

    <div v-if="history.length === 0" class="empty-state">
      <strong>尚無歷史記錄</strong>
      <span>Client 上報後，這裡會顯示最近 {{ limit || 50 }} 條狀態樣本。</span>
    </div>

    <div v-else class="history-content">
      <div class="summary-grid">
        <div>
          <History :size="16" />
          <span>樣本數</span>
          <strong>{{ history.length }}/{{ limit }}</strong>
        </div>
        <div>
          <Activity :size="16" />
          <span>在線樣本</span>
          <strong>{{ onlineSamples }}/{{ history.length }}</strong>
        </div>
        <div>
          <span>腳本切換</span>
          <strong>{{ scriptSwitchCount }}</strong>
        </div>
        <div>
          <span>最近樣本</span>
          <strong>{{ latest ? formatRelativeAge(latest.timestamp_ms) : "無資料" }}</strong>
        </div>
      </div>

      <div class="trend-box" aria-label="在線狀態趨勢">
        <svg viewBox="0 0 100 100" preserveAspectRatio="none" role="img">
          <line x1="0" y1="20" x2="100" y2="20" />
          <line x1="0" y1="80" x2="100" y2="80" />
          <polyline v-if="trendPoints.length > 1" :points="polylinePoints" />
          <circle
            v-for="point in trendPoints"
            :key="`${point.x}-${point.y}`"
            :cx="point.x"
            :cy="point.y"
            r="2.2"
          />
        </svg>
        <div class="trend-labels">
          <span>{{ first ? formatTimestamp(first.timestamp_ms) : "起點" }}</span>
          <span>{{ latest ? formatTimestamp(latest.timestamp_ms) : "終點" }}</span>
        </div>
      </div>

      <div class="history-list">
        <h3>最近記錄</h3>
        <div v-for="item in recentRows" :key="item.message_id" class="history-row">
          <span>{{ formatTimestamp(item.timestamp_ms) }}</span>
          <strong>{{ item.data.online ? "在線" : "離線" }}</strong>
          <small>{{ item.data.current_script ?? "無腳本" }}</small>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.history-panel {
  display: grid;
  gap: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  box-shadow: var(--shadow-panel);
  padding: var(--space-5);
}

header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  color: var(--color-accent);
}

h2,
h3,
p {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 17px;
}

h3 {
  color: var(--color-text);
  font-size: 13px;
}

p {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 13px;
}

.history-content {
  display: grid;
  gap: var(--space-4);
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: var(--space-3);
}

.summary-grid div {
  display: grid;
  gap: var(--space-1);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-page);
  padding: var(--space-3);
  color: var(--color-accent);
}

.summary-grid span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 700;
}

.summary-grid strong {
  overflow: hidden;
  color: var(--color-text);
  font-size: 14px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.trend-box {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: linear-gradient(180deg, #ffffff 0%, var(--color-page) 100%);
  padding: var(--space-3);
}

.trend-box svg {
  display: block;
  width: 100%;
  height: 120px;
}

.trend-box line {
  stroke: var(--color-border);
  stroke-width: 1;
  vector-effect: non-scaling-stroke;
}

.trend-box polyline {
  fill: none;
  stroke: var(--color-teal);
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-width: 3;
  vector-effect: non-scaling-stroke;
}

.trend-box circle {
  fill: var(--color-teal);
  stroke: #ffffff;
  stroke-width: 1.2;
}

.trend-labels,
.history-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
}

.trend-labels span,
.history-row span,
.history-row small {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 700;
}

.history-list {
  display: grid;
  gap: var(--space-2);
}

.history-row {
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-2);
}

.history-row strong {
  color: var(--color-text);
  font-size: 13px;
}

.empty-state {
  display: grid;
  gap: var(--space-1);
  min-height: 130px;
  align-content: center;
  color: var(--color-muted);
  font-size: 13px;
  line-height: 1.6;
}

.empty-state strong {
  color: var(--color-text);
  font-size: 14px;
}

@media (max-width: 820px) {
  .summary-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 520px) {
  .summary-grid {
    grid-template-columns: 1fr;
  }

  .trend-box svg {
    height: 96px;
  }

  .trend-box circle {
    display: none;
  }

  .trend-labels {
    align-items: flex-start;
    flex-direction: column;
    gap: var(--space-1);
  }
}
</style>
