# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)
> Phase 02 完了記録 → [TASKS-phase02.md](./TASKS-phase02.md)
> Phase 03 完了記録 → [TASKS-phase03.md](./TASKS-phase03.md)
> Phase 04 完了記録 → [TASKS-phase04.md](./TASKS-phase04.md)
> Phase 05 完了記録 → [TASKS-phase05.md](./TASKS-phase05.md)
> Phase 06 完了記録 → [TASKS-phase06.md](./TASKS-phase06.md)

## Active

### Phase 06 · 10月　仕上げ・言語化

> **このフェーズで学ぶこと**: 設計判断を言語化することで理解が定着する

- [x] **UI磨き込み** — 細部のUX改善・アニメーション・ローディング状態
  - ボタンに`:hover`/`:active`/`:disabled`の状態遷移（背景色・押下時の縮小・不透明度）を追加。それまでボタンはブラウザ既定のスタイルのみでクリックしても視覚的な反応が無かった
  - 「追加する」「欲しい／やめておく」「購入済みにする」の3箇所の送信ボタンに、処理中はスピナー+「追加中...」「記録中...」を表示するよう変更（それまでは`disabled`になるだけで、リクエスト中かどうかが見た目で分からなかった）
  - `BudgetMeter`/`WishItemList`/`AddWishItemForm`がそれぞれ個別に定義していた同一のスピナーCSS（`@keyframes`3つ）を`App.css`の共通`.spinner`クラスに統合し重複を解消
  - 新しく追加されたWishItemCard・購入フォームの表示にフェードインアニメーションを追加（既存カードはキー据え置きで再アニメーションしないことを確認）
  - `prefers-reduced-motion: reduce`に対応（アニメーション低減希望のユーザー向けに全トランジション/アニメーションを実質無効化。状態を伝えるスピナーの回転のみ維持）
  - 変更後、フロントエンドの`type-check`/`lint`/`test`（38件）・`npm run test:e2e`（実ブラウザ・5シナリオ）が通過することを確認。実際にVite開発サーバーで「追加する」ボタンのリクエストを意図的に遅延させてスクリーンショットを撮り、処理中表示が意図通り見えることを目視確認　完了（2026-07-23）
- [x] **PWA対応** — マニフェスト・Service Worker
  - `frontend/public/manifest.webmanifest`を追加し`index.html`から参照。アイコンは既存のfaviconと同じ⭐デザインのSVG（`icon.svg`）のみを用意。この環境にPNG変換ツール（ImageMagick等）が無いため複数解像度のPNGは用意できておらず、Android/Chromeのインストール可否には影響しないが、iOSのホーム画面追加はSVGを読めないため`apple-touch-icon`を別途用意する場合は今後PNG化が必要
  - Service Workerは`vite-plugin-pwa`等のビルド連携プラグインを使わず`frontend/public/sw.js`を手書き。Viteがハッシュ付与する`/assets/`配下はcache-first、HTMLナビゲーションはnetwork-first（オフライン時のみキャッシュへフォールバック）とし、`/api/`配下は一切キャッシュ対象外にして認証・鮮度が必要なAPIレスポンスがキャッシュされないようにした
  - 開発サーバー（Vite HMR）でキャッシュが介入しないよう、`registerServiceWorker()`は`import.meta.env.PROD`の時のみ登録するようにした
  - `npm run build`後、`npm run preview`で配信しPlaywrightで実機確認：Service Workerが`activated`状態になること、オンラインで一度ロードしてキャッシュを温めた後にオフラインへ切り替えて再読み込みしてもアプリシェル（タイトル・スケルトン表示）が表示されることを確認。API依存部分はオフラインでは意図通り「読み込み中」のまま止まる（SWがAPIをキャッシュしないため、偽のデータを見せない設計）。`type-check`/`lint`/`test`（38件）/`build`もすべて通過　完了（2026-07-28）
- [x] **README整備** — なぜこの設計にしたか・トレードオフ・アーキテクチャ図を書く
  - APIエンドポイント表が実装（購入記録・予算設定エンドポイント、JWT認証の要否、本番`/api`プレフィックス）に追いついていなかったため更新
  - 「リクエストの流れ」としてPOST `/wish-items/:id/purchase`を例にしたシーケンス図（Mermaid）を追加し、抽象的な依存方向の図だけでは伝わらない「実際に1リクエストが各レイヤーをどう通るか」を可視化
  - 「設計判断とトレードオフ」セクションを新設し、実装中に下した非自明な判断を5件記録: ①予算超過をブロックしない事後可視化のみの設計 ②認可が「有効なJWTか」の一点のみでユーザー間分離は別軸という既知の制約 ③`DomainEvent`が生成されるだけで購読者がまだ無い意図的な未完成 ④Sentryへのエラー送信をインフラ層由来のみに絞る分類 ⑤DBインデックスを実測して「足す/足さない」を判断した実例（`idx_wish_items_status`等の削除・複合インデックス追加の見送り）
  - DB設計固有のトレードオフ（ENUM採用・balance非都度計算等）は既存の`db-design.md`にあったため重複させず相互参照のみとした　完了（2026-07-19）
  - ユーザーからのフィードバックを受けて追記（2026-07-23）: 冒頭に「アーキテクチャ概要（30秒で把握する）」表を新設し、詳細セクションへ入る前に全体像（何をするアプリか・3コンテキスト・レイヤー構成・技術スタック・現状のスコープ外）を一目で把握できるようにした
  - トレードオフを4件追加（計9件）: ⑥`sqlx::query!`系コンパイル時チェックマクロを使わずビルド時にDB接続を不要にした判断 ⑦Postgres実装とは別にInMemory実装を用意し、ユニットテストがDB無しで完結するようにした設計 ⑧Categoryを固定シードのみとし動的なCRUD APIを持たせなかった判断 ⑨CORS未設定のまま据え置いている理由（単一オリジン配信の前提が崩れるまでは先回りしない）
