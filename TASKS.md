# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)
> Phase 02 完了記録 → [TASKS-phase02.md](./TASKS-phase02.md)
> Phase 03 完了記録 → [TASKS-phase03.md](./TASKS-phase03.md)
> Phase 04 完了記録 → [TASKS-phase04.md](./TASKS-phase04.md)
> Phase 05 完了記録 → [TASKS-phase05.md](./TASKS-phase05.md)
> Phase 06 完了記録 → [TASKS-phase06.md](./TASKS-phase06.md)

## Active

### Phase 07 · 11月　リリース

- [ ] **ユーザーテスト** — 身近な人に使ってもらいフィードバック収集（Phase 06から繰り越し）
- [ ] **本番環境デプロイ** — Fly.io本番環境・ドメイン設定（Phase 05で保留した`Fly.ioデプロイ`の再開が前提）
- [ ] **バグ修正・安定化** — Sentry活用
- [x] **ポートフォリオ掲載** — 転職活動用のプロジェクト説明文（アーキテクチャの工夫を中心に）
  - `portfolio.md`を作成。ショート版（200字）・ミディアム版（500字・GitHub README向け）・ロング版（技術面接向け）の3段階で記述　完了（2026-09-03）
- [ ] 🎉 **リリース完了** — ホシカ公開！

### Phase 05 · 品質・インフラ（保留中のタスク）

> 大部分は完了 → [TASKS-phase05.md](./TASKS-phase05.md) 参照。以下はアカウントに紐づく判断待ちのため保留中。

- [ ] **Fly.ioデプロイ** — ステージング環境・自動デプロイ（**保留・後回し**）
  - 設定ファイル一式を作成済み: `Dockerfile`（フロントエンドビルド→Rustビルド→実行イメージの3段階）・`.dockerignore`・`fly.toml`・`.github/workflows/fly-deploy.yml`（`main`へのpushで自動デプロイ）
  - `src/main.rs`に`STATIC_DIR`環境変数による分岐を追加。設定時のみAxumバイナリが`frontend/dist`を静的配信し、APIを`/api`配下にネストする（未設定のローカル/CIでは従来通りAPIがルート直下のまま動作し、既存のE2E・CIには一切影響しない）
  - ローカルで`STATIC_DIR`未設定/設定済みの両方を実機確認（`/health`・`/api/health`・`/`・静的アセット配信）。`cargo test`101件も通過を確認
  - この環境にはFly.ioアカウント認証・`flyctl`がないため、実際の`fly launch`（アプリ作成）・`fly postgres create`・`fly secrets set`・GitHub Secretsへの`FLY_API_TOKEN`登録はユーザー自身が行う必要がある（手順は[DEVELOPMENT.md](./DEVELOPMENT.md#デプロイflyio)に記載）。`docker build`自体もこの環境にDockerがないため未実行・未検証
  - ユーザー環境（WSL2）で`fly launch --no-deploy`実行時にIPv6経路不良による接続エラーが発生し解消済み。その後Fly.io側からアカウント確認（クレジットカード登録）を要求されたが、従量課金の発生条件（Postgresは自動停止しないため常時課金対象になる等）を精査してから登録するか判断したいとのことで、一旦保留（2026-07-19）

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
