<script setup lang="ts">
import { MonitorCheck } from "@lucide/vue";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatTimestamp } from "../types/protocol";
import StatusDot from "./StatusDot.vue";

defineProps<{
  clients: ClientStatusEnvelope[];
  selectedClientId: string;
  loading: boolean;
}>();

defineEmits<{
  select: [clientId: string];
}>();
</script>

<template>
  <section class="client-table">
    <header>
      <div>
        <h2>客戶端列表</h2>
        <p>顯示 Server 目前保存的最新狀態。</p>
      </div>
    </header>

    <div v-if="clients.length === 0" class="empty-state">
      <MonitorCheck :size="34" :stroke-width="1.8" />
      <strong>{{ loading ? "正在讀取狀態" : "尚無 Client 上報" }}</strong>
      <span>啟動 Client Agent 並開啟 Server 上報後，這裡會顯示最新狀態。</span>
    </div>

    <div v-else class="table-wrap">
      <table>
        <thead>
          <tr>
            <th>Client</th>
            <th>狀態</th>
            <th>腳本</th>
            <th>消息</th>
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
                {{ client.client_id }}
              </button>
            </td>
            <td data-label="狀態">
              <StatusDot
                :tone="client.data.online ? 'online' : 'offline'"
                :label="client.data.online ? '在線' : '離線'"
              />
            </td>
            <td data-label="腳本">{{ client.data.current_script ?? "無" }}</td>
            <td data-label="消息">{{ client.message_type }}</td>
            <td data-label="最近上報">{{ formatTimestamp(client.timestamp_ms) }}</td>
          </tr>
        </tbody>
      </table>
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
  min-width: 720px;
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
  border: 0;
  background: transparent;
  color: var(--color-accent);
  padding: 0;
  font-size: 13px;
  font-weight: 780;
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

@media (max-width: 720px) {
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
