<script setup lang="ts">
import {
  Activity,
  Clock3,
  MonitorCheck,
  RefreshCw,
  Server,
  ShieldCheck,
} from "@lucide/vue";
import AppShell from "./components/AppShell.vue";
import ClientDetail from "./components/ClientDetail.vue";
import ClientSettingsPanel from "./components/ClientSettingsPanel.vue";
import ClientTable from "./components/ClientTable.vue";
import MetricCard from "./components/MetricCard.vue";
import SnapshotAnalytics from "./components/SnapshotAnalytics.vue";
import StatusDot from "./components/StatusDot.vue";
import { useDashboardStatus } from "./composables/useDashboardStatus";

const {
  serverUrl,
  clientId,
  health,
  clients,
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
</script>

<template>
  <AppShell>
    <header class="topbar">
      <div>
        <h1>客戶端狀態</h1>
        <p>查看 Management Server 與本機 Client Agent 最新狀態。</p>
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
        note="只統計 Server 內存最新狀態"
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

    <p v-if="errorMessage" class="error-banner">{{ errorMessage }}</p>

    <section class="content-grid">
      <div class="main-stack">
        <SnapshotAnalytics
          :clients="clients"
          :online-count="onlineCount"
          :offline-count="offlineCount"
        />
        <ClientTable
          :clients="clients"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @select="selectedClientId = $event"
        />
      </div>
      <aside class="side-stack">
        <ClientSettingsPanel
          v-model:server-url="serverUrl"
          v-model:client-id="clientId"
          :selected-client-id="selectedStatus?.client_id ?? ''"
          :loading="loading"
          @refresh="refreshDashboard"
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
