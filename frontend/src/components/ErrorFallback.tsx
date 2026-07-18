import './ErrorFallback.css'

// アプリ全体を包む Sentry.ErrorBoundary のfallback。
// 個別コンポーネントの想定内エラー（APIエラー等）はここまで到達しない。
// ここに来るのはレンダリング中の予期しない例外のみ。
export function ErrorFallback() {
  return (
    <div className="error-fallback" role="alert">
      <p className="error-fallback__message">予期しないエラーが発生しました。</p>
      <button type="button" onClick={() => window.location.reload()}>
        再読み込み
      </button>
    </div>
  )
}
