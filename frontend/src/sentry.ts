import * as Sentry from '@sentry/react'

// VITE_SENTRY_DSN が未設定のローカル/CIでは何もしない。
// 初期化しなくても Sentry.captureException 等はno-opになるため、呼び出し側で分岐は不要。
// パフォーマンス計測（tracesSampleRate等）は別タスクのスコープのため含めない。
export function initSentry(): void {
  const dsn = import.meta.env.VITE_SENTRY_DSN
  if (!dsn) return

  Sentry.init({
    dsn,
    environment: import.meta.env.MODE,
  })
}
