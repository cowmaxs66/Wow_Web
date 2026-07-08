<script setup lang="ts">
import { FileUp, WandSparkles } from "@lucide/vue";
import { computed, ref } from "vue";
import { sendClientCommand } from "../api/managementServer";
import type { ClientScriptDeployBundle } from "../types/protocol";

const DEFAULT_PUBLIC_KEY = "7f3c8268b3f403594ef6e7e681ca62851e66b07b915c48a523b0cfaf7e54bfc9";
const DEFAULT_PERMISSIONS = ["host.log", "config.read", "dm.access"];

const props = defineProps<{
  serverUrl: string;
  targetClientIds: string[];
  disabled: boolean;
}>();

const bootstrapName = ref("bootstrap");
const bootstrapPath = ref("scripts/bootstrap.lua");
const manifestPath = ref("scripts/bootstrap.manifest.json");
const publicKey = ref(DEFAULT_PUBLIC_KEY);
const luaContent = ref("");
const manifestContent = ref("");
const activate = ref(true);
const runAfterDeploy = ref(false);
const securityEnabled = ref(false);
const selectedPermissions = ref<string[]>([...DEFAULT_PERMISSIONS]);
const deploying = ref(false);
const resultMessage = ref("");
const lastDeployFingerprint = ref("");
const lastDeployAt = ref(0);

const canDeploy = computed(
  () =>
    props.targetClientIds.length > 0 &&
    !props.disabled &&
    !deploying.value &&
    !!bootstrapName.value.trim() &&
    !!bootstrapPath.value.trim() &&
    !!luaContent.value.trim() &&
    (!securityEnabled.value || (!!manifestPath.value.trim() && !!manifestContent.value.trim())),
);

function isPermissionSelected(permission: string): boolean {
  return selectedPermissions.value.includes(permission);
}

function setPermission(permission: string, checked: boolean): void {
  const next = new Set(selectedPermissions.value);
  if (checked) {
    next.add(permission);
  } else {
    next.delete(permission);
  }

  selectedPermissions.value = DEFAULT_PERMISSIONS.filter((item) => next.has(item));
}

function buildPayload(): ClientScriptDeployBundle {
  const payload: ClientScriptDeployBundle = {
    bootstrap_name: bootstrapName.value.trim(),
    bootstrap_path: bootstrapPath.value.trim(),
    lua_content: luaContent.value,
    security_enabled: securityEnabled.value,
    allowed_permissions: selectedPermissions.value,
    activate: activate.value,
    run_after_deploy: runAfterDeploy.value,
  };

  if (securityEnabled.value && manifestPath.value.trim()) {
    payload.manifest_path = manifestPath.value.trim();
  }
  if (securityEnabled.value && manifestContent.value.trim()) {
    payload.manifest_content = manifestContent.value;
  }
  if (publicKey.value.trim()) {
    payload.trusted_signer_public_key = publicKey.value.trim();
  }

  return payload;
}

function fillDmApiSelfTestSample(): void {
  bootstrapName.value = "dm-api-selftest";
  bootstrapPath.value = "scripts/dm_api_selftest_remote.lua";
  securityEnabled.value = false;
  activate.value = true;
  runAfterDeploy.value = false;
  selectedPermissions.value = [...DEFAULT_PERMISSIONS];
  luaContent.value = [
    "log('dm api selftest started')",
    "local abi = dm.abi_version()",
    "dm.init('')",
    "local version = dm.ver()",
    "dm.set_path('.')",
    "local color = dm.get_color_rgb(0, 0)",
    "local wait_ok, wait_color = dm.wait_color(0, 0, color.hex, 300, 50)",
    "local missing_hwnd = dm.find_window('', '__WOW_FRAMEWORK_SELFTEST_WINDOW_SHOULD_NOT_EXIST__')",
    "local bind_ok, bind_err = dm.safe_bind_window(0, 'normal', 'windows', 'windows', 0)",
    "local bridge_error = dm.last_bridge_error()",
    "local dm_error = dm.last_dm_error()",
    "dm.shutdown()",
    "return 'dm api selftest ok: abi=' .. tostring(abi)",
    "  .. ' ver=' .. tostring(version)",
    "  .. ' color=' .. tostring(color.hex)",
    "  .. ' wait=' .. tostring(wait_ok) .. '/' .. tostring(wait_color)",
    "  .. ' missing_hwnd=' .. tostring(missing_hwnd)",
    "  .. ' invalid_bind_ok=' .. tostring(bind_ok)",
    "  .. ' bridge_error=' .. tostring(bridge_error)",
    "  .. ' dm_error=' .. tostring(dm_error)",
  ].join("\n");
}

function fillDmSmokeSample(): void {
  bootstrapName.value = "dm-window-smoke";
  bootstrapPath.value = "scripts/dm_window_smoke_remote.lua";
  securityEnabled.value = false;
  activate.value = true;
  runAfterDeploy.value = false;
  selectedPermissions.value = [...DEFAULT_PERMISSIONS];
  luaContent.value = [
    "local hwnd = dm.find_window('', 'World of Warcraft')",
    "if hwnd <= 0 then",
    "  log('未找到 World of Warcraft 窗口，跳过绑定测试')",
    "  return 'dm window smoke skipped: World of Warcraft not found'",
    "end",
    "local ok, err = dm.safe_bind_window(hwnd, 'normal', 'windows', 'windows', 0)",
    "if not ok then",
    "  log('DM 绑定失败：' .. err)",
    "  return 'bind failed'",
    "end",
    "local color = dm.get_color_rgb(0, 0)",
    "dm.unbind_window()",
    "return 'dm window smoke ok: ' .. color.hex",
  ].join("\n");
}

