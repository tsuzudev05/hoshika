-- Migration: add per-user data isolation
--
-- wish_items / budgets / purchase_records は今までグローバル共有のデータセットだった。
-- JWT の sub（user_id）でパーティションできるように user_id 列を追加する。
-- 既存行は現行の唯一のアプリユーザー 'hoshika-app' に紐付けてからデフォルトを外す
-- （以降のINSERTはuser_idの明示指定を必須にする）。
-- categories は固定の共有マスタ（UNIQUE(name)）のため対象外。

ALTER TABLE wish_items ADD COLUMN user_id TEXT NOT NULL DEFAULT 'hoshika-app';
ALTER TABLE wish_items ALTER COLUMN user_id DROP DEFAULT;
CREATE INDEX idx_wish_items_user_id ON wish_items(user_id);

ALTER TABLE budgets ADD COLUMN user_id TEXT NOT NULL DEFAULT 'hoshika-app';
ALTER TABLE budgets ALTER COLUMN user_id DROP DEFAULT;
CREATE INDEX idx_budgets_user_id ON budgets(user_id);

-- 予算の一意性はユーザー単位に変更する（そうしないと他ユーザーが同じ年月の予算を設定できない）
ALTER TABLE budgets DROP CONSTRAINT budgets_unique_year_month;
ALTER TABLE budgets ADD CONSTRAINT budgets_unique_user_year_month UNIQUE (user_id, year, month);

ALTER TABLE purchase_records ADD COLUMN user_id TEXT NOT NULL DEFAULT 'hoshika-app';
ALTER TABLE purchase_records ALTER COLUMN user_id DROP DEFAULT;
CREATE INDEX idx_purchase_records_user_id ON purchase_records(user_id);
