<script setup lang="ts">
import { FileCode2, ShieldCheck } from "@lucide/vue";
import { computed, ref } from "vue";
import type { ClientStatusEnvelope } from "../types/protocol";

const props = defineProps<{
  status: ClientStatusEnvelope | null;
}>();

const showRaw = ref(false);

const hasDmAccess = computed(() => {
  return props.status?.data.script.allowed_permissions.includes("dm.access") ?? false;
});
</script>

<template>
  <section class="script-panel">
    <header>
      <FileCode2 :size="18" />
      <div>
        <h2>腳本配置</h2>
        <p>查看選中 Client 上報的 Lua 宿主與安全設定。</p>
      </div>
    </header>

    <div v-if="!status" class="empty-detail">
      <strong>未選擇 Client</strong>
      <span>刷新並選擇客戶端後，這裡會顯示腳本配置。</span>
    </div>

    <div v-else class="script-stack">
      <div class="summary-row">
        <div>
          <span>當前腳本</span>
          <strong>{{ status.data.current_script ?? "無" }}</strong>
        </div>
        <div>
          <span>Bootstrap</span>
          <strong>{{ status.data.script.bootstrap_name }}</strong>
        </div>
        <div>
          <span>指令上限</span>
          <strong>{{ status.data.script.instruction_limit }}</strong>
        </div>
        <div>
          <span>DM 權限</span>
          <strong>{{ hasDmAccess ? "已允許" : "未允許" }}</strong>
        </div>
      </div>

      <div class="security-row">
        <ShieldCheck :size="18" />
        <div>
          <strong>
            {{ status.data.script.security_enabled ? "安全门已开启" : "安全门未开启" }}
          </strong>
          <span>客户端 Lua 指令会按 allowed_permissions 做最小权限控制。</span>
        </div>
      </div>

      <section class="permission-section">
        <h3>允许权限</h3>
        <div
          v-if="status.data.script.allowed_permissions.length > 0"
          class="permission-list"
        >
          <span
            v-for="permission in status.data.script.allowed_permissions"
            :key="permission"
          >
            {{ permission }}
          </span>
        </div>
        <p v-else>未上报任何允许权限。</p>
      </section>

      <button class="raw-toggle" type="button" @click="showRaw = !showRaw">
        {{ showRaw ? "隱藏原始資料" : "查看原始資料" }}
      </button>
      <pre v-if="showRaw">{{ JSON.stringify(status.data.script, null, 2) }}</pre>
    </div>
  </section>
</template>

<style scoped>
.script-panel {
  display: grid;
  gap: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  padding: var(--space-5);
  box-shadow: var(--shadow-panel);
}

header,
.security-row {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
}

header {
  color: var(--color-accent);
}

h2,
h3,
p {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 16px;
}

header p,
.empty-detail,
.permission-section p {
  color: var(--color-muted);
  font-size: 13px;
  line-height: 1.6;
}

.script-stack {
  display: grid;
  gap: var(--space-4);
}

.summary-row {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: var(--space-3);
}

.summary-row div {
  min-width: 0;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  padding: var(--space-3);
}

.summary-row span,
.security-row span {
  display: block;
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.45;
}

.summary-row strong,
.security-row strong {
  display: block;
  overflow-wrap: anywhere;
  color: var(--color-text);
  font-size: 14px;
  line-height: 1.4;
}

.security-row {
  border: 1px solid rgba(8, 127, 122, 0.25);
  border-radius: var(--radius-control);
  background: var(--color-teal-soft);
  color: var(--color-teal);
  padding: var(--space-3);
}

.permission-section {
  display: grid;
  gap: var(--space-2);
}

h3 {
  color: var(--color-text);
  font-size: 13px;
}

.permission-list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.permission-list span {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 6px var(--space-2);
  font-size: 12px;
  font-weight: 760;
}

.raw-toggle {
  justify-self: start;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 8px var(--space-3);
  font-size: 12px;
  font-weight: 760;
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
}

.empty-detail strong {
  color: var(--color-text);
  font-size: 14px;
}

@media (max-width: 720px) {
  .summary-row {
    grid-template-columns: 1fr;
  }
}
</style>
