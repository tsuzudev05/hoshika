import { deleteE2EPurchaseRecords, deleteE2EWishItems } from './db'

// e2eテストは実行のたびに `E2E` プレフィックス付きの名前でwish_itemsを作成する。
// クリーンアップ用のDELETE APIがまだ存在しないため、DBへ直接接続して後片付けする。
export default function globalTeardown() {
  // purchase_recordsがwish_itemsを外部キー参照しているため、先に子テーブルを削除する。
  deleteE2EPurchaseRecords()
  deleteE2EWishItems()
}
