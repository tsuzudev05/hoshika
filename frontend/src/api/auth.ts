// 業務APIはJWT認証ミドルウェアで保護されている（src/presentation/auth_middleware.rs）。
// このアプリはまだログイン機能を持たないシングルユーザー想定のため、起動時に固定の
// user_idでトークンを取得し、以降の全リクエストにAuthorizationヘッダーとして付与する。
const BASE_URL = '/api'
const APP_USER_ID = 'hoshika-app'

// リトライ対象は「一時的な失敗」に限定する。4xx（不正なリクエストなど）は
// 再試行しても結果が変わらないため即座に諦める。
const MAX_RETRIES = 2
const RETRY_DELAY_MS = 500

export class TokenFetchError extends Error {
  // status 0 はネットワークエラー（サーバーに到達できなかった場合）を表す。
  constructor(
    public readonly status: number,
    message: string,
  ) {
    super(message)
    this.name = 'TokenFetchError'
  }

  get isRetryable(): boolean {
    return this.status === 0 || (this.status >= 500 && this.status < 600)
  }
}

let tokenPromise: Promise<string> | null = null

async function fetchTokenOnce(): Promise<string> {
  let res: Response
  try {
    res = await fetch(`${BASE_URL}/auth/token`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_id: APP_USER_ID }),
    })
  } catch {
    throw new TokenFetchError(
      0,
      'サーバーに接続できませんでした。ネットワーク接続を確認してください。',
    )
  }

  if (!res.ok) {
    throw new TokenFetchError(
      res.status,
      `認証トークンの取得に失敗しました（status ${res.status}）`,
    )
  }

  const body = (await res.json()) as { token: string }
  return body.token
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

async function fetchToken(): Promise<string> {
  for (let attempt = 0; ; attempt++) {
    try {
      return await fetchTokenOnce()
    } catch (err) {
      const canRetry = err instanceof TokenFetchError && err.isRetryable && attempt < MAX_RETRIES
      if (!canRetry) {
        throw err
      }
      // 指数バックオフ（500ms, 1000ms, ...）
      await sleep(RETRY_DELAY_MS * 2 ** attempt)
    }
  }
}

export function getAuthToken(): Promise<string> {
  if (!tokenPromise) {
    tokenPromise = fetchToken().catch((err: unknown) => {
      // 失敗時は次回リクエストで再取得できるようキャッシュをクリアする
      tokenPromise = null
      throw err
    })
  }
  return tokenPromise
}
