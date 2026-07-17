// 業務APIはJWT認証ミドルウェアで保護されている（src/presentation/auth_middleware.rs）。
// このアプリはまだログイン機能を持たないシングルユーザー想定のため、起動時に固定の
// user_idでトークンを取得し、以降の全リクエストにAuthorizationヘッダーとして付与する。
const BASE_URL = '/api'
const APP_USER_ID = 'hoshika-app'

let tokenPromise: Promise<string> | null = null

async function fetchToken(): Promise<string> {
  const res = await fetch(`${BASE_URL}/auth/token`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ user_id: APP_USER_ID }),
  })

  if (!res.ok) {
    throw new Error(`認証トークンの取得に失敗しました（status ${res.status}）`)
  }

  const body = (await res.json()) as { token: string }
  return body.token
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
