import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import {
  ManagementServerError,
  fetchClientHistory,
  fetchClientStatus,
  fetchClientStatusPage,
  fetchClientStatuses,
  fetchHealth,
} from "../api/managementServer";
import {
  type AdminRealtimeConnection,
  connectAdminRealtime,
} from "../api/realtime";
import type { ClientStatusEnvelope } from "../types/protocol";
import { formatTimestamp } from "../types/protocol";

export function useDashboardStatus() {
  const serverUrl = ref(
    localStorage.getItem("wow-admin-server-url") ?? "http://127.0.0.1:18080",
  );
  const clientId = ref(
    localStorage.getItem("wow-admin-client-id") ?? "local-dev-client",
  );
  const health = ref<"unknown" | "online" | "offline">("unknown");
  const clients = ref<ClientStatusEnvelope[]>([]);
  const clientPage = ref(1);
  const clientPageSize = ref(25);
  const clientTotal = ref(0);
  const clientTotalPages = ref(0);
  const clientSearch = ref("");
  const clientGroupFilter = ref("");
  const clientTagFilter = ref("");
  const clientOnlineFilter = ref<"all" | "online" | "offline">("all");
  const selectedHistory = ref<ClientStatusEnvelope[]>([]);
  const historyLimit = ref(0);
  const selectedClientId = ref("");
  const loading = ref(false);
  const errorMessage = ref("");
  const lastRefreshAt = ref<number | null>(null);
  const realtimeConnected = ref(false);
  let realtimeConnection: AdminRealtimeConnection | null = null;
  let realtimeRefreshTimer: ReturnType<typeof setTimeout> | null = null;

  const selectedStatus = computed(() => {
    return (
      clients.value.find((client) => client.client_id === selectedClientId.value) ??
      clients.value[0] ??
      null
    );
  });

  const onlineCount = computed(() => {
    return clients.value.filter((client) => client.data.online).length;
  });

  const offlineCount = computed(() => {
    return Math.max(clients.value.length - onlineCount.value, 0);
  });

  const securityEnabledCount = computed(() => {
    return clients.value.filter((client) => client.data.script.security_enabled).length;
  });

  const currentScript = computed(() => {
    return selectedStatus.value?.data.current_script ?? "無";
  });

  const selectedReleaseVersion = computed(() => {
    return selectedStatus.value?.data.runtime.release_version ?? "無資料";
  });

  const healthLabel = computed(() => {
    if (health.value === "online") {
      return "Server 正常";
    }

    if (health.value === "offline") {
      return "Server 離線";
    }

    return "尚未檢查";
  });

  const lastReportLabel = computed(() => {
    if (!selectedStatus.value) {
      return "無資料";
    }

    return formatTimestamp(selectedStatus.value.timestamp_ms);
  });

  const lastRefreshLabel = computed(() => {
    if (!lastRefreshAt.value) {
      return "尚未刷新";
    }

    return formatTimestamp(lastRefreshAt.value);
  });

  async function refreshDashboard(): Promise<void> {
    loading.value = true;
    errorMessage.value = "";
    localStorage.setItem("wow-admin-server-url", serverUrl.value);
    localStorage.setItem("wow-admin-client-id", clientId.value);

    try {
      const healthResponse = await fetchHealth(serverUrl.value);
      health.value = healthResponse.status === "ok" ? "online" : "offline";

      // P4 优先读取列表 API；如果列表为空，再按输入 Client ID 查询一次。
      // 输入：Server URL 与 Client ID 表单值。
      // 输出：clients 列表、选中 Client、最近刷新时间。
      // 边界：404 表示该 Client 尚未上报，不制造假数据。
      const statusPage = await fetchStatusPageOrLegacy();
      clients.value = await resolveVisibleClients(statusPage.items);
      clientTotal.value = statusPage.total;
      clientTotalPages.value = statusPage.total_pages;
      clientPage.value = statusPage.page;
      selectedClientId.value =
        clients.value.find((client) => client.client_id === selectedClientId.value)
          ?.client_id ??
        clients.value[0]?.client_id ??
        "";
      await refreshSelectedHistory();
      lastRefreshAt.value = Date.now();
    } catch (error) {
      health.value = "offline";
      clients.value = [];
      clientTotal.value = 0;
      clientTotalPages.value = 0;
      selectedHistory.value = [];
      historyLimit.value = 0;
      selectedClientId.value = "";
      errorMessage.value =
        error instanceof Error ? error.message : `未知錯誤：${String(error)}`;
    } finally {
      loading.value = false;
    }
  }

  async function fetchStatusPageOrLegacy() {
    try {
      return await fetchClientStatusPage(serverUrl.value, {
        page: clientPage.value,
        pageSize: clientPageSize.value,
        group: clientGroupFilter.value,
        tag: clientTagFilter.value,
        online: onlineFilterValue(),
        search: clientSearch.value,
      });
    } catch (error) {
      if (error instanceof ManagementServerError && error.status === 404) {
        const items = await fetchClientStatuses(serverUrl.value);
        return {
          page: 1,
          page_size: items.length || clientPageSize.value,
          total: items.length,
          total_pages: items.length > 0 ? 1 : 0,
          items,
        };
      }

      throw error;
    }
  }

  function onlineFilterValue(): boolean | null {
    if (clientOnlineFilter.value === "online") {
      return true;
    }
    if (clientOnlineFilter.value === "offline") {
      return false;
    }
    return null;
  }

  async function resolveVisibleClients(
    statusList: ClientStatusEnvelope[],
  ): Promise<ClientStatusEnvelope[]> {
    if (statusList.length > 0 || !clientId.value.trim() || hasActiveClientQuery()) {
      return statusList;
    }

    try {
      return [await fetchClientStatus(serverUrl.value, clientId.value.trim())];
    } catch (error) {
      if (error instanceof ManagementServerError && error.status === 404) {
        return [];
      }

      throw error;
    }
  }

  function hasActiveClientQuery(): boolean {
    return (
      !!clientSearch.value.trim() ||
      !!clientGroupFilter.value.trim() ||
      !!clientTagFilter.value.trim() ||
      clientOnlineFilter.value !== "all"
    );
  }

  async function refreshSelectedHistory(): Promise<void> {
    if (!selectedClientId.value) {
      selectedHistory.value = [];
      historyLimit.value = 0;
      return;
    }

    // 历史查询只读取 Server 已保存的真实状态队列。
    // 输入：当前选中的 Client ID。
    // 输出：按 Server 返回顺序保存的历史样本。
    // 边界：404 当前不会出现；空历史按空数组展示，不补假点。
    const history = await fetchClientHistory(serverUrl.value, selectedClientId.value);
    selectedHistory.value = history.items;
    historyLimit.value = history.limit;
  }

  function startAdminRealtime(): void {
    stopAdminRealtime();

    try {
      realtimeConnection = connectAdminRealtime(serverUrl.value, {
        onOpen: () => {
          realtimeConnected.value = true;
        },
        onClose: () => {
          realtimeConnected.value = false;
        },
        onEvent: () => {
          scheduleRealtimeRefresh();
        },
      });
    } catch {
      realtimeConnected.value = false;
    }
  }

  function stopAdminRealtime(): void {
    realtimeConnection?.close();
    realtimeConnection = null;
    realtimeConnected.value = false;
  }

  function scheduleRealtimeRefresh(): void {
    if (realtimeRefreshTimer) {
      return;
    }

    // WS 事件只说明 Server 数据有变化；实际列表仍走原 HTTP 查询，保留分页、筛选和离线判定。
    // 输入：/ws/admin 的状态、命令、回执事件。
    // 输出：300ms 内合并成一次 dashboard 刷新。
    // 边界：Server 离线或 WS 断开时，用户仍可手动刷新走 HTTP。
    realtimeRefreshTimer = setTimeout(() => {
      realtimeRefreshTimer = null;
      void refreshDashboard();
    }, 300);
  }

  onMounted(() => {
    void refreshDashboard();
    startAdminRealtime();
  });

  onBeforeUnmount(() => {
    if (realtimeRefreshTimer) {
      clearTimeout(realtimeRefreshTimer);
      realtimeRefreshTimer = null;
    }
    stopAdminRealtime();
  });

  watch(serverUrl, () => {
    startAdminRealtime();
  });

  watch(selectedClientId, () => {
    if (!loading.value) {
      void refreshSelectedHistory();
    }
  });

  return {
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
    realtimeConnected,
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
  };
}
