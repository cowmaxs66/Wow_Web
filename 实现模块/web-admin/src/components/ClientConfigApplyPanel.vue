<script setup lang="ts">
import { ClipboardCheck, Cpu, FileCode2, Send, ServerCog, ShieldCheck } from "@lucide/vue";
import { computed, ref, watch } from "vue";
import { sendClientCommand } from "../api/managementServer";
import type {
  ClientConfigPatch,
  ClientScriptSecurityConfigPatch,
  ClientStatusEnvelope,
} from "../types/protocol";

const props = defineProps<{
  status: ClientStatusEnvelope | null;
  clients: ClientStatusEnvelope[];
  serverUrl: string;
}>();

const emit = defineEmits<{
  refresh: [];
}>();

const selectedClientId = ref("");
const displayName = ref("Local Dev Client");
const clientGroup = ref("default");
const clientTags = ref("local");
const serverEnabled = ref(true);
const serverHost = ref("127.0.0.1");
const serverPort = ref(18080);
const connectTimeoutMs = ref(3000);
const bootstrapName = ref("bootstrap");
const bootstrapPath = ref("scripts/bootstrap.lua");
const instructionLimit = ref(100000);
const securityEnabled = ref(true);
const manifestPath = ref("scripts/bootstrap.manifest.json");
const trustedSignerPublicKey = ref("");
const allowHostLog = ref(true);
const allowConfigRead = ref(true);
const allowDmAccess = ref(false);
const dmBridgePath = ref("dm-bridge/DmBridge.dll");
const submitting = ref(false);
const resultMessage = ref("");

const clientOptions = computed(() =>
  props.clients.map((client) => ({
    id: client.client_id,
    online: client.data.online,
    label: `${client.client_id} / ${client.data.online ? "在线" : "离线"}`,
  })),
);

const selectedStatus = computed(() =>
  props.clients.find((client) => client.client_id === selectedClientId.value) ?? props.status,
);

const canSubmit = computed(() => {
  return (
    !!selectedClientId.value &&
    displayName.value.trim().length > 0 &&
    clientGroup.value.trim().length > 0 &&
    serverHost.value.trim().length > 0 &&
    serverPort.value > 0 &&
    bootstrapName.value.trim().length > 0 &&
    bootstrapPath.value.trim().length > 0 &&
    instructionLimit.value > 0 &&
    connectTimeoutMs.value > 0 &&
    (trustedSignerPublicKey.value.trim().length === 0 ||
      trustedSignerPublicKey.value.trim().length === 64)
  );
});

watch(
  () => props.status?.client_id ?? "",
  (clientId) => {
    if (clientId) {
      selectedClientId.value = clientId;
    }
  },
  { immediate: true },
);

watch(
  () => props.clients.map((client) => client.client_id).join("\n"),
  () => {
    const ids = props.clients.map((client) => client.client_id);
    if (!ids.includes(selectedClientId.value)) {
      selectedClientId.value = ids[0] ?? "";
    }
  },
  { immediate: true },
);

watch(
  () => selectedStatus.value?.client_id ?? "",
  () => {
    applyStatusDefaults();
  },
  { immediate: true },
);

watch(
  () => props.serverUrl,
  () => {
    applyServerUrlDefaults();
  },
  { immediate: true },
);

function applyStatusDefaults(): void {
  const status = selectedStatus.value;
  if (!status) {
    return;
  }

  bootstrapName.value = status.data.script.bootstrap_name || "bootstrap";
  displayName.value = status.data.identity.display_name || status.client_id;
  clientGroup.value = status.data.identity.group || "default";
  clientTags.value = status.data.identity.tags.join(", ");
  instructionLimit.value = status.data.script.instruction_limit || 100000;
  securityEnabled.value = status.data.script.security_enabled;
  allowHostLog.value = status.data.script.allowed_permissions.includes("host.log");
  allowConfigRead.value = status.data.script.allowed_permissions.includes("config.read");
  allowDmAccess.value = status.data.script.allowed_permissions.includes("dm.access");

  if (status.data.server.report_enabled) {
    serverEnabled.value = true;
    applyReportTarget(status.data.server.report_target);
  }
}

