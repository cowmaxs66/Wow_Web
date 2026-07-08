<script setup lang="ts">
import {
  Bell,
  CircleCheck,
  CircleX,
  Clock3,
  Download,
  FileText,
  MonitorCheck,
  Power,
  Play,
  RefreshCw,
  Send,
  Settings,
  ShieldCheck,
  Square,
  Terminal,
} from "@lucide/vue";
import type { Component } from "vue";
import { computed, ref, watch } from "vue";
import ScriptDeployPanel from "./ScriptDeployPanel.vue";
import {
  fetchClientCommandReceipts,
  sendClientCommand,
  sendClientMessage,
} from "../api/managementServer";
import type {
  ClientCommandReceipt,
  ClientCommandType,
  ClientStatusEnvelope,
} from "../types/protocol";
import { formatRelativeAge } from "../types/protocol";

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

interface ReceiptSummaryLine {
  kind: "summary" | "result" | "log" | "omitted" | "error";
  label: string;
  text: string;
}

const props = defineProps<{
  status: ClientStatusEnvelope | null;
  clients: ClientStatusEnvelope[];
  serverUrl: string;
}>();

const messageTitle = ref("服务端消息");
const messageBody = ref("");
const sendingMessage = ref(false);
const messageResult = ref("");
const pendingCommand = ref<ClientCommandType | null>(null);
const commandResult = ref("");
const selectedClientIds = ref<string[]>([]);
const receipts = ref<ClientCommandReceipt[]>([]);
const receiptsLoading = ref(false);
const receiptsError = ref("");
const lastCommandFingerprint = ref("");
const lastCommandAt = ref(0);

const clientOptions = computed(() =>
  props.clients.map((client) => ({
    id: client.client_id,
    online: client.data.online,
    displayName: client.data.identity.display_name || client.client_id,
    group: client.data.identity.group || "default",
    tags: client.data.identity.tags,
  })),
);

const clientIdSet = computed(() => new Set(clientOptions.value.map((client) => client.id)));

const targetClientIds = computed(() =>
  selectedClientIds.value.filter((clientId) => clientIdSet.value.has(clientId)),
);

const hasTarget = computed(() => targetClientIds.value.length > 0);

const isBulkTarget = computed(() => targetClientIds.value.length > 1);

const canOperateTarget = computed(() => hasTarget.value);

const selectedOnlineCount = computed(
  () =>
    clientOptions.value.filter(
      (client) => client.online && targetClientIds.value.includes(client.id),
    ).length,
);

const canSendMessage = computed(
  () =>
    canOperateTarget.value &&
    !sendingMessage.value &&
    !!messageTitle.value.trim() &&
    !!messageBody.value.trim(),
);

const receiptClientId = computed(() => {
  if (targetClientIds.value.length !== 1) {
    return "";
  }

  return targetClientIds.value[0];
});

const targetLabel = computed(() => {
  if (!targetClientIds.value.length) {
    return "未选择";
  }

  if (targetClientIds.value.length === 1) {
    const client = clientOptions.value.find((item) => item.id === targetClientIds.value[0]);
    return client ? `${client.displayName}（${client.id}）` : targetClientIds.value[0];
  }

  return `已选择 ${targetClientIds.value.length} 台 Client`;
});

watch(
  () => props.status?.client_id ?? "",
  (selectedId) => {
    const ids = clientOptions.value.map((client) => client.id);
    if (selectedId && ids.includes(selectedId)) {
      selectedClientIds.value = [selectedId];
    }
  },
  { immediate: true },
);

watch(
  () => clientOptions.value.map((client) => client.id).join("\n"),
  () => {
    const ids = clientOptions.value.map((client) => client.id);
    const idSet = new Set(ids);
    selectedClientIds.value = selectedClientIds.value.filter((clientId) => idSet.has(clientId));

    if (!selectedClientIds.value.length && ids.length) {
      const preferredId = props.status?.client_id;
      selectedClientIds.value = preferredId && idSet.has(preferredId) ? [preferredId] : [ids[0]];
    }
  },
  { immediate: true },
);

