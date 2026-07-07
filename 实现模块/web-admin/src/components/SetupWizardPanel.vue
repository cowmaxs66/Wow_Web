<script setup lang="ts">
import {
  CheckCircle2,
  Clipboard,
  FileCode2,
  MonitorCog,
  Network,
  PlayCircle,
  RotateCcw,
  Settings2,
  ShieldCheck,
  Terminal,
} from "@lucide/vue";
import { ref } from "vue";
import { useSetupWizard } from "../composables/useSetupWizard";

const props = defineProps<{
  serverUrl: string;
  clientId: string;
}>();

const emit = defineEmits<{
  "update:serverUrl": [value: string];
  "update:clientId": [value: string];
  apply: [];
}>();

const {
  profile,
  copiedTarget,
  serverUrl: wizardServerUrl,
  browserHost,
  isCompleted,
  architectureLabel,
  architectureNote,
  serverCommand,
  clientCommand,
  startupStatusCommand,
  enableStartupCommand,
  disableStartupCommand,
  applyDashboardDefaults,
  markCompleted,
  reset,
  copyText,
} = useSetupWizard();

const expanded = ref(!isCompleted.value);
const showAdvancedCommands = ref(false);

function syncFromDashboard(): void {
  applyDashboardDefaults(props.serverUrl, props.clientId);
}

function applyToDashboard(): void {
  emit("update:serverUrl", wizardServerUrl.value);
  emit("update:clientId", profile.value.clientId);
}

function completeSetup(): void {
  applyToDashboard();
  markCompleted();
  expanded.value = false;
  emit("apply");
}
</script>

<template>
  <section class="setup-panel" :class="{ completed: isCompleted }">
    <header>
      <div class="title-row">
        <Settings2 :size="19" />
        <div>
          <h2>首次設定向導</h2>
          <p>{{ architectureLabel }} · {{ architectureNote }}</p>
        </div>
      </div>
      <button class="ghost-button" type="button" @click="expanded = !expanded">
        <MonitorCog :size="16" />
        <span>{{ expanded ? "收起" : "展開" }}</span>
      </button>
    </header>

    <div v-if="expanded" class="setup-body">
      <div class="simple-flow" aria-label="標準操作流程">
        <article>
          <PlayCircle :size="18" />
          <div>
            <strong>1. 打開 WoW-Manager.exe</strong>
            <span>在控制中心點擊啟動 Server 與 Client，不需要手動敲命令。</span>
          </div>
        </article>
        <article>
          <Network :size="18" />
          <div>
            <strong>2. Web 連線到 {{ wizardServerUrl }}</strong>
            <span v-if="profile.serverHost === '0.0.0.0'">
              Server 仍監聽 0.0.0.0，但瀏覽器會改用 {{ browserHost }} 連線。
            </span>
            <span v-else>Server 地址會同步到左側看板與 Client 上報設定。</span>
          </div>
        </article>
        <article>
          <ShieldCheck :size="18" />
          <div>
            <strong>3. DM/Lua 由 Client 本機套用</strong>
            <span>Client 啟動後讀取 config/client-agent.toml，再執行 scripts/bootstrap.lua。</span>
          </div>
        </article>
      </div>

      <div class="form-grid">
        <label>
          <span>Server 監聽地址</span>
          <input v-model="profile.serverHost" autocomplete="off" spellcheck="false" />
        </label>
        <label>
          <span>Server Port</span>
          <input v-model="profile.serverPort" inputmode="numeric" autocomplete="off" />
        </label>
        <label>
          <span>預設 Client ID</span>
          <input v-model="profile.clientId" autocomplete="off" spellcheck="false" />
        </label>
        <label>
          <span>歷史文件</span>
          <input v-model="profile.historyPath" autocomplete="off" spellcheck="false" />
        </label>
        <label>
          <span>Web 目錄</span>
          <input v-model="profile.webDir" autocomplete="off" spellcheck="false" />
        </label>
        <label>
          <span>Client 運行模式</span>
          <select v-model="profile.clientMode">
            <option value="x64-core">x64 核心模式</option>
            <option value="x86-dm">x86 DM 模式</option>
          </select>
        </label>
        <label v-if="profile.clientMode === 'x86-dm'" class="wide-field">
          <span>DmBridge 路徑</span>
          <input v-model="profile.dmBridgePath" autocomplete="off" spellcheck="false" />
        </label>
        <label class="check-field">
          <input v-model="profile.reportEnabled" type="checkbox" />
          <span>啟用 Client 上報</span>
        </label>
      </div>

      <div class="mode-note">
        <FileCode2 :size="18" />
        <div>
          <strong>{{ architectureLabel }}</strong>
          <span>{{ architectureNote }}</span>
        </div>
      </div>

      <div class="advanced-toggle">
        <button type="button" class="ghost-button" @click="showAdvancedCommands = !showAdvancedCommands">
          <Terminal :size="16" />
          <span>{{ showAdvancedCommands ? "隱藏進階排錯命令" : "顯示進階排錯命令" }}</span>
        </button>
      </div>

      <div v-if="showAdvancedCommands" class="command-grid">
        <article>
          <div class="command-title">
            <strong>Server 排錯命令</strong>
            <button type="button" @click="copyText('server', serverCommand)">
              <Clipboard :size="14" />
              <span>{{ copiedTarget === "server" ? "已複製" : "複製" }}</span>
            </button>
          </div>
          <pre>{{ serverCommand }}</pre>
        </article>

        <article>
          <div class="command-title">
            <strong>Client 排錯命令</strong>
            <button type="button" @click="copyText('client', clientCommand)">
              <Clipboard :size="14" />
              <span>{{ copiedTarget === "client" ? "已複製" : "複製" }}</span>
            </button>
          </div>
          <pre>{{ clientCommand }}</pre>
        </article>

        <article>
          <div class="command-title">
            <strong>Client 開機啟動</strong>
            <div class="command-actions">
              <button type="button" @click="copyText('startup-status', startupStatusCommand)">
                <Clipboard :size="14" />
                <span>{{ copiedTarget === "startup-status" ? "已複製" : "查詢" }}</span>
              </button>
              <button type="button" @click="copyText('startup-enable', enableStartupCommand)">
                <Clipboard :size="14" />
                <span>{{ copiedTarget === "startup-enable" ? "已複製" : "啟用" }}</span>
              </button>
              <button type="button" @click="copyText('startup-disable', disableStartupCommand)">
                <Clipboard :size="14" />
                <span>{{ copiedTarget === "startup-disable" ? "已複製" : "停用" }}</span>
              </button>
            </div>
          </div>
          <pre>{{ startupStatusCommand + "\n" + enableStartupCommand + "\n" + disableStartupCommand }}</pre>
        </article>
      </div>

      <div class="actions">
        <button type="button" class="ghost-button" @click="syncFromDashboard">
          <RotateCcw :size="16" />
          <span>讀取看板設定</span>
        </button>
        <button type="button" class="ghost-button" @click="reset">
          <RotateCcw :size="16" />
          <span>重置</span>
        </button>
        <button type="button" class="primary-button" @click="completeSetup">
          <CheckCircle2 :size="17" />
          <span>套用並完成</span>
        </button>
      </div>
    </div>
  </section>
