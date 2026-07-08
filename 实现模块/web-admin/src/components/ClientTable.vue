<script setup lang="ts">
import { computed } from "vue";
import { MonitorCheck, Search } from "@lucide/vue";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatRelativeAge, formatTimestamp } from "../types/protocol";
import StatusDot from "./StatusDot.vue";

const props = defineProps<{
  clients: ClientStatusEnvelope[];
  selectedClientId: string;
  loading: boolean;
  page: number;
  pageSize: number;
  total: number;
  totalPages: number;
  searchText: string;
  groupFilter: string;
  tagFilter: string;
  onlineFilter: ClientFilter;
}>();

defineEmits<{
  select: [clientId: string];
  "update:searchText": [value: string];
  "update:groupFilter": [value: string];
  "update:tagFilter": [value: string];
  "update:onlineFilter": [value: ClientFilter];
  "update:pageSize": [value: number];
  applyFilters: [];
  pageChange: [page: number];
}>();

type ClientFilter = "all" | "online" | "offline";

const filterOptions: Array<{ value: ClientFilter; label: string }> = [
  { value: "all", label: "全部" },
  { value: "online", label: "在線" },
  { value: "offline", label: "離線" },
];

const onlineCount = computed(() =>
  props.clients.filter((client) => client.data.online).length,
);

const dmEnabledCount = computed(() =>
  props.clients.filter((client) =>
    client.data.script.allowed_permissions.includes("dm.access"),
  ).length,
);

const scriptCount = computed(
  () => props.clients.filter((client) => !!client.data.current_script).length,
);

const groupCount = computed(() => {
  const groups = new Set(
    props.clients.map((client) => client.data.identity.group || "default"),
  );
  return groups.size;
});

function runtimeMode(client: ClientStatusEnvelope): string {
  const arch = client.data.runtime.arch || "unknown";
  const hasDm = client.data.script.allowed_permissions.includes("dm.access");
  return hasDm ? `${arch} / DM` : `${arch} / Core`;
}

function tagText(client: ClientStatusEnvelope): string {
  return client.data.identity.tags.length > 0
    ? client.data.identity.tags.join(", ")
    : "無標籤";
}

const pageSummary = computed(() => {
  if (props.total === 0) {
    return "0 / 0";
  }

  return `${props.page} / ${props.totalPages || 1}`;
});
</script>

