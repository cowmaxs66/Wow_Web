<script setup lang="ts">
import {
  Bell,
  Download,
  FileText,
  MonitorCheck,
  Power,
  RefreshCw,
  Send,
  Settings,
  ShieldCheck,
  Terminal,
} from "@lucide/vue";
import type { Component } from "vue";
import { computed, ref, watch } from "vue";
import { sendClientCommand, sendClientMessage } from "../api/managementServer";
import type { ClientCommandType, ClientStatusEnvelope } from "../types/protocol";

interface CommandAction {
  value: ClientCommandType;
  label: string;
  note: string;
  icon: Component;
  tone?: "default" | "danger";
}

interface CommandGroup {
  title: string;
  actions: CommandAction[];
}

const props = defineProps<{
  status: ClientStatusEnvelope | null;
  clients: ClientStatusEnvelope[];
  serverUrl: string;
}>();

const allClientsValue = "__all_clients__";
const messageTitle = ref("服务端消息");
const messageBody = ref("");
const sendingMessage = ref(false);
const messageResult = ref("");
const pendingCommand = ref<ClientCommandType | null>(null);
const commandResult = ref("");
const selectedTarget = ref("");

const clientOptions = computed(() =>
  props.clients.map((client) => ({
    id: client.client_id,
    online: client.data.online,
    label: `${client.client_id} / ${client.data.online ? "在线" : "离线"}`,
  })),
);

const targetClientIds = computed(() => {
  if (selectedTarget.value === allClientsValue) {
    return clientOptions.value.map((client) => client.id);
  }

  return selectedTarget.value ? [selectedTarget.value] : [];
});

const hasTarget = computed(() => targetClientIds.value.length > 0);

const targetLabel = computed(() => {
  if (selectedTarget.value === allClientsValue) {
    return `全部客户端（${targetClientIds.value.length} 台）`;
  }

  return selectedTarget.value || "未选择";
});

watch(
  () => props.status?.client_id ?? "",
  (selectedId) => {
    const ids = clientOptions.value.map((client) => client.id);
    if (selectedId && ids.includes(selectedId)) {
      selectedTarget.value = selectedId;
    }
  },
  { immediate: true },
);

watch(
  () => clientOptions.value.map((client) => client.id).join("\n"),
  () => {
    const ids = clientOptions.value.map((client) => client.id);
    if (selectedTarget.value === allClientsValue && ids.length > 1) {
      return;
    }

    if (!ids.includes(selectedTarget.value)) {
      selectedTarget.value = ids[0] ?? "";
    }
  },
  { immediate: true },
);

const commandGroups: CommandGroup[] = [
  {
    title: "更新",
    actions: [
      {
        value: "update.apply",
        label: "安装更新",
        note: "下载最新 Release，并启动自替换安装流程。",
        icon: Download,
        tone: "danger",
      },
      {
        value: "update.check",
        label: "检查更新",
        note: "只检查 GitHub Release，不修改本机文件。",
        icon: RefreshCw,
      },
      {
        value: "update.download",
        label: "下载更新包",
        note: "下载新版 zip 到本机 updates 目录。",
        icon: Download,
      },
    ],
  },
  {
    title: "Windows Service",
    actions: [
      {
        value: "service.status",
        label: "查询状态",
        note: "读取 WoWClientAgent 服务状态。",
        icon: MonitorCheck,
      },
      {
        value: "service.install",
        label: "安装服务",
        note: "把客户端注册为 Windows Service。",
        icon: ShieldCheck,
      },
      {
        value: "service.start",
        label: "启动服务",
        note: "启动后台常驻监控服务。",
        icon: Power,
      },
      {
        value: "service.stop",
        label: "停止服务",
        note: "停止后台服务，托盘不受影响。",
        icon: Power,
        tone: "danger",
      },
    ],
  },
  {
    title: "开机启动",
    actions: [
      {
        value: "startup.status",
        label: "查询开机启动",
        note: "检查当前用户 Run 注册项。",
        icon: Terminal,
      },
      {
        value: "startup.enable",
        label: "启用开机启动",
        note: "写入当前用户开机启动项。",
        icon: Power,
      },
      {
        value: "startup.disable",
        label: "停用开机启动",
        note: "删除当前用户开机启动项。",
        icon: Power,
        tone: "danger",
      },
    ],
  },
  {
    title: "本机窗口",
    actions: [
      {
        value: "settings.open",
        label: "打开设置",
        note: "在客户端机器弹出原生设置窗口。",
        icon: Settings,
      },
      {
        value: "log.open",
        label: "打开日志",
        note: "在客户端机器打开本地日志。",
        icon: FileText,
      },
      {
        value: "tray.open",
        label: "打开托盘",
        note: "启动客户端托盘常驻 UI。",
        icon: Bell,
      },
    ],
  },
];

async function submitMessage(): Promise<void> {
  const targets = targetClientIds.value;
  if (!targets.length || !messageTitle.value.trim() || !messageBody.value.trim()) {
    return;
  }

  sendingMessage.value = true;
  messageResult.value = "";

  try {
    const messages = await Promise.all(
      targets.map((clientId) =>
        sendClientMessage(props.serverUrl, clientId, {
          title: messageTitle.value.trim(),
          body: messageBody.value.trim(),
        }),
      ),
    );
    messageResult.value =
      messages.length === 1
        ? `已写入消息队列：${messages[0].id}`
        : `已写入 ${messages.length} 个客户端消息队列`;
    messageBody.value = "";
  } catch (error) {
    messageResult.value =
      error instanceof Error ? error.message : `发送失败：${String(error)}`;
  } finally {
    sendingMessage.value = false;
  }
}

