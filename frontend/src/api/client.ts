// fetchの直接呼び出しをコンポーネントから隠蔽する薄いクライアント。
// vite.config.ts の proxy 設定により /api は Axum サーバー（:3000）へ転送される。

import * as Sentry from '@sentry/react'
import { getAuthToken } from './auth'

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

// ネットワーク断（status 0）・サーバー内部エラー（5xx）・レスポンス契約違反は、
// ユーザーの入力ミスではなくインフラ/実装側の予期しない問題である可能性が高いためSentryに送る。
// 4xx（バリデーションエラー・未認証・未検出等）はアプリ層で想定済みの挙動のため送らない。
function reportIfUnexpected(error: ApiError): ApiError {
  if (error.status === 0 || error.status >= 500) {
    Sentry.captureException(error)
  }
  return error
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  let res: Response
  try {
    const token = await getAuthToken()
    res = await fetch(`${BASE_URL}${path}`, {
      ...init,
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
        ...init?.headers,
      },
    })
  } catch {
    throw reportIfUnexpected(
      new ApiError(0, 'サーバーに接続できませんでした。ネットワーク接続を確認してください。'),
    )
  }

  const body: unknown = await res.json().catch(() => null)

  if (!res.ok) {
    const message =
      (body as { error?: string } | null)?.error ?? `request failed with status ${res.status}`
    throw reportIfUnexpected(new ApiError(res.status, message))
  }

  if (body === null) {
    // res.okなのにbodyが解析できないのはサーバー側の契約違反であり、常に予期しないエラーとして送る。
    const error = new ApiError(res.status, 'サーバーから予期しない形式のレスポンスが返されました。')
    Sentry.captureException(error)
    throw error
  }

  return body as T
}

export const apiClient = {
  get: <T>(path: string) => request<T>(path),
  post: <T>(path: string, body?: unknown) =>
    request<T>(path, { method: 'POST', body: body === undefined ? undefined : JSON.stringify(body) }),
}
