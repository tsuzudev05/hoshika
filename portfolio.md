# ホシカ — ポートフォリオ掲載用プロジェクト説明文

> 転職活動での「プロジェクト紹介」に使う説明文。用途に応じて長短を使い分ける。

---

## ショート版（200字・一言紹介欄向け）

Rust（Axum + SQLx）と React + TypeScript で実装した「欲しいものリスト × 予算管理」アプリ。DDD（ドメイン駆動設計）と Clean Architecture を実践し、依存逆転の原則・集約の不変条件・InMemoryリポジトリによるDB不要テストなど、設計の変更容易性を体得することを目的とした個人プロジェクト。

---

## ミディアム版（500字・履歴書プロジェクト欄・GitHub README向け）

### ホシカ（欲しいものリスト × 予算管理アプリ）

**リポジトリ**: https://github.com/tsuzudev05/hoshika
**デモ**: https://hoshika.fly.dev

衝動買いを防ぐため「登録した瞬間は買えない（Inbox状態）」というドメインルールを持つアプリ。欲しいものを一度リストに入れてレビューを挟み、「本当に欲しいか」を確認してから購入できる仕組み。

**技術スタック**
- Backend: Rust（Axum + SQLx）
- Frontend: React + TypeScript（Vite + TanStack Query）
- DB: PostgreSQL（Supabase） / Infrastructure: Fly.io（東京リージョン）

**設計上の特徴**

DDDと Clean Architecture を組み合わせ、Domain層がフレームワーク（Axum / SQLx）に一切依存しない構造を実現。Repository traitをDomain層に定義し、PostgreSQL実装とInMemory実装を差し替え可能にすることで、`cargo test`がDB無しで完結する。

欲しいものの状態遷移（`Inbox → NextToBuy / OnHold → Purchased`）はドメインオブジェクト自身が不変条件として保護し、Application/Presentation層は関知しない。予算超過をブロックせず「事後可視化のみ」とした設計判断など、ビジネスルールをコードで表現する実践を積んだ。

**品質**
- テスト: バックエンド101件 / フロントエンド38件 / E2E（Playwright）5シナリオ
- Lighthouse: Performance 93 / Accessibility 100 / Best Practices 100 / SEO 100
- CI: Rust（型チェック・clippy・test）/ Frontend（型チェック・lint・test・build）/ E2E の3ジョブ

---

## ロング版（技術面接・詳細説明向け）

### プロジェクトの目的と背景

このプロジェクトの真の目的は「動くものを作る」ではなく、**DDD + Clean Architectureを体得すること**。「AIが生成したコードの設計上の問題を見抜けるエンジニアになる」という動機から始まった。

フレームワークやDBに依存したコードは、動作しても変更に弱い。依存の方向を制御し、ドメインの言葉でコードを書く訓練として、ホビープロジェクトとして取り組んだ。

### アーキテクチャと設計判断

**レイヤー構成と依存の方向**

```
Presentation（Axum handlers）
      ↓
Application（Use Cases）
      ↓
Domain（Entities / Value Objects / Repository traits）
      ↑
Infrastructure（SQLx / 外部API）
```

Domain層は標準ライブラリのみに依存。`use axum` も `use sqlx` も書かない。依存チェックスクリプト（`scripts/check-layer-deps.sh`）でCIに組み込み、違反を自動検出する仕組みを用意した。

**バウンデッドコンテキストの識別**

3つのコンテキスト（欲しいものリスト / 衝動買い防止 / 予算管理）を識別し、コンテキストマップを設計した。Customer-Supplier + ACL（腐敗防止層）と Published Language（ドメインイベント）の2パターンでコンテキスト間を疎結合につなぐ設計を採用。

**設計変更の実例: WaitingPeriodの削除**

最初の設計案は「7日間の待機期間（WaitingPeriod）タイマー」だったが、「衝動買いを防ぐのはシステムのタイマーではなく、リストを見直すという行為そのもの」と気づき概念ごと削除。ユビキタス言語の整理を通じて「ドメインに実在しない概念」を除去できた事例として記録している。

**値オブジェクトとプリミティブ型の判断**

`Balance(i64)` という値オブジェクトを導入し、`is_exceeded()` / `is_sufficient_for()` / `deduct()` でドメイン操作をメソッドとして表現。生の `i64` では `< 0` の計算の羅列になるが、値オブジェクト化で意図が読み取れるコードになった。`WishItemName` も同様に空文字列不可のルールを型レベルで保証し、`WishItem::new()` が `Result` を返す必要をなくした。

**InMemoryRepositoryによる設計の実証**

PostgreSQL実装とInMemory実装を並行して持つことで、`cargo test`がDB無しで動く設計を実現。「InMemoryで動く = Domain/Application層にDBへの漏れがない」ことの証明になっている。実際、開発中にDomain層の `RepositoryError` に `#[from] sqlx::Error` が混入する違反を依存チェックスクリプトで検出・修正した経験がある。

**`new()` と `reconstitute()` の分離**

集約の「新規作成」と「DBからの復元」を別コンストラクタで表現。`Budget::new()` は内部で残高を予算金額から計算する（不変条件をアグリゲートが守る）のに対し、`Budget::reconstitute()` はDB保存済みの値をそのまま復元する。この区別を設けることで、呼び出し側が不変条件を破れない構造を実現した。

### 品質への取り組み

| 項目 | 内容 |
|---|---|
| バックエンドテスト | 101件（ドメイン層・ユースケース層・InMemoryリポジトリをカバー） |
| フロントエンドテスト | 38件（RTL + Vitest・MSWでAPIモック） |
| E2Eテスト | Playwright 5シナリオ（追加→レビュー→購入→予算超過表示の主要フロー） |
| CI | GitHub Actions 3ジョブ（Rust / Frontend / E2E）でPR時に自動検証 |
| Lighthouse | Performance 93 / Accessibility 100 / Best Practices 100 / SEO 100 |
| PWA | Service Worker手書き（assets: cache-first / HTML: network-first / API: キャッシュ対象外） |
| アクセシビリティ | `prefers-reduced-motion` 対応・ARIAラベル・コントラスト比修正 |

### アウトプット

- READMEに設計判断とトレードオフを9件記録（「なぜそう設計したか」を後から追えるように）
- Zennに全12章のbook形式で執筆（DDD + Clean Architecture をRustで実践した知見を公開予定）

### 学んだこと / 正直に書く失敗

**うまくいったこと**
- Clean Architectureのレイヤー境界を守り続けることで、認証ミドルウェア追加・DBインデックス最適化・PWA対応などの横断的変更が既存層に影響を与えなかった
- DBインデックスを「実測して要らないものを消す」判断ができた（使われていない3つを削除、効きそうで効かなかった複合インデックスの追加を見送り）

**難しかったこと**
- バウンデッドコンテキストの境界を引く判断は最後まで迷った。WishItemStatusをShared Kernelとして共有している点は「真のコンテキスト分離」という観点では妥協であり、コンテキストが独立デプロイ単位になる場合は分離が必要
- 認証・認可は「有効なJWTがなければ弾く」という最低限の閂しか閉めておらず、ユーザー登録フロー・パスワード認証は未実装のまま。本番で複数ユーザーに使わせるには見直しが必要な既知の制約として残している
