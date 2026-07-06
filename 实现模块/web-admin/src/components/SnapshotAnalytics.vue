<script setup lang="ts">
import { BarChart3, Clock3, ShieldCheck } from "@lucide/vue";
import { computed } from "vue";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatRelativeAge } from "../types/protocol";

const props = defineProps<{
  clients: ClientStatusEnvelope[];
  onlineCount: number;
  offlineCount: number;
}>();

const totalCount = computed(() => props.clients.length);

const onlinePercent = computed(() => {
  if (totalCount.value === 0) {
    return 0;
  }

  return Math.round((props.onlineCount / totalCount.value) * 100);
});

const onlineBarStyle = computed(() => ({
  width: `${onlinePercent.value}%`,
}));

const scriptRows = computed(() => {
  const counts = new Map<string, number>();

  for (const client of props.clients) {
    const scriptName = client.data.current_script ?? "無腳本";
    counts.set(scriptName, (counts.get(scriptName) ?? 0) + 1);
  }

  // 快照分析只聚合当前最新状态，不推断历史趋势。
  // 输入：Server 返回的 ClientStatusEnvelope 列表。
  // 输出：按数量和名称稳定排序的脚本分布。
  // 边界：没有 Client 时返回空列表，UI 显示空态。
  return [...counts.entries()]
    .map(([name, count]) => ({
      name,
      count,
      percent: totalCount.value === 0 ? 0 : Math.round((count / totalCount.value) * 100),
    }))
    .sort((left, right) => right.count - left.count || left.name.localeCompare(right.name));
});

const latestClient = computed(() => {
  return [...props.clients].sort((left, right) => right.timestamp_ms - left.timestamp_ms)[0] ?? null;
});

const securityEnabledCount = computed(() => {
  return props.clients.filter((client) => client.data.script.security_enabled).length;
});
</script>

<template>
  <section class="analytics-panel">
    <header>
      <div>
        <h2>快照分析</h2>
        <p>基於 Server 目前保存的最新狀態。</p>
      </div>
      <BarChart3 :size="20" />
    </header>

    <div v-if="totalCount === 0" class="empty-state">
      <strong>尚無可分析資料</strong>
      <span>Client Agent 上報後會顯示在線比例與腳本分布。</span>
    </div>

    <div v-else class="analytics-content">
      <div class="ratio-row">
        <div>
          <span>在線比例</span>
          <strong>{{ onlinePercent }}%</strong>
        </div>
        <small>{{ onlineCount }} 在線 / {{ offlineCount }} 離線</small>
      </div>
      <div class="bar-track" aria-hidden="true">
        <span :style="onlineBarStyle"></span>
      </div>

      <div class="summary-grid">
        <div>
          <ShieldCheck :size="16" />
          <span>安全門</span>
          <strong>{{ securityEnabledCount }}/{{ totalCount }}</strong>
        </div>
        <div>
          <Clock3 :size="16" />
          <span>最新上報</span>
          <strong>{{ latestClient ? formatRelativeAge(latestClient.timestamp_ms) : "無資料" }}</strong>
        </div>
      </div>

      <div class="script-list">
        <h3>腳本分布</h3>
        <div v-for="row in scriptRows" :key="row.name" class="script-row">
          <span>{{ row.name }}</span>
          <strong>{{ row.count }} 台</strong>
          <small>{{ row.percent }}%</small>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.analytics-panel {
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

.analytics-content {
  display: grid;
  gap: var(--space-4);
}

.ratio-row {
  display: flex;
  align-items: end;
  justify-content: space-between;
  gap: var(--space-4);
}

.ratio-row div {
  display: grid;
  gap: var(--space-1);
}

.ratio-row span,
.ratio-row small,
.summary-grid span,
.script-row small {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 700;
}

.ratio-row strong {
  color: var(--color-text);
  font-size: 28px;
  line-height: 1;
}

.bar-track {
  overflow: hidden;
  height: 9px;
  border-radius: 999px;
  background: var(--color-surface-muted);
}

.bar-track span {
  display: block;
  height: 100%;
  border-radius: inherit;
  background: var(--color-teal);
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
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

.summary-grid strong {
  overflow: hidden;
  color: var(--color-text);
  font-size: 14px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.script-list {
  display: grid;
  gap: var(--space-2);
}

.script-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto auto;
  align-items: center;
  gap: var(--space-3);
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-2);
}

.script-row span {
  overflow-wrap: anywhere;
  color: var(--color-text);
  font-size: 13px;
  font-weight: 760;
}

.script-row strong {
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

@media (max-width: 640px) {
  .summary-grid {
    grid-template-columns: 1fr;
  }
}
</style>
