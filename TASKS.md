# Tasks

> Phase 01 完了記録 → [TASKS-phase01.md](./TASKS-phase01.md)
> Phase 02 完了記録 → [TASKS-phase02.md](./TASKS-phase02.md)
> Phase 03 完了記録 → [TASKS-phase03.md](./TASKS-phase03.md)
> Phase 04 完了記録 → [TASKS-phase04.md](./TASKS-phase04.md)
> Phase 05 完了記録 → [TASKS-phase05.md](./TASKS-phase05.md)
> Phase 06 完了記録 → [TASKS-phase06.md](./TASKS-phase06.md)

## Active

- [ ] **ユーザーテスト** — 身近な人に使ってもらいフィードバック収集
- [x] **本番デプロイ（Fly.io）** — 本番環境・ドメイン設定
  - Fly.ioアプリ`hoshika`（東京/nrtリージョン）を作成。`Dockerfile`（フロントエンドビルド→Rustビルド→実行イメージの3段階）・`fly.toml`・`.github/workflows/fly-deploy.yml`（`main`へのpushで自動デプロイ）は事前に用意済み
  - `src/main.rs`の`STATIC_DIR`環境変数による分岐（設定時のみAxumバイナリが`frontend/dist`を静的配信し、APIを`/api`配下にネスト）はローカルで`/health`・`/api/health`・`/`・静的アセット配信の両パターンを実機確認済み。`cargo test`101件も通過
  - DBは従量課金の懸念（Fly Postgresは自動停止せず常時課金対象になる）からFly Postgresを避け、Supabase（無料枠・東京/ap-northeast-1リージョン、Transaction pooler接続）を採用しFlyのリージョンと揃えた
  - `DATABASE_URL`（Supabase接続文字列）・`JWT_SECRET`（`openssl rand -hex 32`で生成）を`fly secrets set`で設定。GitHub Actions用の`FLY_API_TOKEN`もリポジトリシークレットに登録
  - 初回デプロイでバイナリが`GLIBC_2.38' not found`でクラッシュループする障害が発生。`Dockerfile`のビルドステージが`rust:1-slim`というバージョン固定なしのタグを使っており、実行ステージの`debian:bookworm-slim`とDebianバージョンがずれてglibcのABIが不整合になっていたのが原因。`rust:1-slim-bookworm`に固定して解消
  - 本番URL（https://hoshika.fly.dev）で実機確認: `/`が200、`/api/health`が`{"status":"ok"}`、認証必須の`/api/categories`が401（未ログイン時の想定通りの挙動）を返すことを確認し、DB接続・JWT認証とも本番で正常動作することを確認　完了（2026-09-06）
- [ ] **バグ修正・安定化** — Sentry活用
- [x] **ポートフォリオ掲載** — 転職活動用のプロジェクト説明文（アーキテクチャの工夫を中心に）
  - `portfolio.md`を作成。ショート版（200字）・ミディアム版（500字・GitHub README向け）・ロング版（技術面接向け）の3段階で記述　完了（2026-09-03）
- [ ] 🎉 **リリース完了** — ホシカ公開！

### 学習（並行）

- [ ] **「ドメイン駆動設計入門」読み進める** - Phase 02作業と同期して読む
  - 集約を設計するタイミングで集約の章を読む
- [ ] **「Clean Architecture」読み進める** - 原則・考え方を先に頭に入れる
  - 章ごとに「なぜそうするか」を自分の言葉でメモする
