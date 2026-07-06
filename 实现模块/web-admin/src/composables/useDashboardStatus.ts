import { computed, onMounted, ref } from "vue";
import {
  ManagementServerError,
  fetchClientStatus,
  fetchClientStatuses,
  fetchHealth,
} from "../api/managementServer";
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
  const selectedClientId = ref("");
  const loading = ref(false);
  const errorMessage = ref("");
  const lastRefreshAt = ref<number | null>(null);

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

  const currentScript = computed(() => {
    return selectedStatus.value?.data.current_script ?? "無";
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
      const statusList = await fetchClientStatuses(serverUrl.value);
      clients.value = await resolveVisibleClients(statusList);
      selectedClientId.value =
        clients.value.find((client) => client.client_id === selectedClientId.value)
          ?.client_id ??
        clients.value[0]?.client_id ??
        "";
      lastRefreshAt.value = Date.now();
    } catch (error) {
      health.value = "offline";
      clients.value = [];
      selectedClientId.value = "";
      errorMessage.value =
        error instanceof Error ? error.message : `未知錯誤：${String(error)}`;
    } finally {
      loading.value = false;
    }
  }

  async function resolveVisibleClients(
    statusList: ClientStatusEnvelope[],
  ): Promise<ClientStatusEnvelope[]> {
    if (statusList.length > 0 || !clientId.value.trim()) {
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

  onMounted(() => {
    void refreshDashboard();
  });

  return {
    serverUrl,
    clientId,
    health,
    clients,
    selectedClientId,
    loading,
    errorMessage,
    selectedStatus,
    onlineCount,
    currentScript,
    healthLabel,
    lastReportLabel,
    lastRefreshLabel,
    refreshDashboard,
  };
}
