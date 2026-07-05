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
- [ ] **UIコンポーネント実装**（欲しいものカード・予算メーター・カテゴリフィルター）
  - [x] 一覧表示 — `WishItemList` で `GET /wish-items` を `useQuery` 経由表示　完了（2026-07-04）
  - [x] カード化・詳細表示 — `WishItemCard` に切り出し、価格・カテゴリ・メモ・登録日を表示（ステータスはバッジ表示）　完了（2026-07-05）
  - [x] 予算メーター — `BudgetMeter` で当月の `GET /budgets/status` を表示（予算・残高・進捗バー・超過バッジ、未設定時は404を空状態として表示）　完了（2026-07-05）
  - [ ] カテゴリフィルター（※ `GET /categories` 相当のエンドポイントが現状バックエンドに無いため、先にAPI追加が必要）
- [ ] **衝動買い防止フロー** — 「本当に欲しいか？」チェックUI（`POST /wish-items/:id/review`）
  - [x] `WishItemList` にレビュー操作（「欲しい」/「やめておく」ボタン、`Inbox` ステータスのみ表示）を実装　完了（2026-07-04）
  - [ ] 追加（`POST /wish-items`）フォーム — `GET /categories` 相当のエンドポイントが無いため保留（下記参照）
- [x] **TanStack Queryで状態管理** — `QueryClientProvider` 設定済み。一覧取得（`useQuery`）・レビュー（`useMutation` + `invalidateQueries`）を導入済み。追加の `useMutation` は上記の理由で未着手
- [ ] **フロントエンドのテスト整備** — UIコンポーネント・衝動買い防止フローの実装が一段落した時点で着手（Phase02/03の「層の実装後にテストを整備する」流れに合わせる）
  - [ ] テスト基盤導入（Vitest + @testing-library/react + jsdom + msw）
  - [ ] `api/client.ts` のユニットテスト — `ApiError` への変換ロジック（正常系 / `{error}` 形式 / パース不能時のフォールバック）
  - [ ] 各コンポーネントのテスト（`WishItemList` の loading / error / empty / data 各状態など）
  - [ ] E2Eテスト（Playwright）はPhase05のタスクとして別途担当　→ [hoshika-roadmap.md](./hoshika-roadmap.md) Phase 05 参照
- [ ] **レスポンシブ対応**（スマホファースト）

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
