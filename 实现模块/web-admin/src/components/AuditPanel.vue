<script setup lang="ts">
import { ClipboardList, RefreshCw } from "@lucide/vue";
import { onMounted, ref, watch } from "vue";
import { fetchServerAuditEvents } from "../api/managementServer";
import type { ServerAuditEvent } from "../types/protocol";
import { formatRelativeAge } from "../types/protocol";

const props = defineProps<{
  serverUrl: string;
}>();

const events = ref<ServerAuditEvent[]>([]);
const loading = ref(false);
const errorMessage = ref("");

async function refreshAudit(): Promise<void> {
  loading.value = true;
  errorMessage.value = "";

  try {
    // 审计列表来自 Server 最近内存队列，开启 AUDIT_PATH 后也会从 JSONL 恢复。
    // 输入：当前 Server URL。
    // 输出：最近 50 条消息、命令和回执摘要。
    // 边界：当前没有操作者身份，生产联网前仍需补登录和角色权限。
    const list = await fetchServerAuditEvents(props.serverUrl, 50);
    events.value = list.items;
  } catch (error) {
    events.value = [];
    errorMessage.value =
      error instanceof Error ? error.message : `读取审计失败：${String(error)}`;
  } finally {
    loading.value = false;
  }
}

function eventTitle(event: ServerAuditEvent): string {
  if (event.event_type === "message.created") {
    return "Server 消息";
  }
  if (event.event_type === "command.created") {
    return "命令下发";
  }
  if (event.event_type === "command.receipt") {
    return event.success === false ? "执行失败" : "执行回执";
  }

  return event.event_type;
}

watch(
  () => props.serverUrl,
  () => {
    void refreshAudit();
  },
);

onMounted(() => {
  void refreshAudit();
});
</script>

<template>
  <section class="audit-panel">
    <header>
      <ClipboardList :size="18" />
      <div>
        <h2>Server 審計</h2>
        <p>最近消息、命令和回执摘要；启用审计路径后可持久化到 JSONL。</p>
      </div>
      <button type="button" :disabled="loading" @click="refreshAudit">
        <RefreshCw :size="15" :class="{ spinning: loading }" />
        <span>{{ loading ? "读取中" : "刷新" }}</span>
      </button>
    </header>

    <p v-if="errorMessage" class="audit-error">{{ errorMessage }}</p>
    <div v-else-if="loading && events.length === 0" class="audit-empty">
      正在读取审计记录
    </div>
    <div v-else-if="events.length === 0" class="audit-empty">
      暂无审计记录。下发消息、命令或收到回执后会显示在这里。
    </div>
    <ul v-else class="audit-list">
      <li v-for="event in events" :key="event.id">
        <div>
          <strong>{{ eventTitle(event) }}</strong>
          <span>{{ formatRelativeAge(event.timestamp_ms) }}</span>
        </div>
        <p>{{ event.summary }}</p>
        <small>
          {{ event.client_id }}
          <template v-if="event.command_type"> / {{ event.command_type }}</template>
        </small>
      </li>
    </ul>
  </section>
</template>

<style scoped>
.audit-panel {
  display: grid;
  gap: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  padding: var(--space-5);
  box-shadow: var(--shadow-panel);
}

header {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr) auto;
  align-items: start;
  gap: var(--space-2);
  color: var(--color-accent);
}

h2,
p {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 16px;
}

header p,
.audit-empty,
.audit-error {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

header button {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 8px var(--space-3);
  font-size: 12px;
  font-weight: 760;
}

button:disabled {
  opacity: 0.6;
}

.audit-error {
  color: var(--color-danger);
}

.audit-list {
  display: grid;
  gap: var(--space-2);
  margin: 0;
  padding: 0;
  list-style: none;
}

.audit-list li {
  display: grid;
  gap: var(--space-1);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #ffffff;
  padding: var(--space-3);
}

.audit-list div {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
}

.audit-list strong {
  color: var(--color-text);
  font-size: 13px;
}

.audit-list span,
.audit-list small {
  color: var(--color-muted);
  font-size: 12px;
}

.audit-list p {
  overflow-wrap: anywhere;
  color: var(--color-text);
  font-size: 12px;
  line-height: 1.5;
}

.spinning {
  animation: spin 0.9s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@media (max-width: 720px) {
  header {
    grid-template-columns: auto minmax(0, 1fr);
  }

  header button {
    grid-column: 1 / -1;
    justify-content: center;
  }
}
</style>