function applyServerUrlDefaults(): void {
  try {
    const url = new URL(props.serverUrl);
    const host = url.hostname === "0.0.0.0" ? "127.0.0.1" : url.hostname;
    serverHost.value = host || "127.0.0.1";
    serverPort.value = Number(url.port || (url.protocol === "https:" ? 443 : 80));
  } catch {
    serverHost.value = "127.0.0.1";
    serverPort.value = 18080;
  }
}

function applyReportTarget(reportTarget: string): void {
  try {
    const url = new URL(reportTarget);
    serverHost.value = url.hostname || serverHost.value;
    serverPort.value = Number(url.port || (url.protocol === "https:" ? 443 : 80));
  } catch {
    // 旧版本状态可能只给出 disabled 或非 URL 文本，保持当前表单值。
  }
}

function buildPatch(): ClientConfigPatch {
  const scriptSecurity: ClientScriptSecurityConfigPatch = {
    enabled: securityEnabled.value,
    manifest_path: manifestPath.value.trim(),
    allowed_permissions: selectedPermissions(),
  };

  if (trustedSignerPublicKey.value.trim()) {
    scriptSecurity.trusted_signer_public_key = trustedSignerPublicKey.value.trim();
  }

  return {
    client: {
      display_name: displayName.value.trim(),
      group: clientGroup.value.trim(),
      tags: normalizedTags(),
    },
    lua: {
      bootstrap_name: bootstrapName.value.trim(),
      bootstrap_path: bootstrapPath.value.trim(),
      instruction_limit: instructionLimit.value,
    },
    script_security: {
      ...scriptSecurity,
    },
    dm: {
      bridge_path: dmBridgePath.value.trim(),
    },
    server: {
      enabled: serverEnabled.value,
      host: serverHost.value.trim(),
      port: serverPort.value,
      status_path: "/api/client/status",
      connect_timeout_ms: connectTimeoutMs.value,
    },
  };
}

function normalizedTags(): string[] {
  const seen = new Set<string>();
  const tags: string[] = [];

  for (const rawTag of clientTags.value.split(",")) {
    const tag = rawTag.trim();
    if (!tag || seen.has(tag)) {
      continue;
    }
    seen.add(tag);
    tags.push(tag);
  }

  return tags;
}

function selectedPermissions(): string[] {
  const permissions: string[] = [];
  if (allowHostLog.value) {
    permissions.push("host.log");
  }
  if (allowConfigRead.value) {
    permissions.push("config.read");
  }
  if (allowDmAccess.value) {
    permissions.push("dm.access");
  }
  return permissions;
}

async function submitConfig(): Promise<void> {
  if (!canSubmit.value || submitting.value) {
    return;
  }

  submitting.value = true;
  resultMessage.value = "";

  try {
    const command = await sendClientCommand(props.serverUrl, selectedClientId.value, {
      command_type: "config.apply",
      payload: buildPatch(),
    });
    resultMessage.value = `已写入配置命令队列：${command.id}`;
    emit("refresh");
  } catch (error) {
    resultMessage.value =
      error instanceof Error ? error.message : `配置下发失败：${String(error)}`;
  } finally {
    submitting.value = false;
  }
}
</script>

