// fetchの直接呼び出しをコンポーネントから隠蔽する薄いクライアント。
// vite.config.ts の proxy 設定により /api は Axum サーバー（:3000）へ転送される。

const BASE_URL = '/api'

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE_URL}${path}`, {
    ...init,
    headers: {
      'Content-Type': 'application/json',
      ...init?.headers,
    },
  })

  const body: unknown = await res.json().catch(() => null)

  if (!res.ok) {
    const message =
      (body as { error?: string } | null)?.error ?? `request failed with status ${res.status}`
    throw new ApiError(res.status, message)
  }

  return body as T
}

export const apiClient = {
  get: <T>(path: string) => request<T>(path),
  post: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: 'POST', body: body === undefined ? undefined : JSON.stringify(body) }),
}