watch(
  () => [receiptClientId.value, props.serverUrl] as const,
  () => {
    void refreshReceipts();
  },
  { immediate: true },
);

const commandGroups: CommandGroup[] = [
  {
    title: "Lua 脚本",
    actions: [
      {
        value: "script.run_bootstrap",
        label: "重新执行 Lua",
        note: "让 Client 重新执行本机 bootstrap.lua，并按安全门校验。",
        icon: FileText,
      },
      {
        value: "script.start",
        label: "启动 Lua",
        note: "启用当前 Lua，并立即执行一次。",
        icon: Play,
      },
      {
        value: "script.stop",
        label: "停止 Lua",
        note: "停止后 Client 仍在线，并继续接收消息和命令。",
        icon: Square,
        tone: "danger",
      },
      {
        value: "script.status",
        label: "查询 Lua 状态",
        note: "返回当前脚本、路径、权限和安全门状态。",
        icon: Terminal,
      },
    ],
  },
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
        label: "打开日志窗口",
        note: "在客户端机器打开本地日志 UI。",
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

function commandLabel(commandType: ClientCommandType): string {
  if (commandType === "script.deploy_bundle") {
    return "推送脚本包";
  }

  if (commandType === "config.apply") {
    return "套用 Client 设置";
  }

  for (const group of commandGroups) {
    const action = group.actions.find((item) => item.value === commandType);
    if (action) {
      return action.label;
    }
  }

  return commandType;
}

function receiptSummaryLines(receipt: ClientCommandReceipt): ReceiptSummaryLine[] {
  const lines: ReceiptSummaryLine[] = [];
  for (const rawLine of receipt.summary.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line) {
      continue;
    }

    const parsed = parseReceiptLine(line);
    lines.push(parsed ?? {
      kind: receipt.success ? "summary" : "error",
      label: receipt.success ? "执行摘要" : "错误摘要",
      text: line,
    });
  }

  if (!lines.length) {
    lines.push({
      kind: receipt.success ? "summary" : "error",
      label: receipt.success ? "执行摘要" : "错误摘要",
      text: receipt.summary || "无摘要",
    });
  }

  return lines;
}

function parseReceiptLine(line: string): ReceiptSummaryLine | null {
  if (line.startsWith("[script.result]")) {
    return {
      kind: "result",
      label: "脚本输出",
      text: line.replace("[script.result]", "").trim() || "无返回值",
    };
  }

  if (line.startsWith("[script.log]")) {
    return {
      kind: "log",
      label: "脚本日志",
      text: line.replace("[script.log]", "").trim() || "空日志",
    };
  }

  if (line.startsWith("[script.log.omitted]")) {
    return {
      kind: "omitted",
      label: "日志省略",
      text: line.replace("[script.log.omitted]", "").trim(),
    };
  }

  return null;
}

async function refreshReceipts(): Promise<void> {
  const clientId = receiptClientId.value;
  if (!clientId) {
    receipts.value = [];
    receiptsError.value = "";
    receiptsLoading.value = false;
    return;
  }

  receiptsLoading.value = true;
  receiptsError.value = "";

  try {
    // P24 回执读取只针对当前单台 Client，避免批量目标时把不同机器结果混合展示。
    // 输入：当前 Server URL 和选中的 Client ID。
    // 输出：按最近优先排列的命令执行结果。
    // 边界：回执仍是 Server 内存数据，刷新或重启 Server 后可能为空。
    const list = await fetchClientCommandReceipts(props.serverUrl, clientId);
    if (receiptClientId.value === clientId) {
      receipts.value = [...list.items].reverse();
    }
  } catch (error) {
    if (receiptClientId.value === clientId) {
      receipts.value = [];
      receiptsError.value =
        error instanceof Error ? error.message : `读取回执失败：${String(error)}`;
    }
  } finally {
    if (receiptClientId.value === clientId) {
      receiptsLoading.value = false;
    }
  }
}