<template>
  <section class="client-table">
    <header>
      <div>
        <h2>客戶端列表</h2>
        <p>先選 Client，再查看腳本、DM 權限與遠程操作。</p>
      </div>
    </header>

    <div class="list-toolbar">
      <div class="summary-strip" aria-label="客戶端摘要">
        <span>總數 <strong>{{ total }}</strong></span>
        <span>在線 <strong>{{ onlineCount }}</strong></span>
        <span>分組 <strong>{{ groupCount }}</strong></span>
        <span>DM <strong>{{ dmEnabledCount }}</strong></span>
        <span>腳本 <strong>{{ scriptCount }}</strong></span>
        <span>頁碼 <strong>{{ pageSummary }}</strong></span>
      </div>
      <label class="search-box">
        <Search :size="15" />
        <input
          :value="searchText"
          placeholder="搜尋 Client / 分組 / 標籤 / 腳本"
          @input="$emit('update:searchText', ($event.target as HTMLInputElement).value)"
        />
      </label>
    </div>

    <div class="server-filter-row" aria-label="Server 端篩選">
      <label>
        <span>分組</span>
        <input
          :value="groupFilter"
          placeholder="default / raid-a"
          @input="$emit('update:groupFilter', ($event.target as HTMLInputElement).value)"
        />
      </label>
      <label>
        <span>標籤</span>
        <input
          :value="tagFilter"
          placeholder="dm / farm"
          @input="$emit('update:tagFilter', ($event.target as HTMLInputElement).value)"
        />
      </label>
      <label>
        <span>每頁</span>
        <select
          :value="pageSize"
          @change="$emit('update:pageSize', Number(($event.target as HTMLSelectElement).value)); $emit('applyFilters')"
        >
          <option :value="10">10</option>
          <option :value="25">25</option>
          <option :value="50">50</option>
          <option :value="100">100</option>
        </select>
      </label>
      <button type="button" :disabled="loading" @click="$emit('applyFilters')">
        套用篩選
      </button>
    </div>

    <div class="filter-row" aria-label="客戶端篩選">
      <button
        v-for="option in filterOptions"
        :key="option.value"
        type="button"
        :class="{ active: onlineFilter === option.value }"
        @click="$emit('update:onlineFilter', option.value); $emit('applyFilters')"
      >
        {{ option.label }}
      </button>
    </div>

    <div v-if="clients.length === 0" class="empty-state">
      <MonitorCheck :size="34" :stroke-width="1.8" />
      <strong>{{ loading ? "正在讀取狀態" : "尚無 Client 上報" }}</strong>
      <span>啟動 Client Agent 並開啟 Server 上報後，這裡會顯示最新狀態。</span>
    </div>

    <div v-else-if="clients.length === 0" class="empty-state">
      <MonitorCheck :size="34" :stroke-width="1.8" />
      <strong>沒有符合條件的 Client</strong>
      <span>清除搜尋字或切換篩選條件後再查看。</span>
    </div>

    <div v-else class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Client</th>
            <th>分組 / 標籤</th>
            <th>狀態</th>
            <th>模式</th>
            <th>腳本</th>
            <th>版本</th>
            <th>最近上報</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="client in clients"
            :key="client.client_id"
            :class="{ selected: client.client_id === selectedClientId }"
            @click="$emit('select', client.client_id)"
          >
            <td data-label="Client">
              <button type="button">
                <strong>{{ client.data.identity.display_name || client.client_id }}</strong>
                <small>{{ client.client_id }}</small>
              </button>
            </td>
            <td data-label="分組 / 標籤">
              <span class="group-cell">
                <strong>{{ client.data.identity.group || "default" }}</strong>
                <small>{{ tagText(client) }}</small>
              </span>
            </td>
            <td data-label="狀態">
              <StatusDot
                :tone="client.data.online ? 'online' : 'offline'"
                :label="client.data.online ? '在線' : '離線'"
              />
            </td>
            <td data-label="模式">{{ runtimeMode(client) }}</td>
            <td data-label="腳本">{{ client.data.current_script ?? "無" }}</td>
            <td data-label="版本">{{ client.data.runtime.release_version }}</td>
            <td data-label="最近上報">
              <span class="time-cell">
                <strong>{{ formatRelativeAge(client.timestamp_ms) }}</strong>
                <small>{{ formatTimestamp(client.timestamp_ms) }}</small>
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="total > 0" class="pager-row">
      <button
        type="button"
        :disabled="loading || page <= 1"
        @click="$emit('pageChange', page - 1)"
      >
        上一頁
      </button>
      <span>{{ pageSummary }}，共 {{ total }} 台</span>
      <button
        type="button"
        :disabled="loading || page >= totalPages"
        @click="$emit('pageChange', page + 1)"
      >
        下一頁
      </button>
    </div>
  </section>
</template>

<style scoped>
.client-table {
  min-width: 0;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  box-shadow: var(--shadow-panel);
}

header {
  border-bottom: 1px solid var(--color-border);
  padding: var(--space-5);
}

.list-toolbar {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(260px, 340px);
  align-items: center;
  gap: var(--space-3);
  border-bottom: 1px solid var(--color-border);
  padding: var(--space-3) var(--space-5);
}

.summary-strip {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.summary-strip span {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-page);
  color: var(--color-muted);
  padding: 6px var(--space-2);
  font-size: 12px;
  font-weight: 760;
}

.summary-strip strong {
  color: var(--color-text);
}

.search-box {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-muted);
  padding: 0 var(--space-3);
}

.search-box input {
  min-width: 0;
  width: 100%;
  border: 0;
  background: transparent;
  color: var(--color-text);
  padding: 9px 0;
  font-size: 13px;
  outline: none;
}

.filter-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  border-bottom: 1px solid var(--color-border);
  padding: var(--space-3) var(--space-5);
}

.server-filter-row {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr)) auto;
  align-items: end;
  gap: var(--space-3);
  border-bottom: 1px solid var(--color-border);
  padding: var(--space-3) var(--space-5);
}

.server-filter-row label {
  display: grid;
  gap: var(--space-2);
}

.server-filter-row span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

