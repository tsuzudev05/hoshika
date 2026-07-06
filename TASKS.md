# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)
> Phase 02 完了記録 → [TASKS-phase02.md](./TASKS-phase02.md)
> Phase 03 完了記録 → [TASKS-phase03.md](./TASKS-phase03.md)

## Active

### Phase 04 · フロントエンド実装（React + TypeScript）（8月〜）

> **このフェーズで学ぶこと**: フロントエンドでも関心の分離を意識する

- [x] **Vite + React + TypeScript セットアップ** - 完了（既存のscaffold: vite.config.ts で `/api` → `:3000` にプロキシ設定済み）
- [x] **API層の分離** - `frontend/src/api/` にAPIコール関数を集約　完了（2026-07-04）
  - `client.ts` — fetchラッパー、`{error: string}` 形式のエラーレスポンスを `ApiError` に変換
  - `types.ts` — バックエンドDTOと1対1対応の型（`WishItem` / `BudgetStatus` / 各Input型）
  - `wishItems.ts` / `budgets.ts` — エンドポイントごとの関数（コンポーネントは直接fetchしない）
- [x] **UIコンポーネント実装**（欲しいものカード・予算メーター・カテゴリフィルター）
  - [x] 一覧表示 — `WishItemList` で `GET /wish-items` を `useQuery` 経由表示　完了（2026-07-04）
  - [x] カード化・詳細表示 — `WishItemCard` に切り出し、価格・カテゴリ・メモ・登録日を表示（ステータスはバッジ表示）　完了（2026-07-05）
  - [x] 予算メーター — `BudgetMeter` で当月の `GET /budgets/status` を表示（予算・残高・進捗バー・超過バッジ、未設定時は404を空状態として表示）　完了（2026-07-05）
  - [x] カテゴリフィルター — `CategoryFilter` コンポーネントを追加し、`WishItemList` でカテゴリ選択によるクライアントサイド絞り込み（「すべて」/各カテゴリ切り替え、該当なし時のメッセージ表示）を実装　完了（2026-07-06）
- [x] **衝動買い防止フロー** — 「本当に欲しいか？」チェックUI（`POST /wish-items/:id/review`）
  - [x] `WishItemList` にレビュー操作（「欲しい」/「やめておく」ボタン、`Inbox` ステータスのみ表示）を実装　完了（2026-07-04）
  - [x] 追加（`POST /wish-items`）フォーム — `AddWishItemForm` を実装。バックエンドに `GET /categories`（`CategoryOutput` / `list_categories` ハンドラー）を追加し、カテゴリ選択のブロックを解消　完了（2026-07-05、要DevContainer側で `cargo build` / `npm test` 確認）
- [x] **TanStack Queryで状態管理** — `QueryClientProvider` 設定済み。一覧取得（`useQuery`）・レビュー（`useMutation` + `invalidateQueries`）を導入済み。追加の `useMutation` は上記の理由で未着手
- [x] **フロントエンドのテスト整備** — UIコンポーネント・衝動買い防止フローの実装が一段落した時点で着手（Phase02/03の「層の実装後にテストを整備する」流れに合わせる）
  - [x] テスト基盤導入（Vitest + @testing-library/react + jsdom + msw）　完了（2026-07-05）
  - [x] `api/client.ts` のユニットテスト — 正常系 / `{error}` 形式 / パース不能時のフォールバック / ネットワークエラー / POST送信内容　完了（2026-07-05、6テスト）
  - [x] `WishItemList` のテスト — loading / error(+再試行ボタン) / empty / data / レビュー操作成功時の更新　完了（2026-07-05、5テスト）
  - [x] `WishItemCard` のテスト — 詳細表示 / メモ空欄時の非表示 / Inboxのみレビューボタン表示 / onReviewの引数 / disabled / reviewError表示　完了（2026-07-05、6テスト）
  - [x] `BudgetMeter` のテスト — loading / 404時の空状態 / それ以外のエラー(+再試行) / 予算表示 / 超過バッジ　完了（2026-07-05、5テスト、`vi.setSystemTime` で当月判定を固定）
  - [x] E2Eテスト（Playwright）はPhase05のタスクとして別途担当　→ [hoshika-roadmap.md](./hoshika-roadmap.md) Phase 05 参照
- [x] **レスポンシブ対応**（スマホファースト） — `App.css` にモバイルファーストのコンテナ・タイポグラフィ・ボタンの基本スタイルを追加し、`min-width` メディアクエリで640px→720pxへ段階的に拡張。`AddWishItemForm` は600px以上で2カラムグリッドに、`WishItemCard` のレビューボタンは480px未満で縦積み・以上で横並びに切り替え　完了（2026-07-06、Playwrightでモバイル幅375px/デスクトップ幅1280pxの表示を確認）

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
