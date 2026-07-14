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

  // 認証情報をコマンドライン引数に渡すと `ps` 等から他プロセスに見えてしまうため、
  // 接続情報はPG*環境変数経由でpsqlに渡す（引数にはURLを含めない）。
  const url = new URL(databaseUrl)

  const env = {
    ...process.env,
    PGHOST: url.hostname,
    PGPORT: url.port || '5432',
    PGUSER: decodeURIComponent(url.username),
    PGPASSWORD: decodeURIComponent(url.password),
    PGDATABASE: url.pathname.replace(/^\//, ''),
  }

  // purchase_recordsがwish_itemsを外部キー参照しているため、先に子テーブルを削除する。
  execFileSync(
    'psql',
    [
      '-v',
      'ON_ERROR_STOP=1',
      '-c',
      "DELETE FROM purchase_records WHERE wish_item_id IN (SELECT id FROM wish_items WHERE name LIKE 'E2E%');",
    ],
    { stdio: 'inherit', env },
  )

  execFileSync('psql', ['-v', 'ON_ERROR_STOP=1', '-c', "DELETE FROM wish_items WHERE name LIKE 'E2E%';"], {
    stdio: 'inherit',
    env,
  })
}
