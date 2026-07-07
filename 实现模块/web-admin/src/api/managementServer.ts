import type {
  ClientMessage,
  ClientMessageRequest,
  ClientStatusHistory,
  ClientStatusEnvelope,
  HealthResponse,
} from "../types/protocol";

export class ManagementServerError extends Error {
  readonly status?: number;

  constructor(message: string, status?: number) {
    super(message);
    this.name = "ManagementServerError";
    this.status = status;
  }
}

export async function fetchHealth(baseUrl: string): Promise<HealthResponse> {
  return readJson<HealthResponse>(baseUrl, "/health");
}

export async function fetchClientStatuses(
  baseUrl: string,
): Promise<ClientStatusEnvelope[]> {
  return readJson<ClientStatusEnvelope[]>(baseUrl, "/api/client/status");
}

export async function fetchClientStatus(
  baseUrl: string,
  clientId: string,
): Promise<ClientStatusEnvelope> {
  return readJson<ClientStatusEnvelope>(
    baseUrl,
    `/api/client/status/${encodeURIComponent(clientId)}`,
  );
}

export async function fetchClientHistory(
  baseUrl: string,
  clientId: string,
): Promise<ClientStatusHistory> {
  return readJson<ClientStatusHistory>(
    baseUrl,
    `/api/client/history/${encodeURIComponent(clientId)}`,
  );
}

export async function sendClientMessage(
  baseUrl: string,
  clientId: string,
  request: ClientMessageRequest,
): Promise<ClientMessage> {
  return writeJson<ClientMessage>(
    baseUrl,
    `/api/client/messages/${encodeURIComponent(clientId)}`,
    request,
  );
}

function normalizeBaseUrl(baseUrl: string): string {
  const normalized = baseUrl.trim().replace(/\/+$/, "");

  // Server 地址是用户可编辑输入，进入 fetch 前先做最小校验。
  // 输入：表单中的 Server URL。
  // 输出：无结尾斜线的 HTTP(S) base URL。
  // 边界：空值或非法 URL 直接抛错，不拼接成隐藏的相对路径。
  if (!normalized) {
    throw new ManagementServerError("Server 地址不能为空");
  }

  const url = new URL(normalized);
  if (url.protocol !== "http:" && url.protocol !== "https:") {
    throw new ManagementServerError("Server 地址必须以 http:// 或 https:// 开头");
  }

  return normalized;
}

async function readJson<T>(baseUrl: string, path: string): Promise<T> {
  const url = `${normalizeBaseUrl(baseUrl)}${path}`;
  let response: Response;

  try {
    response = await fetch(url, {
      headers: { Accept: "application/json" },
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new ManagementServerError(`连接 Management Server 失败：${message}`);
  }

  if (!response.ok) {
    throw new ManagementServerError(
      `Management Server 返回 HTTP ${response.status}`,
      response.status,
    );
  }

  try {
    return (await response.json()) as T;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new ManagementServerError(`解析 Server JSON 失败：${message}`);
  }
}

async function writeJson<T>(baseUrl: string, path: string, body: unknown): Promise<T> {
  const url = `${normalizeBaseUrl(baseUrl)}${path}`;
  let response: Response;

  try {
    response = await fetch(url, {
      method: "POST",
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body),
    });
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new ManagementServerError(`连接 Management Server 失败：${message}`);
  }

  if (!response.ok) {
    throw new ManagementServerError(
      `Management Server 返回 HTTP ${response.status}`,
      response.status,
    );
  }

  try {
    return (await response.json()) as T;
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new ManagementServerError(`解析 Server JSON 失败：${message}`);
  }
}
