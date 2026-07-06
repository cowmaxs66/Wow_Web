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
}

export interface HealthResponse {
  status: string;
}

export type ClientStatusEnvelope = WsEnvelope<ClientStatus>;

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
