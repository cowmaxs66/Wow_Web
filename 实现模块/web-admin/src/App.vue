<script setup lang="ts">
import {
  Activity,
  Bolt,
  Clock3,
  FileCode2,
  ListChecks,
  MonitorCheck,
  RefreshCw,
  ScrollText,
  Server,
  ShieldCheck,
} from "@lucide/vue";
import { computed, ref } from "vue";
import AppShell from "./components/AppShell.vue";
import AuditPanel from "./components/AuditPanel.vue";
import ClientConfigApplyPanel from "./components/ClientConfigApplyPanel.vue";
import ClientDetail from "./components/ClientDetail.vue";
import ClientRemoteActions from "./components/ClientRemoteActions.vue";
import ClientSettingsPanel from "./components/ClientSettingsPanel.vue";
import ClientTable from "./components/ClientTable.vue";
import DmLuaGuidePanel from "./components/DmLuaGuidePanel.vue";
import HistoryTrendPanel from "./components/HistoryTrendPanel.vue";
import MetricCard from "./components/MetricCard.vue";
import ScriptPanel from "./components/ScriptPanel.vue";
import SnapshotAnalytics from "./components/SnapshotAnalytics.vue";
import StatusDot from "./components/StatusDot.vue";
import SetupWizardPanel from "./components/SetupWizardPanel.vue";
import { useDashboardStatus } from "./composables/useDashboardStatus";

const {
  serverUrl,
  clientId,
  health,
  clients,
  clientPage,
  clientPageSize,
  clientTotal,
  clientTotalPages,
  clientSearch,
  clientGroupFilter,
  clientTagFilter,
  clientOnlineFilter,
  selectedHistory,
  historyLimit,
  selectedClientId,
  loading,
  errorMessage,
  selectedStatus,
  onlineCount,
  offlineCount,
  securityEnabledCount,
  currentScript,
  selectedReleaseVersion,
  healthLabel,
  lastReportLabel,
  lastRefreshLabel,
  refreshDashboard,
} = useDashboardStatus();

type AdminView =
  | "overview"
  | "clients"
  | "scripts"
  | "operations"
  | "logs"
  | "settings";

const activeView = ref<AdminView>("overview");
const allowedViews = new Set<AdminView>([
  "overview",
  "clients",
  "scripts",
  "operations",
  "logs",
  "settings",
]);

const viewMeta = computed(() => {
  switch (activeView.value) {
    case "clients":
      return {
        title: "客戶端管理",
        description: "查看上線狀態、最近上報、歷史趨勢與單機詳細信息。",
      };
    case "scripts":
      return {
        title: "腳本配置",
        description: "查看 Client 如何執行 Lua、如何套用 DM 權限，以及如何遠程重跑 bootstrap。",
      };
    case "operations":
      return {
        title: "遠程操作",
        description: "選擇單台或多台 Client，下發消息、Lua、設定和本機操作。",
      };
    case "logs":
      return {
        title: "日誌與回執",
        description: "集中查看 Server 審計、命令回執與最近一次 Client 上報。",
      };
    case "settings":
      return {
        title: "設定",
        description: "管理控制台連線、首次部署向導與 Client 遠程設定。",
      };
    default:
      return {
        title: "總覽",
        description: "先確認 Server 與 Client 狀態，再選擇機器進行腳本、設定或日志排查。",
      };
  }
});

const clientTableProps = computed(() => ({
  page: clientPage.value,
  pageSize: clientPageSize.value,
  total: clientTotal.value,
  totalPages: clientTotalPages.value,
  searchText: clientSearch.value,
  groupFilter: clientGroupFilter.value,
  tagFilter: clientTagFilter.value,
  onlineFilter: clientOnlineFilter.value,
}));

const selectedClientTitle = computed(() => {
  const status = selectedStatus.value;
  return status?.data.identity.display_name || status?.client_id || "未選擇 Client";
});

const selectedClientSubtitle = computed(() => {
  const status = selectedStatus.value;
  if (!status) {
    return "請先在客戶端列表中選擇目標";
  }

  const group = status.data.identity.group || "default";
  const tags = status.data.identity.tags.length
    ? status.data.identity.tags.join(", ")
    : "無標籤";
  return `${status.client_id} / ${group} / ${tags}`;
});