function isTargetSelected(clientId: string): boolean {
  return selectedClientIds.value.includes(clientId);
}

function setTargetSelected(clientId: string, checked: boolean): void {
  const next = new Set(selectedClientIds.value);
  if (checked) {
    next.add(clientId);
  } else {
    next.delete(clientId);
  }

  selectedClientIds.value = clientOptions.value
    .map((client) => client.id)
    .filter((id) => next.has(id));
}

function selectAllTargets(): void {
  selectedClientIds.value = clientOptions.value.map((client) => client.id);
}

function clearTargets(): void {
  selectedClientIds.value = [];
}

async function submitMessage(): Promise<void> {
  const targets = targetClientIds.value;
  if (!canSendMessage.value || !targets.length) {
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
  if (!canOperateTarget.value || !targets.length || pendingCommand.value) {
    return;
  }

  pendingCommand.value = commandType;
  commandResult.value = "";

  try {
    const payload = {};
    const fingerprint = JSON.stringify({
      targets,
      commandType,
      payload,
    });
    const now = Date.now();
    if (fingerprint === lastCommandFingerprint.value && now - lastCommandAt.value < 15000) {
      commandResult.value = "已拦截重复命令：15 秒内不要对同一批 Client 重复下发同一命令。";
      return;
    }

    // Server 只负责写入白名单命令队列，Client monitor 轮询到后在本机执行。
    // 输入：Web 中明确选择的 Client ID 列表与命令类型。
    // 输出：对应客户端命令队列记录，后续由 Client 上报执行回执。
    // 边界：命令写入不等于同步执行完成，回执会在下一轮 Client monitor 后出现。
    const commands = await Promise.all(
      targets.map((clientId) =>
        sendClientCommand(props.serverUrl, clientId, {
          command_type: commandType,
          payload,
        }),
      ),
    );
    commandResult.value =
      commands.length === 1
        ? `已写入命令队列：${commands[0].id}`
        : `已写入 ${commands.length} 个客户端命令队列`;
    lastCommandFingerprint.value = fingerprint;
    lastCommandAt.value = now;
    if (targets.length === 1) {
      await refreshReceipts();
    }
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
    <header class="remote-header">
      <Send :size="18" />
      <div>
        <h2>遠程操作</h2>
        <p>目标：{{ targetLabel }}</p>
      </div>
      <div class="target-metrics">
        <span>已选 <strong>{{ targetClientIds.length }}</strong></span>
        <span>在线 <strong>{{ selectedOnlineCount }}</strong></span>
        <span>总数 <strong>{{ clientOptions.length }}</strong></span>
      </div>
    </header>

    <div v-if="!hasTarget" class="empty-detail">
      <strong>未發現 Client</strong>
      <span>刷新並等待客戶端上報後，這裡會顯示可下發的操作。</span>
    </div>

    <div v-else class="remote-console">
      <aside class="target-rail">
        <div class="target-list-panel">
          <div class="target-list-heading">
            <span>下发目标</span>
            <div>
              <button type="button" @click="selectAllTargets">全选</button>
              <button type="button" @click="clearTargets">清空</button>
            </div>
          </div>
          <div class="target-list" role="listbox" aria-label="选择客户端">
            <label
              v-for="client in clientOptions"
              :key="client.id"
              :class="{ active: isTargetSelected(client.id) }"
            >
              <input
                type="checkbox"
                :checked="isTargetSelected(client.id)"
                @change="setTargetSelected(client.id, ($event.target as HTMLInputElement).checked)"
              />
              <span>
                <strong>{{ client.displayName }}</strong>
                <small>{{ client.id }} / {{ client.online ? "在线" : "离线" }}</small>
                <small>{{ client.group }} / {{ client.tags.join(", ") || "无标签" }}</small>
              </span>
            </label>
          </div>
        </div>

        <div v-if="isBulkTarget" class="bulk-summary">
          已选择 {{ targetClientIds.length }} 台 Client。命令会写入每台机器队列，执行结果以各自回执为准。
        </div>

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
            :disabled="!canSendMessage"
          >
            <Send :size="15" />
            <span>{{ sendingMessage ? "发送中" : "发送消息" }}</span>
          </button>
          <p v-if="messageResult">{{ messageResult }}</p>
        </form>

        <div v-if="receiptClientId" class="receipt-section">
          <div class="receipt-heading">
            <div>
              <h3>最近执行回执</h3>
              <p>当前 Client：{{ receiptClientId }}</p>
            </div>
            <button
              type="button"
              :disabled="receiptsLoading"
              @click="refreshReceipts"
            >
              <RefreshCw :size="15" />
              <span>{{ receiptsLoading ? "读取中" : "刷新" }}</span>
            </button>
          </div>

          <p v-if="receiptsError" class="receipt-error">{{ receiptsError }}</p>
          <div v-else-if="receiptsLoading" class="receipt-empty">
            正在读取最近执行结果
          </div>
          <div v-else-if="!receipts.length" class="receipt-empty">
            暂无回执。下发命令后，等待 Client monitor 下一轮轮询。
          </div>
          <ul v-else class="receipt-list">
            <li
              v-for="receipt in receipts"
              :key="receipt.id"
              :data-success="receipt.success ? 'true' : 'false'"
            >
              <component
                :is="receipt.success ? CircleCheck : CircleX"
                :size="17"
                :stroke-width="2.2"
              />
              <div>
                <div class="receipt-meta">
                  <strong>{{ commandLabel(receipt.command_type) }}</strong>
                  <span>
                    <Clock3 :size="13" />
                    {{ formatRelativeAge(receipt.timestamp_ms) }}
                  </span>
                </div>
                <div class="receipt-summary">
                  <p
                    v-for="line in receiptSummaryLines(receipt)"
                    :key="`${receipt.id}-${line.kind}-${line.text}`"
                    :data-kind="line.kind"
                  >
                    <strong>{{ line.label }}</strong>
                    <span>{{ line.text }}</span>
                  </p>
                </div>
                <small>{{ receipt.command_id }}</small>
              </div>
            </li>
          </ul>
        </div>
      </aside>

      <div class="action-workspace">
        <ScriptDeployPanel
          :server-url="serverUrl"
          :target-client-ids="targetClientIds"
          :disabled="!!pendingCommand || !canOperateTarget"
        />

        <div class="command-section">
          <h3>客户端白名单命令</h3>
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
                :disabled="!!pendingCommand || !canOperateTarget"
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

.remote-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-2);
  color: var(--color-accent);
}

.remote-header > div:first-of-type {
  min-width: 0;
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

.remote-header p,
.message-form p,
.command-result {
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

.target-metrics {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: var(--space-2);
  margin-left: auto;
}

.target-metrics span {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-page);
  color: var(--color-muted);
  padding: 6px var(--space-2);
  font-size: 12px;
  font-weight: 760;
}

.target-metrics strong {
  color: var(--color-text);
}

.remote-console {
  display: grid;
  grid-template-columns: minmax(280px, 360px) minmax(0, 1fr);
  align-items: start;
  gap: var(--space-4);
}

.target-rail,
.action-workspace,
.message-form,
.command-section,
.command-group,
.receipt-section {
  display: grid;
  gap: var(--space-3);
}

.target-rail {
  position: sticky;
  top: var(--space-4);
  align-self: start;
}

.action-workspace {
  min-width: 0;
}

.target-list-panel {
  display: grid;
  gap: var(--space-2);
}

.target-list-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
}

.target-list-heading > span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

.target-list-heading div {
  display: inline-flex;
  gap: var(--space-2);
}

.target-list-heading button {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 5px var(--space-2);
  font-size: 12px;
  font-weight: 780;
}

.target-list {
  display: grid;
  max-height: 360px;
  overflow: auto;
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
}

.target-list label {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  align-items: flex-start;
  gap: var(--space-2);
  border-bottom: 1px solid var(--color-border);
  padding: var(--space-3);
}

.target-list label:last-child {
  border-bottom: 0;
}

.target-list label.active {
  background: var(--color-accent-soft);
}

.target-list input {
  width: 16px;
  height: 16px;
  margin-top: 2px;
}

.target-list strong,
.target-list small {
  display: block;
}

.target-list strong {
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.3;
}

.target-list small {
  margin-top: 2px;
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.35;
  overflow-wrap: anywhere;
}

.bulk-summary {
  border: 1px solid rgba(161, 92, 7, 0.28);
  border-radius: var(--radius-control);
  background: #fff7ed;
  color: var(--color-warning);
  padding: var(--space-3);
  font-size: 12px;
  font-weight: 780;
  line-height: 1.5;
}

.message-form,
.receipt-section,
.command-section {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: #f8fafc;
  padding: var(--space-3);
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
.command-grid button,
.receipt-heading button {
  border-radius: var(--radius-control);
  font-size: 13px;
  font-weight: 760;
}

.message-form button,
.receipt-heading button {
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
.command-grid button:disabled,
.receipt-heading button:disabled {
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

.receipt-heading {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-3);
}

.receipt-heading p {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 12px;
}

.receipt-heading button {
  border: 1px solid var(--color-border-strong);
  background: #ffffff;
  color: var(--color-text);
  white-space: nowrap;
}

.receipt-list {
  display: grid;
  gap: var(--space-2);
  margin: 0;
  padding: 0;
  list-style: none;
}

.receipt-list li {
  display: grid;
  grid-template-columns: 20px minmax(0, 1fr);
  gap: var(--space-2);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: #ffffff;
  padding: var(--space-3);
}

.receipt-list li[data-success="true"] > svg {
  color: #067647;
}

.receipt-list li[data-success="false"] > svg {
  color: #b42318;
}

.receipt-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
}

.receipt-meta strong {
  color: var(--color-text);
  font-size: 13px;
}

.receipt-meta span {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  color: var(--color-muted);
  font-size: 12px;
  white-space: nowrap;
}

.receipt-summary {
  display: grid;
  gap: var(--space-1);
  margin-top: var(--space-2);
}

.receipt-summary p {
  display: grid;
  gap: 2px;
  margin-top: var(--space-1);
  border-left: 3px solid var(--color-border-strong);
  border-radius: 4px;
  background: #f8fafc;
  color: var(--color-text);
  padding: 6px var(--space-2);
  font-size: 12px;
  line-height: 1.45;
  overflow-wrap: anywhere;
}

.receipt-summary p[data-kind="result"] {
  border-left-color: var(--color-accent);
  background: var(--color-accent-soft);
}

.receipt-summary p[data-kind="log"] {
  border-left-color: #067647;
  background: #ecfdf3;
}

.receipt-summary p[data-kind="error"] {
  border-left-color: #b42318;
  background: #fff1f0;
}

.receipt-summary p[data-kind="omitted"] {
  border-left-color: #a15c07;
  background: #fff7ed;
}

.receipt-summary strong,
.receipt-summary span {
  display: block;
}

.receipt-summary strong {
  color: var(--color-muted);
  font-size: 11px;
}

.receipt-list small,
.receipt-empty,
.receipt-error {
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

.receipt-error {
  color: #b42318;
}

@media (max-width: 1180px) {
  .remote-console {
    grid-template-columns: 1fr;
  }

  .target-rail {
    position: static;
  }
}

@media (max-width: 720px) {
  .remote-header {
    display: grid;
  }

  .target-metrics {
    justify-content: flex-start;
    margin-left: 0;
  }

  .command-grid {
    grid-template-columns: 1fr;
  }
}
</style>
