import { execFileSync } from 'node:child_process'

// 認証情報をコマンドライン引数に渡すと`ps`等から他プロセスに見えてしまうため、
// パスワードだけURLから外し、PGPASSWORD環境変数経由でpsqlに渡す。
function connect() {
  const databaseUrl = process.env.DATABASE_URL
  if (!databaseUrl) {
    console.warn('[e2e] DATABASE_URL が未設定のため、DB操作をスキップしました。')
    return undefined
  }

  const url = new URL(databaseUrl)
  const password = decodeURIComponent(url.password)
  url.password = ''

  return { connectionString: url.toString(), env: { ...process.env, PGPASSWORD: password } }
}

// DELETE/UPDATE等、結果行を読み取る必要のないSQLを実行する。
export function runSql(sql: string): void {
  const connection = connect()
  if (!connection) return

  execFileSync('psql', [connection.connectionString, '-v', 'ON_ERROR_STOP=1', '-c', sql], {
    stdio: 'inherit',
    env: connection.env,
  })
}

// SELECTの結果行を読み取るためのSQLを実行する。各行は列を`|`で連結した配列。
export function querySql(sql: string): string[][] {
  const connection = connect()
  if (!connection) return []

  const output = execFileSync('psql', [connection.connectionString, '-v', 'ON_ERROR_STOP=1', '-t', '-A', '-c', sql], {
    env: connection.env,
  })
    .toString()
    .trim()

  return output ? output.split('\n').map((line) => line.split('|')) : []
}
