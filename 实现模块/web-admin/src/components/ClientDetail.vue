<script setup lang="ts">
import { FileJson2, Send } from "@lucide/vue";
import { ref } from "vue";
import { sendClientCommand, sendClientMessage } from "../api/managementServer";
import type { ClientCommandType, ClientStatusEnvelope } from "../types/protocol";
import { formatFullTimestamp, formatRelativeAge } from "../types/protocol";
import StatusDot from "./StatusDot.vue";

const props = defineProps<{
  status: ClientStatusEnvelope | null;
  serverUrl: string;
}>();

const messageTitle = ref("服务端消息");
const messageBody = ref("");
const sendingMessage = ref(false);
const messageResult = ref("");
const selectedCommand = ref<ClientCommandType>("startup.status");
const sendingCommand = ref(false);
const commandResult = ref("");

const commandOptions: Array<{ value: ClientCommandType; label: string }> = [
  { value: "startup.status", label: "查询开机启动" },
  { value: "startup.enable", label: "启用开机启动" },
  { value: "startup.disable", label: "停用开机启动" },
  { value: "service.status", label: "查询 Service" },
  { value: "service.install", label: "安装 Service" },
  { value: "service.start", label: "启动 Service" },
  { value: "service.stop", label: "停止 Service" },
  { value: "update.check", label: "检查更新" },
  { value: "update.download", label: "下载更新包" },
  { value: "settings.open", label: "打开设置窗口" },
  { value: "log.open", label: "打开日志" },
  { value: "tray.open", label: "打开托盘" },
];

async function submitMessage(): Promise<void> {
  if (!props.status || !messageTitle.value.trim() || !messageBody.value.trim()) {
    return;
  }

  sendingMessage.value = true;
  messageResult.value = "";

  try {
    const message = await sendClientMessage(props.serverUrl, props.status.client_id, {
      title: messageTitle.value.trim(),
      body: messageBody.value.trim(),
    });
    messageResult.value = `已写入消息队列：${message.id}`;
    messageBody.value = "";
  } catch (error) {
    messageResult.value =
      error instanceof Error ? error.message : `发送失败：${String(error)}`;
  } finally {
    sendingMessage.value = false;
  }
}

async function submitCommand(): Promise<void> {
  if (!props.status) {
    return;
  }

  sendingCommand.value = true;
  commandResult.value = "";

  try {
    const command = await sendClientCommand(props.serverUrl, props.status.client_id, {
      command_type: selectedCommand.value,
      payload: {},
    });
    commandResult.value = `已写入命令队列：${command.id}`;
  } catch (error) {
    commandResult.value =
      error instanceof Error ? error.message : `下发失败：${String(error)}`;
  } finally {
    sendingCommand.value = false;
  }
}
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

      <section>
        <h3>Server 消息</h3>
        <form class="message-form" @submit.prevent="submitMessage">
          <label>
            <span>标题</span>
            <input v-model="messageTitle" maxlength="80" />
          </label>
          <label>
            <span>内容</span>
            <textarea v-model="messageBody" maxlength="1000" rows="4" />
          </label>
          <button
            type="submit"
            :disabled="sendingMessage || !messageTitle.trim() || !messageBody.trim()"
          >
            <Send :size="15" />
            <span>{{ sendingMessage ? "发送中" : "发送消息" }}</span>
          </button>
          <p v-if="messageResult">{{ messageResult }}</p>
        </form>
      </section>

      <section>
        <h3>远程本机操作</h3>
        <form class="message-form" @submit.prevent="submitCommand">
          <label>
            <span>命令</span>
            <select v-model="selectedCommand">
              <option
                v-for="option in commandOptions"
                :key="option.value"
                :value="option.value"
              >
                {{ option.label }}
              </option>
            </select>
          </label>
          <button type="submit" :disabled="sendingCommand">
            <Send :size="15" />
            <span>{{ sendingCommand ? "下发中" : "下发命令" }}</span>
          </button>
          <p v-if="commandResult">{{ commandResult }}</p>
        </form>
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

.message-form {
  display: grid;
  gap: var(--space-3);
}

.message-form label {
  display: grid;
  gap: var(--space-2);
}

.message-form span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

.message-form input,
.message-form textarea,
.message-form select {
  width: 100%;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 9px var(--space-3);
  font: inherit;
  font-size: 13px;
  outline: none;
}

.message-form textarea {
  resize: vertical;
}

.message-form button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  border: 0;
  border-radius: var(--radius-control);
  background: var(--color-accent);
  color: #ffffff;
  padding: 9px var(--space-3);
  font-size: 13px;
  font-weight: 760;
}

.message-form button:disabled {
  background: var(--color-border-strong);
}

.message-form p {
  margin: 0;
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
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
