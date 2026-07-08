import type { AdminRealtimeMessage } from "../types/protocol";

export interface AdminRealtimeHandlers {
  onEvent: (message: AdminRealtimeMessage) => void;
  onOpen?: () => void;
  onClose?: () => void;
}

export interface AdminRealtimeConnection {
  close: () => void;
}

const RECONNECT_DELAY_MS = 2000;

export function connectAdminRealtime(
  baseUrl: string,
  handlers: AdminRealtimeHandlers,
): AdminRealtimeConnection {
  const wsUrl = toAdminWsUrl(baseUrl);
  let closedByCaller = false;
  let socket: WebSocket | null = null;
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

  function connect(): void {
    if (closedByCaller) {
      return;
    }

    socket = new WebSocket(wsUrl);
    socket.onopen = () => handlers.onOpen?.();
    socket.onmessage = (event) => {
      const parsed = parseRealtimeMessage(event.data);
      if (parsed) {
        handlers.onEvent(parsed);
      }
    };
    socket.onclose = () => {
      handlers.onClose?.();
      scheduleReconnect();
    };
    socket.onerror = () => {
      socket?.close();
    };
  }

  function scheduleReconnect(): void {
    if (closedByCaller || reconnectTimer) {
      return;
    }

    reconnectTimer = setTimeout(() => {
      reconnectTimer = null;
      connect();
    }, RECONNECT_DELAY_MS);
  }

  connect();

  return {
    close: () => {
      closedByCaller = true;
      if (reconnectTimer) {
        clearTimeout(reconnectTimer);
        reconnectTimer = null;
      }
      socket?.close();
      socket = null;
    },
  };
}

export function toAdminWsUrl(baseUrl: string): string {
  const url = new URL(baseUrl.trim().replace(/\/+$/, ""));
  if (url.protocol === "https:") {
    url.protocol = "wss:";
  } else if (url.protocol === "http:") {
    url.protocol = "ws:";
  } else {
    throw new Error("Server 地址必须以 http:// 或 https:// 开头");
  }

  url.pathname = "/ws/admin";
  url.search = "";
  url.hash = "";
  return url.toString();
}

function parseRealtimeMessage(data: unknown): AdminRealtimeMessage | null {
  if (typeof data !== "string") {
    return null;
  }

  try {
    return JSON.parse(data) as AdminRealtimeMessage;
  } catch {
    // 实时通道只做刷新提示，解析失败时丢弃该帧，不阻断 HTTP 主流程。
    return null;
  }
}