const selectedRuntimeMode = computed(() => {
  const status = selectedStatus.value;
  if (!status) {
    return "未選擇";
  }

  const arch = status.data.runtime.arch || "unknown";
  const hasDmAccess = status.data.script.allowed_permissions.includes("dm.access");
  return hasDmAccess ? `${arch} / DM` : `${arch} / Core`;
});

const selectedScriptState = computed(() => {
  const status = selectedStatus.value;
  if (!status) {
    return "未選擇";
  }

  return status.data.script.enabled
    ? `${status.data.current_script ?? status.data.script.bootstrap_name}`
    : "Lua 已停用";
});

const selectedStatusTone = computed<"online" | "offline" | "idle">(() => {
  if (!selectedStatus.value) {
    return "idle";
  }

  return selectedStatus.value.data.online ? "online" : "offline";
});

const selectedStatusLabel = computed(() => {
  if (!selectedStatus.value) {
    return "未選擇";
  }

  return selectedStatus.value.data.online ? "Client 在線" : "Client 離線";
});

const primaryActionText = computed(() => {
  if (!selectedStatus.value) {
    return "先選擇 Client";
  }

  if (!selectedStatus.value.data.online) {
    return "先檢查離線原因";
  }

  return "可下發命令";
});

function changeView(view: string): void {
  if (allowedViews.has(view as AdminView)) {
    activeView.value = view as AdminView;
  }
}

function updateClientSearch(value: string): void {
  clientSearch.value = value;
}

function updateClientGroupFilter(value: string): void {
  clientGroupFilter.value = value;
}

function updateClientTagFilter(value: string): void {
  clientTagFilter.value = value;
}

function updateClientOnlineFilter(value: "all" | "online" | "offline"): void {
  clientOnlineFilter.value = value;
}

function updateClientPageSize(value: number): void {
  clientPageSize.value = value;
}

function applyClientFilters(): void {
  clientPage.value = 1;
  void refreshDashboard();
}

function changeClientPage(page: number): void {
  clientPage.value = page;
  void refreshDashboard();
}
</script>

