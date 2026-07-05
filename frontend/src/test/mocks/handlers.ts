import type { HttpHandler } from 'msw'

// 各テストが server.use() で必要なハンドラーを都度登録するため、既定値は空にしている。
// 未登録のリクエストは setup.ts の onUnhandledRequest: 'error' で検知される。
export const handlers: HttpHandler[] = []