<template>
  <section class="config-panel">
    <header>
      <ServerCog :size="18" />
      <div>
        <h2>Client 遠程設定</h2>
        <p>Server 下發白名單配置，Client 寫回本機 TOML，下一輪 monitor 套用。</p>
      </div>
    </header>

    <div v-if="!clientOptions.length" class="empty-detail">
      <strong>未發現 Client</strong>
      <span>等待客戶端上報後，這裡會顯示可套用設定的目標。</span>
    </div>

    <form v-else class="config-form" @submit.prevent="submitConfig">
      <label class="wide-field">
        <span>設定目標</span>
        <select v-model="selectedClientId">
          <option
            v-for="client in clientOptions"
            :key="client.id"
            :value="client.id"
          >
            {{ client.label }}
          </option>
        </select>
      </label>

      <fieldset>
        <legend>
          <ServerCog :size="16" />
          <span>Client 身份</span>
        </legend>
        <div class="field-grid">
          <label>
            <span>显示名称</span>
            <input v-model="displayName" />
          </label>
          <label>
            <span>分组</span>
            <input v-model="clientGroup" />
          </label>
          <label>
            <span>标签（逗号分隔）</span>
            <input v-model="clientTags" />
          </label>
        </div>
      </fieldset>

      <fieldset>
        <legend>
          <ServerCog :size="16" />
          <span>Server 上報</span>
        </legend>
        <label class="inline-toggle">
          <input v-model="serverEnabled" type="checkbox" />
          <span>啟用 Client 上報</span>
        </label>
        <div class="field-grid">
          <label>
            <span>Server Host</span>
            <input v-model="serverHost" />
          </label>
          <label>
            <span>Server Port</span>
            <input v-model.number="serverPort" min="1" max="65535" type="number" />
          </label>
          <label>
            <span>連線逾時 ms</span>
            <input v-model.number="connectTimeoutMs" min="1" type="number" />
          </label>
        </div>
      </fieldset>

      <fieldset>
        <legend>
          <FileCode2 :size="16" />
          <span>Lua 腳本</span>
        </legend>
        <div class="field-grid">
          <label>
            <span>Bootstrap 名稱</span>
            <input v-model="bootstrapName" />
          </label>
          <label>
            <span>Bootstrap 路徑</span>
            <input v-model="bootstrapPath" />
          </label>
          <label>
            <span>指令上限</span>
            <input v-model.number="instructionLimit" min="1" type="number" />
          </label>
        </div>
      </fieldset>

      <fieldset>
        <legend>
          <ShieldCheck :size="16" />
          <span>腳本安全門</span>
        </legend>
        <label class="inline-toggle">
          <input v-model="securityEnabled" type="checkbox" />
          <span>啟用 manifest 校驗</span>
        </label>
        <div class="field-grid">
          <label>
            <span>Manifest 路徑</span>
            <input v-model="manifestPath" />
          </label>
          <label class="wide-field">
            <span>Ed25519 公鑰</span>
            <input v-model="trustedSignerPublicKey" maxlength="64" />
          </label>
        </div>
        <div class="permission-grid">
          <label>
            <input v-model="allowHostLog" type="checkbox" />
            <span>host.log</span>
          </label>
          <label>
            <input v-model="allowConfigRead" type="checkbox" />
            <span>config.read</span>
          </label>
          <label>
            <input v-model="allowDmAccess" type="checkbox" />
            <span>dm.access</span>
          </label>
        </div>
      </fieldset>

      <fieldset>
        <legend>
          <Cpu :size="16" />
          <span>DM Bridge</span>
        </legend>
        <label class="wide-field">
          <span>DmBridge.dll 路徑</span>
          <input v-model="dmBridgePath" />
        </label>
      </fieldset>

      <div class="form-actions">
        <button type="submit" :disabled="!canSubmit || submitting">
          <Send :size="15" />
          <span>{{ submitting ? "下發中" : "套用到 Client" }}</span>
        </button>
        <p v-if="resultMessage">
          <ClipboardCheck :size="14" />
          <span>{{ resultMessage }}</span>
        </p>
      </div>
    </form>
  </section>
</template>

<style scoped>
.config-panel {
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
p {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 16px;
}

header p,
.form-actions p,
.empty-detail {
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

.config-form {
  display: grid;
  gap: var(--space-4);
}

fieldset {
  display: grid;
  gap: var(--space-3);
  margin: 0;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-page);
  padding: var(--space-4);
}

legend {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: 0 var(--space-1);
  color: var(--color-text);
  font-size: 13px;
  font-weight: 800;
}

.field-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-3);
}

label {
  display: grid;
  gap: var(--space-2);
}

label span {
  color: var(--color-muted);
  font-size: 12px;
  font-weight: 760;
}

input,
select {
  min-width: 0;
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

.wide-field {
  grid-column: 1 / -1;
}

.inline-toggle,
.permission-grid label {
  display: inline-flex;
  grid-template-columns: auto minmax(0, 1fr);
  align-items: center;
  gap: var(--space-2);
}

.inline-toggle input,
.permission-grid input {
  width: 16px;
  height: 16px;
}

.permission-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
}

.form-actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--space-3);
}

.form-actions button {
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

.form-actions button:disabled {
  opacity: 0.6;
}

.form-actions p {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
}

.empty-detail {
  display: grid;
  gap: var(--space-1);
}

.empty-detail strong {
  color: var(--color-text);
  font-size: 14px;
}

@media (max-width: 980px) {
  .field-grid {
    grid-template-columns: 1fr;
  }
}
</style>
