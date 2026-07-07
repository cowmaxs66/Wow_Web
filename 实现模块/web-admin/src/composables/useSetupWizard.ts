import { computed, ref, watch } from "vue";

const STORAGE_KEY = "wow-admin-setup-profile-v1";

export type ClientMode = "x64-core" | "x86-dm";

export interface SetupProfile {
  serverHost: string;
  serverPort: string;
  historyPath: string;
  webDir: string;
  clientId: string;
  reportEnabled: boolean;
  clientMode: ClientMode;
  dmBridgePath: string;
  completedAt: string;
}

const defaultProfile: SetupProfile = {
  serverHost: "127.0.0.1",
  serverPort: "18080",
  historyPath: "data/status-history.jsonl",
  webDir: "web-admin/dist",
  clientId: "local-dev-client",
  reportEnabled: true,
  clientMode: "x64-core",
  dmBridgePath: "dm-bridge/Win32/DmBridge.dll",
  completedAt: "",
};

export function useSetupWizard() {
  const profile = ref(loadProfile());
  const copiedTarget = ref("");

  watch(
    profile,
    (value) => {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
    },
    { deep: true },
  );

  const normalizedPort = computed(() => {
    return fallbackText(profile.value.serverPort, defaultProfile.serverPort);
  });

  const browserHost = computed(() => {
    const host = fallbackText(profile.value.serverHost, defaultProfile.serverHost);

    // 0.0.0.0 是服务端监听地址，浏览器不能把它当成目标主机访问。
    // 输入：向导里的 Server Host。
    // 输出：Web 控制台实际 fetch 使用的地址。
    // 边界：只有通配监听地址会转成本机地址，局域网 IP 与域名保持用户输入。
    if (host === "0.0.0.0" || host === "::") {
      return "127.0.0.1";
    }

    return host;
  });

  const serverUrl = computed(() => {
    return `http://${browserHost.value}:${normalizedPort.value}`;
  });

  const isCompleted = computed(() => {
    return profile.value.completedAt.trim().length > 0;
  });

  const architectureLabel = computed(() => {
    if (profile.value.clientMode === "x86-dm") {
      return "x86 DM 模式";
    }

    return "x64 核心模式";
  });

  const architectureNote = computed(() => {
    if (profile.value.clientMode === "x86-dm") {
      return "用于 32 位大漠与 Win32 DmBridge。Server 仍可保持 x64。";
    }

    return "用于 Server/Web/基础 Client，不直接加载 32 位大漠 DLL。";
  });

  const serverCommand = computed(() => {
    // 向导命令只用于复制执行，不直接运行用户输入。
    // 所有可编辑字段统一按 PowerShell 单引号转义，避免路径空格或单引号破坏命令结构。
    return [
      ".\\tools\\start-server.ps1",
      "-HostAddress",
      quotePowerShellValue(fallbackText(profile.value.serverHost, defaultProfile.serverHost)),
      "-Port",
      normalizedPort.value,
      "-HistoryPath",
      quotePowerShellValue(fallbackText(profile.value.historyPath, defaultProfile.historyPath)),
      "-OpenBrowser",
    ].join(" ");
  });

  const clientCommand = computed(() => {
    return clientToolCommand("-Monitor");
  });

  const startupStatusCommand = computed(() => {
    return clientToolCommand("-StartupStatus");
  });

  const enableStartupCommand = computed(() => {
    return clientToolCommand("-EnableStartup");
  });

  const disableStartupCommand = computed(() => {
    return clientToolCommand("-DisableStartup");
  });

  function applyDashboardDefaults(serverUrlValue: string, clientIdValue: string): void {
    let url: URL;

    try {
      url = new URL(serverUrlValue);
    } catch {
      return;
    }

    profile.value.serverHost = url.hostname;
    profile.value.serverPort = url.port || defaultProfile.serverPort;
    profile.value.clientId = clientIdValue.trim() || defaultProfile.clientId;
  }

  function markCompleted(): void {
    profile.value.completedAt = new Date().toISOString();
  }

  function reset(): void {
    profile.value = { ...defaultProfile };
  }

  async function copyText(target: string, text: string): Promise<void> {
    await navigator.clipboard.writeText(text);
    copiedTarget.value = target;
    window.setTimeout(() => {
      if (copiedTarget.value === target) {
        copiedTarget.value = "";
      }
    }, 1600);
  }

  function clientToolCommand(action: string): string {
    const arch = profile.value.clientMode === "x86-dm" ? "x86" : "x64";
    const args = [
      ".\\tools\\start-client.ps1",
      "-ClientArch",
      quotePowerShellValue(arch),
      "-ServerHost",
      quotePowerShellValue(browserHost.value),
      "-ServerPort",
      normalizedPort.value,
    ];

    if (!profile.value.reportEnabled) {
      args.push("-DisableReport");
    }

    if (profile.value.clientMode === "x86-dm") {
      args.push(
        "-DmBridgePath",
        quotePowerShellValue(
          fallbackText(profile.value.dmBridgePath, defaultProfile.dmBridgePath),
        ),
      );
    }

    args.push(action);
    return args.join(" ");
  }

  return {
    profile,
    copiedTarget,
    serverUrl,
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
  };
}

function loadProfile(): SetupProfile {
  const raw = localStorage.getItem(STORAGE_KEY);

  if (!raw) {
    return { ...defaultProfile };
  }

  try {
    return { ...defaultProfile, ...JSON.parse(raw) };
  } catch {
    return { ...defaultProfile };
  }
}

function fallbackText(value: string, fallback: string): string {
  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : fallback;
}

function quotePowerShellValue(value: string): string {
  return `'${value.replaceAll("'", "''")}'`;
}