async function submitCommand(commandType: ClientCommandType): Promise<void> {
  const targets = targetClientIds.value;
  if (!targets.length || pendingCommand.value) {
    return;
  }

  pendingCommand.value = commandType;
  commandResult.value = "";

  try {
    // Server 只负责写入白名单命令队列，Client monitor 轮询到后在本机执行。
    // 输入：Web 中明确选择的 Client ID 列表与命令类型。
    // 输出：对应客户端命令队列记录，便于和客户端日志对照。
    // 边界：当前阶段没有强确认回执，执行结果以客户端本机日志为准。
    const commands = await Promise.all(
      targets.map((clientId) =>
        sendClientCommand(props.serverUrl, clientId, {
          command_type: commandType,
          payload: {},
        }),
      ),
    );
    commandResult.value =
      commands.length === 1
        ? `已写入命令队列：${commands[0].id}`
        : `已写入 ${commands.length} 个客户端命令队列`;
  } catch (error) {
    commandResult.value =
      error instanceof Error ? error.message : `下发失败：${String(error)}`;
  } finally {
    pendingCommand.value = null;
  }
}
</script>

<template>
  <section class="remote-panel">
    <header>
      <Send :size="18" />
      <div>
        <h2>遠程操作</h2>
        <p>目标：{{ targetLabel }}</p>
      </div>
    </header>

    <div v-if="!hasTarget" class="empty-detail">
      <strong>未發現 Client</strong>
      <span>刷新並等待客戶端上報後，這裡會顯示可下發的操作。</span>
    </div>

    <div v-else class="remote-stack">
      <label class="target-select">
        <span>下发目标</span>
        <select v-model="selectedTarget">
          <option
            v-if="clientOptions.length > 1"
            :value="allClientsValue"
          >
            全部已上报客户端
          </option>
          <option
            v-for="client in clientOptions"
            :key="client.id"
            :value="client.id"
          >
            {{ client.label }}
          </option>
        </select>
      </label>

      <form class="message-form" @submit.prevent="submitMessage">
        <h3>Server 消息</h3>
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
          :disabled="sendingMessage || !hasTarget || !messageTitle.trim() || !messageBody.trim()"
        >
          <Send :size="15" />
          <span>{{ sendingMessage ? "发送中" : "发送消息" }}</span>
        </button>
        <p v-if="messageResult">{{ messageResult }}</p>
      </form>

      <div class="command-section">
        <h3>本机命令</h3>
        <div
          v-for="group in commandGroups"
          :key="group.title"
          class="command-group"
        >
          <h4>{{ group.title }}</h4>
          <div class="command-grid">
            <button
              v-for="action in group.actions"
              :key="action.value"
              type="button"
              :data-tone="action.tone ?? 'default'"
              :disabled="!!pendingCommand || !hasTarget"
              @click="submitCommand(action.value)"
            >
              <component :is="action.icon" :size="16" :stroke-width="2" />
              <span>
                <strong>
                  {{ pendingCommand === action.value ? "下发中" : action.label }}
                </strong>
                <small>{{ action.note }}</small>
              </span>
            </button>
          </div>
        </div>
        <p v-if="commandResult" class="command-result">{{ commandResult }}</p>
      </div>
    </div>
  </section>
</template>

<style scoped>
.remote-panel {
  display: grid;
  gap: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  padding: var(--space-5);
  box-shadow: var(--shadow-panel);
}

header {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  color: var(--color-accent);
}

h2,
h3,
h4,
p {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 16px;
}

header p,
.message-form p,
.command-result {
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

.remote-stack,
.message-form,
.command-section,
.command-group {
  display: grid;
  gap: var(--space-3);
}

.target-select {
  display: grid;
  gap: var(--space-2);
}

.target-select span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

.target-select select {
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

.command-group {
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-3);
}

h3 {
  color: var(--color-text);
  font-size: 13px;
}

h4 {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 800;
}

.message-form label {
  display: grid;
  gap: var(--space-2);
}

.message-form label span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

.message-form input,
.message-form textarea {
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

.message-form button,
.command-grid button {
  border-radius: var(--radius-control);
  font-size: 13px;
  font-weight: 760;
}

.message-form button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  border: 0;
  background: var(--color-accent);
  color: #ffffff;
  padding: 9px var(--space-3);
}

.message-form button:disabled,
.command-grid button:disabled {
  opacity: 0.6;
}

.command-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
  gap: var(--space-3);
}

.command-grid button {
  display: grid;
  grid-template-columns: 20px minmax(0, 1fr);
  align-items: flex-start;
  gap: var(--space-2);
  border: 1px solid var(--color-border);
  background: #ffffff;
  color: var(--color-text);
  padding: var(--space-3);
  text-align: left;
}

.command-grid button:hover:not(:disabled) {
  border-color: var(--color-accent);
  background: var(--color-accent-soft);
}

.command-grid button[data-tone="danger"] {
  border-color: rgba(180, 35, 24, 0.28);
}

.command-grid button[data-tone="danger"]:hover:not(:disabled) {
  background: #fff1f0;
}

.command-grid strong,
.command-grid small {
  display: block;
}

.command-grid strong {
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.25;
}

.command-grid small {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.45;
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
