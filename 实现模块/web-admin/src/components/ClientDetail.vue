<script setup lang="ts">
import { FileJson2 } from "@lucide/vue";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatTimestamp } from "../types/protocol";
import StatusDot from "./StatusDot.vue";

defineProps<{
  status: ClientStatusEnvelope | null;
}>();
</script>

<template>
  <section class="detail-panel">
    <header>
      <FileJson2 :size="18" />
      <h2>狀態詳情</h2>
    </header>

    <div v-if="!status" class="empty-detail">
      <strong>未選擇 Client</strong>
      <span>刷新後選擇列表中的 Client 查看協議欄位。</span>
    </div>

    <dl v-else>
      <div>
        <dt>Client ID</dt>
        <dd>{{ status.client_id }}</dd>
      </div>
      <div>
        <dt>在線狀態</dt>
        <dd>
          <StatusDot
            :tone="status.data.online ? 'online' : 'offline'"
            :label="status.data.online ? '在線' : '離線'"
          />
        </dd>
      </div>
      <div>
        <dt>當前腳本</dt>
        <dd>{{ status.data.current_script ?? "無" }}</dd>
      </div>
      <div>
        <dt>Message ID</dt>
        <dd>{{ status.message_id }}</dd>
      </div>
      <div>
        <dt>最近上報</dt>
        <dd>{{ formatTimestamp(status.timestamp_ms) }}</dd>
      </div>
    </dl>

    <pre v-if="status">{{ JSON.stringify(status.data, null, 2) }}</pre>
  </section>
</template>

<style scoped>
.detail-panel {
  display: grid;
  gap: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  padding: var(--space-5);
}

header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--color-accent);
}

h2 {
  margin: 0;
  color: var(--color-text);
  font-size: 16px;
}

dl {
  display: grid;
  gap: var(--space-3);
  margin: 0;
}

dl div {
  display: grid;
  gap: var(--space-1);
}

dt {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

dd {
  overflow-wrap: anywhere;
  margin: 0;
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.5;
}

pre {
  overflow-x: auto;
  margin: 0;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #111827;
  color: #d1fae5;
  padding: var(--space-4);
  font-size: 12px;
  line-height: 1.55;
}

.empty-detail {
  display: grid;
  gap: var(--space-1);
  color: var(--color-muted);
  font-size: 13px;
  line-height: 1.6;
}

.empty-detail strong {
  color: var(--color-text);
  font-size: 14px;
}
</style>