- [ ] **Zenn記事執筆** — 「RustでDDD + Clean Architectureを実践した」知見を記事化
  - DDD と Clean Architecture をどう組み合わせたか（役割分担の整理）
  - 依存逆転の原則をRustのtraitでどう実現したか
  - どこで悩んだか・失敗した設計・直した理由を正直に書く
- [ ] **ユーザーテスト** — 身近な人に使ってもらいフィードバック収集

### Phase 07 · 11月　リリース

- [ ] **ユーザーテスト** — 身近な人に使ってもらいフィードバック収集（Phase 06から繰り越し）
- [x] **本番環境デプロイ** — Fly.io本番環境・ドメイン設定（Phase 05で保留した`Fly.ioデプロイ`の再開が前提）
  - Fly.ioアプリ`hoshika`（東京/nrtリージョン）を作成。DBはFly Postgres（常時起動で課金対象になる）を避け、Supabase（東京/ap-northeast-1リージョン、Transaction pooler接続）を採用しリージョンをFlyと揃えた
  - `DATABASE_URL`（Supabase接続文字列）・`JWT_SECRET`（`openssl rand -hex 32`で生成）を`fly secrets set`で設定。GitHub Actions用の`FLY_API_TOKEN`もリポジトリシークレットに登録し、`main`へのpushで`fly-deploy.yml`が自動デプロイする状態にした
  - 初回デプロイでバイナリが`GLIBC_2.38' not found`でクラッシュループする障害が発生。`Dockerfile`のビルドステージが`rust:1-slim`というバージョン固定なしのタグを使っており、実行ステージの`debian:bookworm-slim`とDebianバージョンがずれてglibcのABIが不整合になっていたのが原因。`rust:1-slim-bookworm`に固定して解消
  - 本番URL（https://hoshika.fly.dev）で実機確認: `/`が200、`/api/health`が`{"status":"ok"}`、認証必須の`/api/categories`が401（未ログイン時の想定通りの挙動）を返すことを確認し、DB接続・JWT認証とも本番で正常動作することを確認　完了（2026-09-06）
- [ ] **バグ修正・安定化** — Sentry活用
- [x] **ポートフォリオ掲載** — 転職活動用のプロジェクト説明文（アーキテクチャの工夫を中心に）
  - `portfolio.md`を作成。ショート版（200字）・ミディアム版（500字・GitHub README向け）・ロング版（技術面接向け）の3段階で記述　完了（2026-09-03）
- [ ] 🎉 **リリース完了** — ホシカ公開！

### Phase 05 · 品質・インフラ（保留中のタスク）

> 大部分は完了 → [TASKS-phase05.md](./TASKS-phase05.md) 参照。以下はアカウントに紐づく判断待ちのため保留中。

- [x] **Fly.ioデプロイ** — ステージング環境・自動デプロイ
  - 設定ファイル一式を作成済み: `Dockerfile`（フロントエンドビルド→Rustビルド→実行イメージの3段階）・`.dockerignore`・`fly.toml`・`.github/workflows/fly-deploy.yml`（`main`へのpushで自動デプロイ）
  - `src/main.rs`に`STATIC_DIR`環境変数による分岐を追加。設定時のみAxumバイナリが`frontend/dist`を静的配信し、APIを`/api`配下にネストする（未設定のローカル/CIでは従来通りAPIがルート直下のまま動作し、既存のE2E・CIには一切影響しない）
  - ローカルで`STATIC_DIR`未設定/設定済みの両方を実機確認（`/health`・`/api/health`・`/`・静的アセット配信）。`cargo test`101件も通過を確認
  - 従量課金の懸念（Fly Postgresは自動停止せず常時課金対象になる）を検討した結果、DBはFly Postgresを使わずSupabase（無料枠・東京リージョン）を採用する方針に決定。Fly.io側もクレジットカード登録（デビットカードで対応）・`fly launch`・シークレット設定を実施し、本番デプロイまで完了（詳細は上記「本番環境デプロイ」参照）　完了（2026-09-06）

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
