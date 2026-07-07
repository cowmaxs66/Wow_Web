<script setup lang="ts">
import { BarChart3, Clock3, Cpu, ShieldCheck, TriangleAlert } from "@lucide/vue";
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

const dmAccessCount = computed(() => {
  return props.clients.filter((client) =>
    client.data.script.allowed_permissions.includes("dm.access"),
  ).length;
});

const reportEnabledCount = computed(() => {
  return props.clients.filter((client) => client.data.server.report_enabled).length;
});

const readinessScore = computed(() => {
  if (totalCount.value === 0) {
    return 0;
  }

  // 健康分只用于管理端快速扫描，不替代真实监控告警。
  // 输入：在线、安全门、上报开关三个当前快照指标。
  // 输出：0-100 的粗略成熟度分数。
  // 边界：没有历史失败率、CPU、内存数据时，不把它解释为生产 SLA。
  const online = props.onlineCount / totalCount.value;
  const secure = securityEnabledCount.value / totalCount.value;
  const reporting = reportEnabledCount.value / totalCount.value;
  return Math.round((online * 0.45 + secure * 0.3 + reporting * 0.25) * 100);
});

const archRows = computed(() => {
  const counts = new Map<string, number>();

  for (const client of props.clients) {
    const arch = client.data.runtime.arch || "unknown";
    counts.set(arch, (counts.get(arch) ?? 0) + 1);
  }

  return [...counts.entries()]
    .map(([name, count]) => ({
      name,
      count,
      percent: totalCount.value === 0 ? 0 : Math.round((count / totalCount.value) * 100),
    }))
    .sort((left, right) => right.count - left.count || left.name.localeCompare(right.name));
});

const groupRows = computed(() => {
  const counts = new Map<string, number>();

  for (const client of props.clients) {
    const group = client.data.identity.group || "default";
    counts.set(group, (counts.get(group) ?? 0) + 1);
  }

  return [...counts.entries()]
    .map(([name, count]) => ({
      name,
      count,
      percent: totalCount.value === 0 ? 0 : Math.round((count / totalCount.value) * 100),
    }))
    .sort((left, right) => right.count - left.count || left.name.localeCompare(right.name));
});

const issueRows = computed(() => {
  const rows: Array<{ label: string; count: number; tone: "warning" | "danger" }> = [
    {
      label: "離線 Client",
      count: props.offlineCount,
      tone: props.offlineCount > 0 ? "danger" : "warning",
    },
    {
      label: "未開安全門",
      count: totalCount.value - securityEnabledCount.value,
      tone: "warning",
    },
    {
      label: "未開上報",
      count: totalCount.value - reportEnabledCount.value,
      tone: "warning",
    },
  ];

  return rows.filter((row) => row.count > 0);
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
          <span>健康分</span>
          <strong>{{ readinessScore }}</strong>
        </div>
        <div>
          <Clock3 :size="16" />
          <span>最新上報</span>
          <strong>{{ latestClient ? formatRelativeAge(latestClient.timestamp_ms) : "無資料" }}</strong>
        </div>
        <div>
          <ShieldCheck :size="16" />
          <span>安全門</span>
          <strong>{{ securityEnabledCount }}/{{ totalCount }}</strong>
        </div>
        <div>
          <Cpu :size="16" />
          <span>DM 權限</span>
          <strong>{{ dmAccessCount }}/{{ totalCount }}</strong>
        </div>
      </div>

      <div class="split-lists">
        <div class="script-list">
          <h3>分組分布</h3>
          <div v-for="row in groupRows" :key="row.name" class="script-row">
            <span>{{ row.name }}</span>
            <strong>{{ row.count }} 台</strong>
            <small>{{ row.percent }}%</small>
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

      <div class="script-list">
        <h3>架構分布</h3>
        <div v-for="row in archRows" :key="row.name" class="script-row">
          <span>{{ row.name }}</span>
          <strong>{{ row.count }} 台</strong>
          <small>{{ row.percent }}%</small>
        </div>
      </div>

      <div v-if="issueRows.length > 0" class="issue-list">
        <h3>
          <TriangleAlert :size="15" />
          需要處理
        </h3>
        <span
          v-for="issue in issueRows"
          :key="issue.label"
          :data-tone="issue.tone"
        >
          {{ issue.label }}：{{ issue.count }}
        </span>
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

.split-lists {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-4);
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

.issue-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: center;
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-3);
}

.issue-list h3 {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  margin-right: var(--space-1);
}

.issue-list span {
  border: 1px solid rgba(161, 92, 7, 0.25);
  border-radius: var(--radius-control);
  background: #fff7ed;
  color: var(--color-warning);
  padding: 6px var(--space-2);
  font-size: 12px;
  font-weight: 780;
}

.issue-list span[data-tone="danger"] {
  border-color: rgba(180, 35, 24, 0.25);
  background: #fff1f0;
  color: var(--color-danger);
}

@media (max-width: 640px) {
  .summary-grid,
  .split-lists {
    grid-template-columns: 1fr;
  }
}
</style>
