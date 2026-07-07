<script setup lang="ts">
import {
  Activity,
  Bolt,
  Clock3,
  FileCode2,
  MonitorCheck,
  RefreshCw,
  Server,
  ShieldCheck,
} from "@lucide/vue";
import { computed, ref } from "vue";
import AppShell from "./components/AppShell.vue";
import ClientDetail from "./components/ClientDetail.vue";
import ClientRemoteActions from "./components/ClientRemoteActions.vue";
import ClientSettingsPanel from "./components/ClientSettingsPanel.vue";
import ClientTable from "./components/ClientTable.vue";
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

type AdminView = "overview" | "clients" | "scripts" | "operations" | "settings";

const activeView = ref<AdminView>("overview");
const allowedViews = new Set<AdminView>([
  "overview",
  "clients",
  "scripts",
  "operations",
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
        description: "檢查 Client Agent 上報的 Lua 宿主、安全門與權限配置。",
      };
    case "operations":
      return {
        title: "遠程操作",
        description: "由 Server 寫入命令隊列，Client monitor 輪詢後在本機執行。",
      };
    case "settings":
      return {
        title: "設定",
        description: "管理 Web 端連線設定與首次部署向導。",
      };
    default:
      return {
        title: "總覽",
        description: "查看 Management Server 與 Client Agent 的整體運行狀態。",
      };
  }
});

function changeView(view: string): void {
  if (allowedViews.has(view as AdminView)) {
    activeView.value = view as AdminView;
  }
}
</script>

<template>
  <AppShell :active-view="activeView" @navigate="changeView">
    <header class="topbar">
      <div>
        <h1>{{ viewMeta.title }}</h1>
        <p>{{ viewMeta.description }}</p>
      </div>
      <div class="topbar-actions">
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
          label="當前腳本"
          :value="currentScript"
          note="選中 Client 的 current_script"
          :icon="Activity"
        />
        <MetricCard
          label="腳本安全門"
          :value="`${securityEnabledCount}/${clients.length}`"
          :note="`Agent 版本：${selectedReleaseVersion}`"
          :icon="ShieldCheck"
          tone="success"
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
          <SnapshotAnalytics
            :clients="clients"
            :online-count="onlineCount"
            :offline-count="offlineCount"
          />
          <HistoryTrendPanel
            :history="selectedHistory"
            :limit="historyLimit"
          />
          <ClientTable
            :clients="clients"
            :selected-client-id="selectedStatus?.client_id ?? ''"
            :loading="loading"
            @select="selectedClientId = $event"
          />
        </div>
        <aside class="side-stack">
          <ClientDetail :status="selectedStatus" />
        </aside>
      </section>
    </template>

    <section v-else-if="activeView === 'clients'" class="content-grid">
      <div class="main-stack">
        <ClientTable
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @select="selectedClientId = $event"
        />
        <HistoryTrendPanel
          :history="selectedHistory"
          :limit="historyLimit"
        />
      </div>
      <aside class="side-stack">
        <ClientDetail :status="selectedStatus" />
      </aside>
    </section>

    <section v-else-if="activeView === 'scripts'" class="content-grid">
      <div class="main-stack">
        <ScriptPanel :status="selectedStatus" />
        <ClientTable
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
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
        <ClientTable
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @select="selectedClientId = $event"
        />
      </div>
      <aside class="side-stack">
        <MetricCard
          label="遠程目標"
          :value="selectedStatus?.client_id ?? '未選擇'"
          note="操作面板可选择单台或全部 Client"
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

    <section v-else class="content-grid">
      <div class="main-stack">
        <SetupWizardPanel
          v-model:server-url="serverUrl"
          v-model:client-id="clientId"
          @apply="refreshDashboard"
        />
        <ClientSettingsPanel
          v-model:server-url="serverUrl"
          v-model:client-id="clientId"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @refresh="refreshDashboard"
        />
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
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

h1,
p {
  margin: 0;
}

h1 {
  color: var(--color-text);
  font-size: 30px;
  font-weight: 780;
  letter-spacing: 0;
  line-height: 1.15;
}

.topbar p {
  margin-top: var(--space-2);
  color: var(--color-muted);
  font-size: 14px;
  line-height: 1.6;
}

.topbar-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  align-items: center;
  gap: var(--space-3);
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
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: var(--space-4);
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
  grid-template-columns: minmax(0, 1fr) 340px;
  align-items: start;
  gap: var(--space-5);
}

.main-stack,
.side-stack {
  display: grid;
  gap: var(--space-5);
}

.spinning {
  animation: spin 0.9s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@media (max-width: 1180px) {
  .content-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .topbar {
    display: grid;
  }

  .topbar-actions {
    justify-content: flex-start;
  }

  h1 {
    font-size: 25px;
  }

  .metrics-grid {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>
