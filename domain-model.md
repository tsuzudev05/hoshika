# ドメインモデル設計

> Phase 01 成果物 — 2026-06-13

---

## 集約の設計判断

### 問い: WishItem と CheckFlow（レビューフロー）は同じ集約か？

**結論: 同じ集約。`WishItem` が集約ルート。**

**理由:**

レビューフローは「WishItemのステータスを変化させる行為」であり、WishItem外部に存在するオブジェクトではない。`WishItemStatus` という値オブジェクトが状態を表し、その遷移ロジックは WishItem のメソッドとして定義される。

```
wishItem.review(Decision::NextToBuy)  // ← これは WishItem のメソッド
```

独立した集約にすべき条件（独自のIDで追跡が必要・別トランザクションで整合性を取る必要がある）をCheckフローは満たさない。

---

## 集約一覧

### 集約 1: WishItem

| 要素 | 種別 | 説明 |
|------|------|------|
| `WishItem` | エンティティ（集約ルート） | 欲しいものリストの1件。IDで同一性を追跡 |
| `Price` | 値オブジェクト | 希望価格。0円以上の正の値 |
| `Category` | 値オブジェクト | アイテムの分類（書籍・ガジェット・ファッション等） |
| `WishItemStatus` | 値オブジェクト | ステータス（Inbox / NextToBuy / OnHold / Archived / Purchased） |

**不変条件（Invariants）:**

- `Price` は負の値を取れない
- 新規作成時のステータスは必ず `Inbox`
- `Archived` または `Purchased` からの遷移は不可（終端状態）
- `NextToBuy` 以外のステータスから `Purchased` への遷移は不可

**ステータス遷移図:**

```
[Inbox]
    │  review(NextToBuy)
    ├──────────────────→ [NextToBuy]
    │                          │ purchase()
    │                          └─────────→ [Purchased] ← 終端
    │  review(OnHold)
    ├──────────────────→ [OnHold]
    │                          │ review(NextToBuy)
    │                          └─────────→ [NextToBuy]
    │                          │ review(Archived)
    │                          └─────────→ [Archived] ← 終端
    │  review(Archived)
    └──────────────────→ [Archived] ← 終端
```

---

### 集約 2: Budget

| 要素 | 種別 | 説明 |
|------|------|------|
| `Budget` | エンティティ（集約ルート） | 月次の使用可能金額。月・金額・残高を持つ |
| `PurchaseRecord` | エンティティ | 購入記録。WishItemへの参照・実支払額・日付・メモを持つ |
| `Price` | 値オブジェクト | 予算金額・残高・実支払額（WishListコンテキストとShared Kernel） |
| `Memo` | 値オブジェクト | 購入メモ。空可 |

**不変条件（Invariants）:**

- 同一月に `Budget` は1件のみ存在する
- 予算金額（amount）は 0円より大きい値
- 残高（balance）は負になりうる（超過を許容し、`BudgetExceeded` イベントで通知）
- `PurchaseRecord` は必ず対応する `WishItem` の ID を持つ

---

## 値オブジェクト一覧

| 名前 | 型イメージ（Rust） | 不変条件 |
|------|--------------------|----------|
| `Price` | `struct Price(u64)` — 円単位の整数 | 負の値不可（`u64` で型レベルで保証） |
| `Category` | `enum Category` または `struct Category(String)` | 空文字不可 |
| `WishItemStatus` | `enum WishItemStatus` | 定義外のバリアントへの遷移は compile error |
| `Memo` | `struct Memo(String)` | 最大文字数制限（例: 500文字） |
| `YearMonth` | `struct YearMonth { year: u16, month: u8 }` | 月は 1〜12 |

> `WishItemStatus` は ImpulsePrevention コンテキストで定義するが、MVP では WishList コンテキストとの Shared Kernel として扱う。

---

## ドメインイベント 完全版

### WishList コンテキスト

| イベント名 | 発生トリガー | 主なペイロード |
|------------|-------------|---------------|
| `ItemAdded` | WishItem が Inbox に追加された | `wish_item_id`, `name`, `price`, `category`, `added_at` |
| `ItemReviewed` | ユーザーがレビューしステータスを変更した | `wish_item_id`, `old_status`, `new_status`, `reviewed_at` |
| `ItemMovedToNextToBuy` | `ItemReviewed` のうち NextToBuy への変更（購入候補になった） | `wish_item_id`, `reviewed_at` |
| `ItemArchived` | WishItem が「不要」と判断されアーカイブされた | `wish_item_id`, `archived_at` |
| `ItemPurchased` | WishItem が購入され PurchaseRecord が生成された | `wish_item_id`, `purchase_record_id`, `actual_price`, `purchased_at` |

### 予算管理コンテキスト

| イベント名 | 発生トリガー | 主なペイロード |
|------------|-------------|---------------|
| `BudgetSet` | 月次予算が設定された | `budget_id`, `year_month`, `amount`, `set_at` |
| `PurchaseRecorded` | PurchaseRecord が Budget に記録された | `budget_id`, `purchase_record_id`, `actual_price`, `balance_after` |
| `BudgetExceeded` | 購入によって残高が 0 を下回った | `budget_id`, `year_month`, `over_amount`, `occurred_at` |

### コンテキスト間のイベントフロー

```
[WishList]                    [ImpulsePrevention]           [Budget]

ItemAdded ──────────────────→ Inboxへ追加
                               （ACLで変換）

ItemReviewed (→ NextToBuy) ─→ 購入候補としてマーク

ItemPurchased ──────────────────────────────────────────→ PurchaseRecorded
（Published Language）                                      BudgetExceeded（残高不足時）
```

---

## 設計上のトレードオフと判断

### `PurchaseRecord` を `Budget` 集約内に置いた理由

WishItemの希望価格と実際の支払額が異なるケース（セール等）が存在する。購入記録は予算の文脈（いくら使ったか）に強く依存するため、`Budget` 集約に内包した。WishItem側は `Purchased` ステータスになるだけで、金額の詳細は持たない。

### `WaitingPeriod` は見送り

当初 `WaitingPeriod` を値オブジェクトとして想定していたが、衝動買い防止は「システムのタイマーで強制する」設計ではなく「ユーザーが能動的にレビューする」設計に決定したため、現時点では不要。Inbox 状態であること自体が待機期間の役割を担う。

将来的に「登録からN日経過しないとレビューできない」機能を追加する場合は `WaitingPeriod` を導入する。

---

## 次のステップ

- [ ] DB設計（ドメインモデルからテーブル設計を導出）— due 6/25
- [ ] Rust での型定義実装（Phase 02）
