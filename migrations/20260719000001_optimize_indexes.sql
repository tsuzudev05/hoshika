-- Migration: DBクエリ最適化（実測に基づくインデックス見直し）
--
-- 20,000件規模の合成データでEXPLAIN ANALYZE・pg_stat_user_indexesを使って検証した結果、
-- 以下の3つのインデックスがどのクエリからも使われていないことを確認したため削除する。
-- （検証方法・結果はTASKS.mdに記録）

-- 1. wish_items.status で絞り込むクエリは存在しない
--    （ステータスフィルターはフロントエンドで取得済みデータに対して行っている）。
--    実際にAPI経由でGET /wish-itemsを叩いてもidx_scanが0のままだったことを確認済み。
DROP INDEX idx_wish_items_status;

-- 2. budgets には UNIQUE (user_id, year, month) 制約があり、そのインデックスの
--    先頭列がuser_idであるため、user_id単体で絞り込むクエリもこの制約インデックスで
--    賄われる（EXPLAINで確認済み）。idx_budgets_user_idは完全な重複インデックス。
DROP INDEX idx_budgets_user_id;

-- 3. PostgresPurchaseRecordRepository::find_by_id (WHERE user_id = $1 AND id = $2) は
--    application層のどのユースケースからも呼ばれておらず（purchase_record_repoは
--    save()のみが実際の呼び出し経路）、このインデックスを使えるクエリが実行時に
--    そもそも発生しない。
DROP INDEX idx_purchase_records_user_id;

-- 検討したが見送ったもの:
-- wish_items(user_id, added_at) の複合インデックス — find_all（ORDER BY added_at、LIMIT無し）
-- を高速化できるか検証したが、LIMITが無いため実行計画は複合インデックスがあっても
-- Bitmap Heap Scan + 明示的Sortを選び続け、単一列インデックス+quicksortと比べて
-- 実測上の優位が確認できなかった（該当ユーザーの行数が数千件規模でも数百マイクロ秒差）。
-- 書き込みコストだけが増えるため追加を見送った。
