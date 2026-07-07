<script setup lang="ts">
import { FileJson2 } from "@lucide/vue";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatFullTimestamp, formatRelativeAge } from "../types/protocol";
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

    <div v-else class="detail-sections">
      <section>
        <h3>基本狀態</h3>
        <dl>
          <div>
            <dt>Client ID</dt>
            <dd>{{ status.client_id }}</dd>
          </div>
          <div>
            <dt>显示名称</dt>
            <dd>{{ status.data.identity.display_name || "未命名 Client" }}</dd>
          </div>
          <div>
            <dt>分组</dt>
            <dd>{{ status.data.identity.group || "default" }}</dd>
          </div>
          <div>
            <dt>标签</dt>
            <dd>{{ status.data.identity.tags.join(", ") || "无" }}</dd>
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
            <dt>最近上報</dt>
            <dd>{{ formatRelativeAge(status.timestamp_ms) }}</dd>
          </div>
          <div>
            <dt>完整時間</dt>
            <dd>{{ formatFullTimestamp(status.timestamp_ms) }}</dd>
          </div>
        </dl>
      </section>

      <section>
        <h3>Agent 运行</h3>
        <dl>
          <div>
            <dt>版本</dt>
            <dd>{{ status.data.runtime.release_version }}</dd>
          </div>
          <div>
            <dt>系统 / 架构</dt>
            <dd>{{ status.data.runtime.os }} / {{ status.data.runtime.arch }}</dd>
          </div>
          <div>
            <dt>Process ID</dt>
            <dd>{{ status.data.runtime.process_id }}</dd>
          </div>
        </dl>
      </section>

      <section>
        <h3>脚本设置</h3>
        <dl>
          <div>
            <dt>Bootstrap</dt>
            <dd>{{ status.data.script.bootstrap_name }}</dd>
          </div>
          <div>
            <dt>指令上限</dt>
            <dd>{{ status.data.script.instruction_limit }}</dd>
          </div>
          <div>
            <dt>安全门</dt>
            <dd>{{ status.data.script.security_enabled ? "已开启" : "未开启" }}</dd>
          </div>
          <div>
            <dt>允许权限</dt>
            <dd>{{ status.data.script.allowed_permissions.join(", ") || "无" }}</dd>
          </div>
        </dl>
      </section>

      <section>
        <h3>Server 上报</h3>
        <dl>
          <div>
            <dt>上报状态</dt>
            <dd>{{ status.data.server.report_enabled ? "已开启" : "未开启" }}</dd>
          </div>
          <div>
            <dt>上报目标</dt>
            <dd>{{ status.data.server.report_target }}</dd>
          </div>
          <div>
            <dt>Message ID</dt>
            <dd>{{ status.message_id }}</dd>
          </div>
        </dl>
      </section>
    </div>

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

.detail-sections {
  display: grid;
  gap: var(--space-4);
}

.detail-sections section {
  display: grid;
  gap: var(--space-3);
}

h3 {
  margin: 0;
  color: var(--color-text);
  font-size: 13px;
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