<template>
  <AppShell
    :active-view="activeView"
    :server-label="healthLabel"
    :server-url="serverUrl"
    @navigate="changeView"
  >
    <header class="topbar">
      <div class="title-block">
        <h1>{{ viewMeta.title }}</h1>
        <p>{{ viewMeta.description }}</p>
      </div>
      <div class="topbar-actions">
        <div class="target-card" :data-online="selectedStatus?.data.online ? 'true' : 'false'">
          <StatusDot :tone="selectedStatusTone" :label="selectedStatusLabel" />
          <strong>{{ selectedClientTitle }}</strong>
          <span class="target-subtitle">{{ selectedClientSubtitle }}</span>
        </div>
        <StatusDot
          :tone="health === 'online' ? 'online' : health === 'offline' ? 'offline' : 'idle'"
          :label="healthLabel"
        />
        <button type="button" :disabled="loading" @click="refreshDashboard">
          <RefreshCw :size="16" :class="{ spinning: loading }" />
          <span>刷新</span>
        </button>
      </div>
    </header>

    <p v-if="errorMessage" class="error-banner">{{ errorMessage }}</p>

    <template v-if="activeView === 'overview'">
      <section class="metrics-grid" aria-label="狀態指標">
        <MetricCard
          label="Server 健康"
          :value="healthLabel"
          :note="serverUrl"
          :icon="Server"
          :tone="health === 'online' ? 'success' : 'warning'"
        />
        <MetricCard
          label="在線客戶端"
          :value="`${onlineCount}/${clients.length}`"
          note="超過心跳窗口會自動顯示離線"
          :icon="MonitorCheck"
          tone="success"
        />
        <MetricCard
          label="目前目標"
          :value="primaryActionText"
          :note="selectedRuntimeMode"
          :icon="ListChecks"
        />
        <MetricCard
          label="當前腳本"
          :value="selectedScriptState"
          :note="`Agent 版本：${selectedReleaseVersion}`"
          :icon="Activity"
        />
        <MetricCard
          label="最近上報"
          :value="lastReportLabel"
          :note="`頁面刷新：${lastRefreshLabel}`"
          :icon="Clock3"
        />
      </section>

      <section class="content-grid">
        <div class="main-stack">
          <ClientTable
            v-bind="clientTableProps"
            :clients="clients"
            :selected-client-id="selectedStatus?.client_id ?? ''"
            :loading="loading"
            @update:search-text="updateClientSearch"
            @update:group-filter="updateClientGroupFilter"
            @update:tag-filter="updateClientTagFilter"
            @update:online-filter="updateClientOnlineFilter"
            @update:page-size="updateClientPageSize"
            @apply-filters="applyClientFilters"
            @page-change="changeClientPage"
            @select="selectedClientId = $event"
          />
          <SnapshotAnalytics
            :clients="clients"
            :online-count="onlineCount"
            :offline-count="offlineCount"
          />
          <HistoryTrendPanel
            :history="selectedHistory"
            :limit="historyLimit"
          />
        </div>
        <aside class="side-stack">
          <MetricCard
            label="DM 權限"
            :value="`${securityEnabledCount}/${clients.length}`"
            note="已上报 dm.access 的 Client 数量"
            :icon="ShieldCheck"
            tone="success"
          />
          <ClientDetail :status="selectedStatus" />
        </aside>
      </section>
    </template>

    <section v-else-if="activeView === 'clients'" class="content-grid">
      <div class="main-stack">
        <ClientTable
          v-bind="clientTableProps"
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @update:search-text="updateClientSearch"
          @update:group-filter="updateClientGroupFilter"
          @update:tag-filter="updateClientTagFilter"
          @update:online-filter="updateClientOnlineFilter"
          @update:page-size="updateClientPageSize"
          @apply-filters="applyClientFilters"
          @page-change="changeClientPage"
          @select="selectedClientId = $event"
        />
        <HistoryTrendPanel
          :history="selectedHistory"
          :limit="historyLimit"
        />
      </div>
      <aside class="side-stack">
        <MetricCard
          label="操作目標"
          :value="selectedClientTitle"
          :note="selectedRuntimeMode"
          :icon="ListChecks"
        />
        <ClientDetail :status="selectedStatus" />
      </aside>
    </section>

    <section v-else-if="activeView === 'scripts'" class="content-grid">
      <div class="main-stack">
        <DmLuaGuidePanel :status="selectedStatus" />
        <ScriptPanel :status="selectedStatus" />
        <ClientTable
          v-bind="clientTableProps"
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @update:search-text="updateClientSearch"
          @update:group-filter="updateClientGroupFilter"
          @update:tag-filter="updateClientTagFilter"
          @update:online-filter="updateClientOnlineFilter"
          @update:page-size="updateClientPageSize"
          @apply-filters="applyClientFilters"
          @page-change="changeClientPage"
          @select="selectedClientId = $event"
        />
      </div>
      <aside class="side-stack">
        <MetricCard
          label="腳本入口"
          :value="currentScript"
          note="選中 Client 当前脚本"
          :icon="FileCode2"
        />
        <MetricCard
          label="安全門"
          :value="`${securityEnabledCount}/${clients.length}`"
          note="开启安全门的 Client 数量"
          :icon="ShieldCheck"
          tone="success"
        />
      </aside>
    </section>

    <section v-else-if="activeView === 'operations'" class="content-grid">
      <div class="main-stack">
        <ClientRemoteActions
          :status="selectedStatus"
          :clients="clients"
          :server-url="serverUrl"
        />
        <ClientConfigApplyPanel
          :status="selectedStatus"
          :clients="clients"
          :server-url="serverUrl"
          @refresh="refreshDashboard"
        />
        <AuditPanel :server-url="serverUrl" />
        <ClientTable
          v-bind="clientTableProps"
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @update:search-text="updateClientSearch"
          @update:group-filter="updateClientGroupFilter"
          @update:tag-filter="updateClientTagFilter"
          @update:online-filter="updateClientOnlineFilter"
          @update:page-size="updateClientPageSize"
          @apply-filters="applyClientFilters"
          @page-change="changeClientPage"
          @select="selectedClientId = $event"
        />
      </div>
      <aside class="side-stack">
        <MetricCard
          label="遠程目標"
          :value="selectedClientTitle"
          :note="selectedRuntimeMode"
          :icon="Bolt"
        />
        <MetricCard
          label="最近上報"
          :value="lastReportLabel"
          :note="`頁面刷新：${lastRefreshLabel}`"
          :icon="Clock3"
        />
        <ClientDetail :status="selectedStatus" />
      </aside>
    </section>

    <section v-else-if="activeView === 'logs'" class="content-grid logs-view">
      <div class="main-stack">
        <AuditPanel :server-url="serverUrl" />
        <ClientRemoteActions
          :status="selectedStatus"
          :clients="clients"
          :server-url="serverUrl"
        />
      </div>
      <aside class="side-stack">
        <MetricCard
          label="日誌目標"
          :value="selectedClientTitle"
          :note="selectedStatusLabel"
          :icon="ScrollText"
        />
        <MetricCard
          label="最近上報"
          :value="lastReportLabel"
          :note="`頁面刷新：${lastRefreshLabel}`"
          :icon="Clock3"
        />
        <ClientDetail :status="selectedStatus" />
      </aside>
    </section>

    <section v-else class="content-grid">
      <div class="main-stack">
        <SetupWizardPanel
          v-model:server-url="serverUrl"
          v-model:client-id="clientId"
          @apply="refreshDashboard"
        />
        <ClientConfigApplyPanel
          :status="selectedStatus"
          :clients="clients"
          :server-url="serverUrl"
          @refresh="refreshDashboard"
        />
        <ClientSettingsPanel
          v-model:server-url="serverUrl"
          v-model:client-id="clientId"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @refresh="refreshDashboard"
        />
        <DmLuaGuidePanel :status="selectedStatus" />
      </div>
      <aside class="side-stack">
        <MetricCard
          label="Server"
          :value="healthLabel"
          :note="serverUrl"
          :icon="Server"
          :tone="health === 'online' ? 'success' : 'warning'"
        />
        <ClientDetail :status="selectedStatus" />
      </aside>
    </section>
  </AppShell>
