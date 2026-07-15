import { runSql } from './db'

// e2eテストは実行のたびに `E2E` プレフィックス付きの名前でwish_itemsを作成する。
// クリーンアップ用のDELETE APIがまだ存在しないため、DBへ直接接続して後片付けする。
export default function globalTeardown() {
  // purchase_recordsがwish_itemsを外部キー参照しているため、先に子テーブルを削除する。
  runSql("DELETE FROM purchase_records WHERE wish_item_id IN (SELECT id FROM wish_items WHERE name LIKE 'E2E%');")
  runSql("DELETE FROM wish_items WHERE name LIKE 'E2E%';")
}
