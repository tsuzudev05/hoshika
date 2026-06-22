# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)

## Active

### Phase 02 · ドメイン層・アプリケーション層（7月〜）　※ DDD + Clean Architecture

- [ ] **Rustプロジェクト構造をレイヤーで切る** - domain / application / infrastructure / presentation
  - 依存ルール確認: `domain/` が `axum` / `sqlx` に依存していないこと
- [ ] **WishItemエンティティ実装** - IDによる同一性・不変条件をメソッドで保護
  - ステータス遷移は `WishItem::review()` メソッドに閉じ込める
- [ ] **値オブジェクト実装** - Price / Category / WishItemStatus / Memo / YearMonth
  - `WaitingPeriod` は見送り（レビュー行為で防止する設計のため）
- [ ] **WishItemRepository trait定義** - DBを知らないインターフェース（domain層に置く）
- [ ] **BudgetService ドメインサービス実装** - 予算超過チェック（複数集約をまたぐため）
- [ ] **ユースケース実装** - AddWishItem / ReviewWishItem / GetBudgetStatus
  - application層に置く。HTTPを知らない。引数・戻り値はDTO
- [ ] **InMemoryRepository実装** - テスト用。DBなしでドメイン・ユースケース層をテスト
- [ ] **ドメイン層のテスト整備** - cargo testだけで通るか確認（DBもAxumも不要）

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする

## Someday

### Phase 03 · 8月前半　インフラ層・プレゼンテーション層（Rust）

> **このフェーズで学ぶこと**:
> - **Clean Architecture視点**: 依存逆転の原則の実感。traitのimplを差し替えるだけで外部依存を切り替えられる
> - **DDD視点**: ドメインモデルをDBスキーマに写像するときの「インピーダンスミスマッチ」への対処

- [ ] **インフラ層の実装**
  - `PostgresWishItemRepository` — `WishItemRepository` traitのimpl（SQLxで実装）
  - `JwtAuthService` — 認証の実装はここ（Domain層はAuthを知らない）
  - DIコンテナ的な組み立て（Rustではstate管理やtrait objectで）
  - 注意: ドメインオブジェクト ↔ DBレコードの変換（mapping）はInfrastructure層の責務
- [ ] **プレゼンテーション層の実装**
  - Axumハンドラーは「リクエストのパース → ユースケース呼び出し → レスポンス変換」のみ
  - ビジネスロジックがハンドラーに漏れていたら設計ミスのサイン
  - HTTPステータスコードへのエラーマッピングもここで行う
- [ ] **変更容易性の検証（Clean Architectureの真価）**
  - `InMemoryWishItemRepository` を実装してドメイン・アプリケーション層のテストが通るか確認
  - 通れば「データ永続化の詳細をドメインが知らない」設計が証明される（依存逆転の原則の実証）
  - 通らない場合は設計に漏れがある → 修正してレイヤー境界を正す