</template>

<style scoped>
.setup-panel {
  display: grid;
  gap: var(--space-4);
  margin-bottom: var(--space-5);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  padding: var(--space-5);
}

.setup-panel.completed {
  border-color: rgba(21, 128, 61, 0.25);
}

header,
.title-row,
.actions,
.command-title {
  display: flex;
  align-items: center;
}

header {
  justify-content: space-between;
  gap: var(--space-4);
}

.title-row {
  min-width: 0;
  gap: var(--space-3);
  color: var(--color-accent);
}

h2,
p {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 17px;
  line-height: 1.3;
}

p {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 13px;
  line-height: 1.5;
}

.setup-body {
  display: grid;
  gap: var(--space-4);
}

.simple-flow {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-3);
}

.simple-flow article,
.mode-note {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-page);
  color: var(--color-accent);
  padding: var(--space-3);
}

.simple-flow strong,
.mode-note strong {
  display: block;
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.35;
}

.simple-flow span,
.mode-note span {
  display: block;
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.55;
}

.form-grid {
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
  padding: 10px var(--space-3);
  font-size: 13px;
  outline: none;
}

input:focus,
select:focus {
  border-color: var(--color-accent);
  box-shadow: 0 0 0 3px rgba(33, 95, 154, 0.12);
}

.wide-field {
  grid-column: span 2;
}

.check-field {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding-top: 22px;
}

.check-field input {
  width: 16px;
  height: 16px;
}

.command-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
}

.advanced-toggle {
  display: flex;
  justify-content: flex-end;
}

article {
  display: grid;
  gap: var(--space-2);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-surface-muted);
  padding: var(--space-3);
}

.command-title {
  justify-content: space-between;
  gap: var(--space-3);
}

.command-actions {
  display: inline-flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: var(--space-2);
}

.command-title strong {
  font-size: 13px;
}

pre {
  overflow-x: auto;
  margin: 0;
  border-radius: var(--radius-control);
  background: #172033;
  color: #f8fbff;
  padding: var(--space-3);
  font-size: 12px;
  line-height: 1.55;
}

button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  border-radius: var(--radius-control);
  padding: 9px var(--space-3);
  font-size: 13px;
  font-weight: 760;
}

.ghost-button,
.command-title button {
  border: 1px solid var(--color-border-strong);
  background: #ffffff;
  color: var(--color-text);
}

.primary-button {
  border: 0;
  background: var(--color-accent);
  color: #ffffff;
}

.actions {
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: var(--space-3);
}

@media (max-width: 960px) {
  .simple-flow,
  .form-grid,
  .command-grid {
    grid-template-columns: 1fr;
  }

  .wide-field {
    grid-column: auto;
  }
}

@media (max-width: 720px) {
  header {
    display: grid;
  }

  .actions {
    justify-content: flex-start;
  }
}
</style>
