import { execFileSync } from 'node:child_process'

// e2eテストは実行のたびに `E2E` プレフィックス付きの名前でwish_itemsを作成する。
// クリーンアップ用のDELETE APIがまだ存在しないため、DBへ直接接続して後片付けする。
export default function globalTeardown() {
  const databaseUrl = process.env.DATABASE_URL
  if (!databaseUrl) {
    console.warn(
      '[e2e] DATABASE_URL が未設定のため、テストで作成したデータのクリーンアップをスキップしました。',
    )
    return
  }

  execFileSync('psql', [databaseUrl, '-v', 'ON_ERROR_STOP=1', '-c', "DELETE FROM wish_items WHERE name LIKE 'E2E%';"], {
    stdio: 'inherit',
  })
}
