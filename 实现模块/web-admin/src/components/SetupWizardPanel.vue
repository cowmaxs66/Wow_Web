<script setup lang="ts">
import {
  CheckCircle2,
  Clipboard,
  MonitorCog,
  RotateCcw,
  Settings2,
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
  isCompleted,
  architectureLabel,
  architectureNote,
  serverCommand,
  clientCommand,
  applyDashboardDefaults,
  markCompleted,
  reset,
  copyText,
} = useSetupWizard();

const expanded = ref(!isCompleted.value);

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
      <div class="form-grid">
        <label>
          <span>Server Host</span>
          <input v-model="profile.serverHost" autocomplete="off" spellcheck="false" />
        </label>
        <label>
          <span>Server Port</span>
          <input v-model="profile.serverPort" inputmode="numeric" autocomplete="off" />
        </label>
        <label>
          <span>Client ID</span>
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
          <span>Client 模式</span>
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

      <div class="command-grid">
        <article>
          <div class="command-title">
            <strong>Server 啟動命令</strong>
            <button type="button" @click="copyText('server', serverCommand)">
              <Clipboard :size="14" />
              <span>{{ copiedTarget === "server" ? "已複製" : "複製" }}</span>
            </button>
          </div>
          <pre>{{ serverCommand }}</pre>
        </article>

        <article>
          <div class="command-title">
            <strong>Client 啟動命令</strong>
            <button type="button" @click="copyText('client', clientCommand)">
              <Clipboard :size="14" />
              <span>{{ copiedTarget === "client" ? "已複製" : "複製" }}</span>
            </button>
          </div>
          <pre>{{ clientCommand }}</pre>
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