</template>

<style scoped>
.topbar {
  position: sticky;
  top: 0;
  z-index: 8;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  margin: calc(var(--space-5) * -1) calc(var(--space-5) * -1) var(--space-5);
  border-bottom: 1px solid rgba(211, 220, 232, 0.92);
  background: rgba(242, 246, 250, 0.96);
  backdrop-filter: blur(18px);
  padding: var(--space-3) var(--space-5);
}

h1,
p {
  margin: 0;
}

h1 {
  color: var(--color-text);
  font-size: 24px;
  font-weight: 780;
  letter-spacing: 0;
  line-height: 1.15;
}

.topbar p {
  margin-top: var(--space-1);
  color: var(--color-muted);
  font-size: 13px;
  line-height: 1.45;
}

.topbar-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  align-items: center;
  gap: var(--space-2);
}

.target-card {
  display: grid;
  min-width: 300px;
  max-width: 440px;
  gap: 2px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-control);
  background: rgba(255, 255, 255, 0.82);
  padding: 8px var(--space-3);
}

.target-card strong {
  overflow: hidden;
  color: var(--color-text);
  font-size: 13px;
  font-weight: 820;
  line-height: 1.25;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.target-subtitle {
  overflow: hidden;
  color: var(--color-muted);
  font-size: 12px;
  line-height: 1.35;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.target-card[data-online="true"] {
  border-color: rgba(8, 127, 122, 0.28);
}

.topbar-actions button {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-control);
  background: var(--color-surface);
  color: var(--color-text);
  padding: 9px var(--space-3);
  font-size: 13px;
  font-weight: 760;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
  gap: var(--space-3);
  margin-bottom: var(--space-5);
}

.error-banner {
  margin-bottom: var(--space-5);
  border: 1px solid rgba(180, 35, 24, 0.25);
  border-radius: var(--radius-control);
  background: #fff1f0;
  color: var(--color-danger);
  padding: var(--space-3) var(--space-4);
  font-size: 13px;
  font-weight: 700;
}

.content-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(320px, 380px);
  align-items: start;
  gap: var(--space-4);
}

.main-stack,
.side-stack {
  display: grid;
  gap: var(--space-4);
}

.logs-view {
  grid-template-columns: minmax(0, 1fr) minmax(320px, 400px);
}

.spinning {
  animation: spin 0.9s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@media (max-width: 1380px) {
  .content-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .topbar {
    display: grid;
    margin: calc(var(--space-4) * -1) calc(var(--space-4) * -1) var(--space-4);
    padding: var(--space-3) var(--space-4);
  }

  .topbar-actions {
    justify-content: flex-start;
  }

  .target-card {
    min-width: 0;
    width: 100%;
  }

  h1 {
    font-size: 23px;
  }

  .metrics-grid {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
