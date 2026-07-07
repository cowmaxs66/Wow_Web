export type MessageType = "status";

export interface WsEnvelope<T> {
  schema_version: number;
  message_id: string;
  message_type: MessageType;
  client_id: string;
  timestamp_ms: number;
  data: T;
}

export interface ClientStatus {
  client_id: string;
  online: boolean;
  current_script: string | null;
  runtime: ClientRuntimeInfo;
  script: ClientScriptInfo;
  server: ClientServerInfo;
}

export interface ClientRuntimeInfo {
  release_version: string;
  os: string;
  arch: string;
  process_id: number;
}

export interface ClientScriptInfo {
  bootstrap_name: string;
  instruction_limit: number;
  security_enabled: boolean;
  allowed_permissions: string[];
}

export interface ClientServerInfo {
  report_enabled: boolean;
  report_target: string;
}

export interface HealthResponse {
  status: string;
}

export type ClientStatusEnvelope = WsEnvelope<ClientStatus>;

export interface ClientStatusHistory {
  client_id: string;
  limit: number;
  total: number;
  items: ClientStatusEnvelope[];
}

export interface ClientMessageRequest {
  title: string;
  body: string;
}

export interface ClientMessage {
  id: string;
  client_id: string;
  timestamp_ms: number;
  title: string;
  body: string;
}

export interface ClientMessageList {
  client_id: string;
  total: number;
  items: ClientMessage[];
}

export type ClientCommandType =
  | "script.run_bootstrap"
  | "startup.status"
  | "startup.enable"
  | "startup.disable"
  | "service.status"
  | "service.install"
  | "service.start"
  | "service.stop"
  | "update.check"
  | "update.download"
  | "update.apply"
  | "config.apply"
  | "settings.open"
  | "log.open"
  | "tray.open";

export interface ClientConfigPatch {
  lua?: ClientLuaConfigPatch;
  script_security?: ClientScriptSecurityConfigPatch;
  dm?: ClientDmConfigPatch;
  server?: ClientServerConfigPatch;
}

export interface ClientLuaConfigPatch {
  bootstrap_name?: string;
  bootstrap_path?: string;
  instruction_limit?: number;
}

export interface ClientScriptSecurityConfigPatch {
  enabled?: boolean;
  manifest_path?: string;
  trusted_signer_public_key?: string;
  allowed_permissions?: string[];
}

export interface ClientDmConfigPatch {
  bridge_path?: string;
}

export interface ClientServerConfigPatch {
  enabled?: boolean;
  host?: string;
  port?: number;
  status_path?: string;
  connect_timeout_ms?: number;
}

export interface ClientCommandRequest {
  command_type: ClientCommandType;
  payload: Record<string, unknown> | ClientConfigPatch;
}

export interface ClientCommand {
  id: string;
  client_id: string;
  timestamp_ms: number;
  command_type: ClientCommandType;
  payload: Record<string, unknown> | ClientConfigPatch;
}

export interface ClientCommandList {
  client_id: string;
  total: number;
  items: ClientCommand[];
}

export interface ClientCommandReceiptRequest {
  command_id: string;
  command_type: ClientCommandType;
  success: boolean;
  summary: string;
}

export interface ClientCommandReceipt {
  id: string;
  client_id: string;
  timestamp_ms: number;
  command_id: string;
  command_type: ClientCommandType;
  success: boolean;
  summary: string;
}

export interface ClientCommandReceiptList {
  client_id: string;
  total: number;
  items: ClientCommandReceipt[];
}

export function formatTimestamp(timestampMs: number): string {
  if (!Number.isFinite(timestampMs) || timestampMs <= 0) {
    return "無資料";
  }

  return new Intl.DateTimeFormat("zh-Hant", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  }).format(new Date(timestampMs));
}

export function formatFullTimestamp(timestampMs: number): string {
  if (!Number.isFinite(timestampMs) || timestampMs <= 0) {
    return "無資料";
  }

  return new Intl.DateTimeFormat("zh-Hant", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  }).format(new Date(timestampMs));
}

export function formatRelativeAge(timestampMs: number, nowMs = Date.now()): string {
  if (!Number.isFinite(timestampMs) || timestampMs <= 0) {
    return "無資料";
  }

  const diffMs = nowMs - timestampMs;
  if (diffMs < 0) {
    return "本機時間早於上報時間";
  }

  // 快照分析只描述当前 Server 内存中的最后一次上报年龄。
  // 输入：状态消息 timestamp_ms 与浏览器当前时间。
  // 输出：便于扫描的相对时间标签。
  // 边界：这里不是历史趋势统计，不推断 Client 是否长期在线。
  const seconds = Math.floor(diffMs / 1000);
  if (seconds < 60) {
    return `${seconds} 秒前`;
  }

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) {
    return `${minutes} 分鐘前`;
  }

  const hours = Math.floor(minutes / 60);
  if (hours < 24) {
    return `${hours} 小時前`;
  }

  const days = Math.floor(hours / 24);
  return `${days} 天前`;
}
