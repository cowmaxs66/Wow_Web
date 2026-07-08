<script setup lang="ts">
import { Clipboard, Cpu, FileCode2, PlayCircle, ShieldCheck } from "@lucide/vue";
import { computed, ref } from "vue";
import type { ClientStatusEnvelope } from "../types/protocol";

const props = defineProps<{
  status: ClientStatusEnvelope | null;
}>();

const copied = ref(false);

const hasDmAccess = computed(() => {
  return props.status?.data.script.allowed_permissions.includes("dm.access") ?? false;
});

const currentScript = computed(() => {
  return props.status?.data.current_script ?? "bootstrap";
});

const sampleLua = `log("dm bootstrap started")
dm.init("")

local hwnd = dm.find_window("", "窗口标题")
if hwnd <= 0 then
  dm.shutdown()
  return "window not found"
end

local ok, err = dm.safe_bind_window(hwnd, "normal", "windows", "windows", 0)
if not ok then
  log("bind failed: " .. err)
  dm.shutdown()
  return "bind failed"
end

local color = dm.get_color(10, 10)
dm.unbind_window()
dm.shutdown()

return "dm color=" .. color`;

async function copySample(): Promise<void> {
  await navigator.clipboard.writeText(sampleLua);
  copied.value = true;
  window.setTimeout(() => {
    copied.value = false;
  }, 1600);
}
</script>

<template>
  <section class="dm-lua-panel">
    <header>
      <FileCode2 :size="18" />
      <div>
        <h2>DM / Lua 使用流程</h2>
        <p>Client 本機讀取設定與腳本，Server 只負責查看狀態和下發白名單操作。</p>
      </div>
    </header>

    <div class="status-strip">
      <div>
        <span>當前 Client</span>
        <strong>{{ status?.client_id ?? "未選擇" }}</strong>
      </div>
      <div>
        <span>目前腳本</span>
        <strong>{{ currentScript }}</strong>
      </div>
      <div :data-state="hasDmAccess ? 'ok' : 'warn'">
        <span>DM 權限</span>
        <strong>{{ hasDmAccess ? "已允許 dm.access" : "未允許 dm.access" }}</strong>
      </div>
    </div>

    <div class="flow-grid">
      <article>
        <PlayCircle :size="18" />
        <div>
          <strong>Lua 何時執行</strong>
          <span>Client 啟動、monitor 每輪刷新、收到「重新執行 Lua」或熱推送後立即執行時，會執行目前配置的 bootstrap。</span>
        </div>
      </article>
      <article>
        <ShieldCheck :size="18" />
        <div>
          <strong>安全門如何生效</strong>
          <span>開啟安全門後，manifest 的 hash、簽名和 permissions 必須通過，Lua 才會進入 mlua 執行。</span>
        </div>
      </article>
      <article>
        <Cpu :size="18" />
        <div>
          <strong>DM 模式如何套用</strong>
          <span>Client 使用 Win32 DmBridge.dll，並在 manifest 與 allowed_permissions 中同時允許 dm.access。</span>
        </div>
      </article>
    </div>

    <div class="checklist">
      <h3>最小落地步驟</h3>
      <ol>
        <li>在「遠程操作」勾選 Client，使用 Lua 熱推送直接寫入 <code>scripts/bootstrap.lua</code>。</li>
        <li>需要手動放置腳本時，在 <code>config/client-agent.toml</code> 指定 <code>lua.bootstrap_path</code> 和 <code>dm.bridge_path</code>。</li>
        <li>內部測試模式可不啟用 manifest；重新開啟安全門後，manifest 和 TOML 都要允許 <code>dm.access</code>。</li>
        <li>绑定窗口优先用 <code>dm.safe_bind_window</code>；<code>dx</code> 模式失败时先试 <code>normal</code> 或 <code>gdi</code>。</li>
      </ol>
    </div>

    <div class="sample-block">
      <div class="sample-title">
        <strong>DM Lua 最小樣例</strong>
        <button type="button" @click="copySample">
          <Clipboard :size="14" />
          <span>{{ copied ? "已複製" : "複製樣例" }}</span>
        </button>
      </div>
      <pre>{{ sampleLua }}</pre>
    </div>
  </section>
</template>

<style scoped>
.dm-lua-panel {
  display: grid;
  gap: var(--space-4);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-panel);
  background: var(--color-surface);
  padding: var(--space-5);
  box-shadow: var(--shadow-panel);
}

header,
.flow-grid article,
.sample-title {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
}

header {
  color: var(--color-accent);
}

h2,
h3,
p,
ol {
  margin: 0;
}

h2 {
  color: var(--color-text);
  font-size: 16px;
}

header p {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 13px;
  line-height: 1.6;
}

.status-strip {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-3);
}

.status-strip div,
.flow-grid article,
.checklist {
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: var(--color-page);
  padding: var(--space-3);
}

.status-strip div[data-state="ok"] {
  border-color: rgba(8, 127, 122, 0.25);
  background: var(--color-teal-soft);
}

.status-strip div[data-state="warn"] {
  border-color: rgba(161, 92, 7, 0.25);
  background: #fff7ed;
}

.status-strip span,
.flow-grid span {
  display: block;
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

.status-strip strong,
.flow-grid strong {
  display: block;
  overflow-wrap: anywhere;
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.45;
}

.flow-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--space-3);
}

.flow-grid article {
  color: var(--color-accent);
}

.checklist {
  display: grid;
  gap: var(--space-2);
}

h3 {
  color: var(--color-text);
  font-size: 13px;
}

ol {
  padding-left: 20px;
  color: var(--color-text);
  font-size: 13px;
  line-height: 1.75;
}

code {
  border-radius: 4px;
  background: #e8eef5;
  padding: 1px 5px;
  color: var(--color-text);
  font-size: 12px;
}

.sample-block {
  display: grid;
  gap: var(--space-2);
}

.sample-title {
  justify-content: space-between;
}

.sample-title strong {
  color: var(--color-text);
  font-size: 13px;
}

.sample-title button {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: #ffffff;
  color: var(--color-text);
  padding: 7px var(--space-2);
  font-size: 12px;
  font-weight: 760;
}

pre {
  overflow-x: auto;
  margin: 0;
  border-radius: var(--radius-control);
  background: #111827;
  color: #d1fae5;
  padding: var(--space-4);
  font-size: 12px;
  line-height: 1.55;
}

@media (max-width: 900px) {
  .status-strip,
  .flow-grid {
    grid-template-columns: 1fr;
  }
}
</style>
