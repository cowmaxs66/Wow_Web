<script setup lang="ts">
import { RefreshCw, Settings } from "@lucide/vue";

defineProps<{
  serverUrl: string;
  clientId: string;
  selectedClientId: string;
  loading: boolean;
}>();

defineEmits<{
  "update:serverUrl": [value: string];
  "update:clientId": [value: string];
  refresh: [];
}>();
</script>

<template>
  <section class="settings-panel">
    <header>
      <Settings :size="18" />
      <h2>本地設定</h2>
    </header>

    <label>
      <span>Server 地址</span>
      <input
        :value="serverUrl"
        autocomplete="off"
        spellcheck="false"
        @input="$emit('update:serverUrl', ($event.target as HTMLInputElement).value)"
      />
    </label>

    <label>
      <span>查詢 Client ID</span>
      <input
        :value="clientId"
        autocomplete="off"
        spellcheck="false"
        @input="$emit('update:clientId', ($event.target as HTMLInputElement).value)"
      />
    </label>

    <dl>
      <div>
        <dt>目前選中</dt>
        <dd>{{ selectedClientId || "無資料" }}</dd>
      </div>
      <div>
        <dt>設定範圍</dt>
        <dd>瀏覽器本地</dd>
      </div>
    </dl>

    <button type="button" :disabled="loading" @click="$emit('refresh')">
      <RefreshCw :size="16" :class="{ spinning: loading }" />
      <span>{{ loading ? "刷新中" : "刷新狀態" }}</span>
    </button>
  </section>
</template>

<style scoped>
.settings-panel {
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
  line-height: 1.3;
}

label {
  display: grid;
  gap: var(--space-2);
}

label span,
dt {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 750;
}

input {
  width: 100%;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 10px var(--space-3);
  font-size: 14px;
  outline: none;
}

input:focus {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 3px rgba(33, 95, 154, 0.12);
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

dd {
  overflow-wrap: anywhere;
  margin: 0;
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.5;
}

button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  border: 0;
  border-radius: var(--radius-control);
  background: var(--color-accent);
  color: #ffffff;
  padding: 11px var(--space-4);
  font-size: 14px;
  font-weight: 760;
}

button:disabled {
  opacity: 0.68;
}

.spinning {
  animation: spin 0.9s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