.server-filter-row input,
.server-filter-row select {
  width: 100%;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 8px var(--space-3);
  font: inherit;
  font-size: 13px;
  outline: none;
}

.filter-row button {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-muted);
  padding: 7px var(--space-3);
  font-size: 12px;
  font-weight: 780;
}

.server-filter-row button,
.pager-row button {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 8px var(--space-3);
  font-size: 12px;
  font-weight: 780;
}

.server-filter-row button:hover:not(:disabled),
.pager-row button:hover:not(:disabled) {
  border-color: var(--color-accent);
  background: var(--color-accent-soft);
  color: var(--color-accent);
}

.server-filter-row button:disabled,
.pager-row button:disabled {
  opacity: 0.55;
}

.filter-row button.active,
.filter-row button:hover {
  border-color: var(--color-accent);
  background: var(--color-accent-soft);
  color: var(--color-accent);
}

h2,
p {
  margin: 0;
}

h2 {
  font-size: 17px;
}

p {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 13px;
}

.table-wrap {
  overflow-x: auto;
}

table {
  width: 100%;
  min-width: 980px;
  border-collapse: collapse;
}

th,
td {
  border-bottom: 1px solid var(--color-border);
  padding: 13px var(--space-5);
  text-align: left;
  font-size: 13px;
  white-space: nowrap;
}

th {
  color: var(--color-muted);
  font-size: 11px;
  font-weight: 800;
  letter-spacing: 0;
  text-transform: uppercase;
}

tbody tr {
  transition:
    background 140ms ease,
    color 140ms ease;
}

tbody tr:hover,
tbody tr.selected {
  background: var(--color-accent-soft);
}

td {
  color: var(--color-text);
}

td button {
  display: grid;
  gap: 2px;
  border: 0;
  background: transparent;
  color: var(--color-accent);
  padding: 0;
  font-size: 13px;
  font-weight: 780;
}

td button strong,
td button small,
.group-cell strong,
.group-cell small {
  display: block;
}

td button small,
.group-cell small {
  color: var(--color-muted);
  font-size: 11px;
  font-weight: 700;
}

.group-cell {
  display: grid;
  gap: 2px;
}

.group-cell strong {
  color: var(--color-text);
  font-size: 13px;
}

.time-cell {
  display: grid;
  gap: 2px;
}

.time-cell strong,
.time-cell small {
  display: block;
}

.time-cell strong {
  color: var(--color-text);
  font-size: 13px;
}

.time-cell small {
  color: var(--color-muted);
  font-size: 11px;
}

.empty-state {
  display: grid;
  min-height: 260px;
  place-items: center;
  align-content: center;
  gap: var(--space-2);
  padding: var(--space-6);
  color: var(--color-muted);
  text-align: center;
}

.empty-state strong {
  color: var(--color-text);
  font-size: 15px;
}

.empty-state span {
  max-width: 360px;
  font-size: 13px;
  line-height: 1.6;
}

.pager-row {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--space-3);
  border-top: 1px solid var(--color-border);
  padding: var(--space-3) var(--space-5);
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

@media (max-width: 720px) {
  .list-toolbar {
    grid-template-columns: 1fr;
    padding: var(--space-3);
  }

  .filter-row {
    padding: var(--space-3);
  }

  .server-filter-row {
    grid-template-columns: 1fr;
    padding: var(--space-3);
  }

  .pager-row {
    justify-content: stretch;
    padding: var(--space-3);
  }

  .table-wrap {
    overflow: visible;
    padding: var(--space-3);
  }

  table,
  tbody,
  tr,
  td {
    display: block;
    width: 100%;
    min-width: 0;
  }

  table {
    border-collapse: separate;
    border-spacing: 0;
  }

  thead {
    display: none;
  }

  tbody tr {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-control);
    background: #ffffff;
    padding: var(--space-2);
  }

  tbody tr + tr {
    margin-top: var(--space-3);
  }

  td {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
    border-bottom: 0;
    padding: 9px var(--space-2);
    white-space: normal;
  }

  td::before {
    content: attr(data-label);
    color: var(--color-muted);
    font-size: 12px;
    font-weight: 780;
  }

  td button {
    max-width: 210px;
    overflow-wrap: anywhere;
    text-align: right;
  }
}
</style>