function deployFingerprint(payload: ClientScriptDeployBundle): string {
  return JSON.stringify({
    targets: props.targetClientIds,
    payload,
  });
}

async function deployScript(): Promise<void> {
  if (!canDeploy.value) {
    return;
  }

  deploying.value = true;
  resultMessage.value = "";

  try {
    const payload = buildPayload();
    const fingerprint = deployFingerprint(payload);
    const now = Date.now();
    if (fingerprint === lastDeployFingerprint.value && now - lastDeployAt.value < 15000) {
      resultMessage.value = "已拦截重复脚本推送：15 秒内不要重复下发同一批目标和同一份脚本。";
      return;
    }

    const commands = await Promise.all(
      props.targetClientIds.map((clientId) =>
        sendClientCommand(props.serverUrl, clientId, {
          command_type: "script.deploy_bundle",
          payload,
        }),
      ),
    );
    resultMessage.value =
      commands.length === 1
        ? `已写入脚本推送命令：${commands[0].id}`
        : `已向 ${commands.length} 台 Client 写入脚本推送命令`;
    lastDeployFingerprint.value = fingerprint;
    lastDeployAt.value = now;
  } catch (error) {
    resultMessage.value =
      error instanceof Error ? error.message : `脚本推送失败：${String(error)}`;
  } finally {
    deploying.value = false;
  }
}
</script>

<template>
  <form class="script-deploy-panel" @submit.prevent="deployScript">
    <div class="deploy-heading">
      <FileUp :size="17" />
      <div>
        <h3>脚本推送</h3>
        <p>选中 Client 热推送 Lua</p>
      </div>
      <button class="sample-button" type="button" @click="fillDmApiSelfTestSample">
        <WandSparkles :size="15" />
        <span>API 自检</span>
      </button>
      <button class="sample-button" type="button" @click="fillDmSmokeSample">
        <WandSparkles :size="15" />
        <span>窗口 smoke</span>
      </button>
    </div>

    <div class="field-grid">
      <label>
        <span>Script ID</span>
        <input v-model="bootstrapName" maxlength="80" />
      </label>
      <label>
        <span>Lua 路径</span>
        <input v-model="bootstrapPath" maxlength="160" />
      </label>
      <label>
        <span>Manifest 路径</span>
        <input v-model="manifestPath" maxlength="160" :disabled="!securityEnabled" />
      </label>
    </div>

    <label>
      <span>Ed25519 公钥</span>
      <input v-model="publicKey" maxlength="64" :disabled="!securityEnabled" />
    </label>

    <div class="permission-row">
      <label
        v-for="permission in DEFAULT_PERMISSIONS"
        :key="permission"
      >
        <input
          type="checkbox"
          :checked="isPermissionSelected(permission)"
          @change="setPermission(permission, ($event.target as HTMLInputElement).checked)"
        />
        <span>{{ permission }}</span>
      </label>
    </div>

    <div class="content-grid">
      <label>
        <span>Lua 内容</span>
        <textarea v-model="luaContent" rows="9" spellcheck="false" />
      </label>
      <label>
        <span>Manifest JSON</span>
        <textarea v-model="manifestContent" rows="9" spellcheck="false" :disabled="!securityEnabled" />
      </label>
    </div>

    <div class="option-row">
      <label>
        <input v-model="securityEnabled" type="checkbox" />
        <span>启用 manifest 校验</span>
      </label>
      <label>
        <input v-model="activate" type="checkbox" />
        <span>切换为当前脚本</span>
      </label>
      <label>
        <input v-model="runAfterDeploy" type="checkbox" />
        <span>推送后执行</span>
      </label>
    </div>

    <p v-if="runAfterDeploy" class="run-warning">
      推送后会立即在目标机器执行。实机调试建议先只推送，再用“启动 Lua”观察回执。
    </p>

    <button type="submit" :disabled="!canDeploy">
      <FileUp :size="15" />
      <span>{{ deploying ? "推送中" : "推送脚本包" }}</span>
    </button>

    <p v-if="resultMessage">{{ resultMessage }}</p>
  </form>
</template>

<style scoped>
.script-deploy-panel {
  display: grid;
  gap: var(--space-3);
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-3);
}

.deploy-heading {
  display: flex;
  align-items: flex-start;
  gap: var(--space-2);
  color: var(--color-accent);
}

.deploy-heading > div {
  min-width: 0;
}

.deploy-heading .sample-button {
  margin-left: auto;
  border: 1px solid var(--color-border-strong);
  background: #ffffff;
  color: var(--color-text);
  white-space: nowrap;
}

h3,
p {
  margin: 0;
}

h3 {
  color: var(--color-text);
  font-size: 13px;
}

.deploy-heading p,
.script-deploy-panel > p {
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.5;
}

.field-grid,
.content-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
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
textarea {
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

textarea {
  min-height: 190px;
  resize: vertical;
  font-family: ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", monospace;
  line-height: 1.45;
}

.permission-row,
.option-row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
}

.permission-row label,
.option-row label {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
}

.permission-row input,
.option-row input {
  width: 15px;
  height: 15px;
  padding: 0;
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
  padding: 9px var(--space-3);
  font-size: 13px;
  font-weight: 760;
}

button:disabled {
  opacity: 0.6;
}

.run-warning {
  border: 1px solid rgba(161, 92, 7, 0.28);
  border-radius: var(--radius-control);
  background: #fff7ed;
  color: var(--color-warning);
  padding: var(--space-3);
  font-size: 12px;
  font-weight: 760;
}

@media (max-width: 720px) {
  .deploy-heading {
    display: grid;
  }

  .sample-button {
    margin-left: 0;
  }
}
</style>
