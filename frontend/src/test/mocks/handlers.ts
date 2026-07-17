import { http, HttpResponse } from 'msw'
import type { HttpHandler } from 'msw'

// 業務APIは全リクエストの前提としてトークン取得（api/auth.ts）を行うため、
// 横断的関心事としてここに既定ハンドラーを登録する。
// それ以外の業務APIのモックは各テストが server.use() で都度登録する。
// 未登録のリクエストは setup.ts の onUnhandledRequest: 'error' で検知される。
export const handlers: HttpHandler[] = [
  http.post('/api/auth/token', () => HttpResponse.json({ token: 'test-token' })),
]
