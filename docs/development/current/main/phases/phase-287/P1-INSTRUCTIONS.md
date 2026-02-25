# Phase 287 P1: スモークテスト（quick）軽量化 — 作業指示書

## 目的（P1のゴール）

- `tools/smokes/v2/run.sh --profile quick` を「開発中に気軽に回せる速さ」に戻す。
- 既定挙動・意味論は不変（テストの配置/分類の変更のみ）。

## 成功条件（受け入れ基準）

- `--profile quick` が **45秒以内**（目安）で完走する。
- quick のテスト本数は **~100本以下**（目安）。
- 重い/環境依存のケースは integration/full（または plugins）へ移し、quick には残さない。
- `--filter` の導線を壊さない（パス階層は可能な限り維持する）。

## 方針（設計）

- 現状: quick 配下に `*.sh` が **651本**あり、`run.sh` はディレクトリ全探索で全て実行している。
- 解決: quick の責務を「最小ゲート」に再定義し、重いテストを profile 間で責務分離する。
  - quick: 入口導線＋using＋最小の制御/型/演算＋最小 selfhost 1本（必要なら）
  - integration: selfhost/Stage-B 系、長尺、S3/LLVM 連携、crate-exe 等の重いもの
  - full: 網羅（広い回帰）
  - plugins: プラグイン依存（動的ロード検証など）

## 手順（推奨の進め方）

### 1) 計測（現状の遅さを「どれが遅いか」に分解）

```bash
./tools/smokes/v2/measure_test_times.sh quick /tmp/smoke_test_times_quick.txt
head -50 /tmp/smoke_test_times_quick.txt.sorted
```

ここで「遅いファミリー」を把握する（例: `phase2100/*`, `selfhost_*`, `s3_*`, crate EXE 系など）。

### 2) quick に残す最小セットを決める（まず削る）

優先して quick に残す候補（あくまで例。実際は計測結果に合わせて調整）:

- `using` 解決の最小ケース（named / relative）
- 入口の `--backend vm` 実行（最小の `.hako`）
- if/loop/return の最小
- Stage-B/selfhost は **最小1本**に絞る（必要なら）

P1では「網羅」より「速さ」を優先する（integration/full が網羅担当）。

### 3) 重いテストを integration/full に移す（構造的に解決）

- 原則 `git mv` で移動する。
- **相対パスの階層は維持**する（例: `core/phase2100/...` は `profiles/integration/core/phase2100/...` に移す）。
  - これにより `--filter 'phase2100/...'` の使い勝手が保たれる。

例（方針イメージ）:

```bash
git mv tools/smokes/v2/profiles/quick/core/phase2100 tools/smokes/v2/profiles/integration/core/
git mv tools/smokes/v2/profiles/quick/core/phase251  tools/smokes/v2/profiles/integration/core/
```

注意:
- 既存の `tools/smokes/v2/README.md` にある通り、Stage‑B/selfhost canary 群は quick には不向き。
- まず「ディレクトリ単位で移動」→ 速くなる → その後に例外だけ戻す、の順で行う。

### 4) quick の再計測（差分の確認）

```bash
time ./tools/smokes/v2/run.sh --profile quick
./tools/smokes/v2/measure_test_times.sh quick /tmp/smoke_test_times_quick_after.txt
```

### 5) ドキュメント更新（SSOTを合わせる）

- `tools/smokes/v2/README.md`: quick / integration / full の責務と目安（秒・本数）を明記。
- `docs/development/current/main/phases/phase-287/README.md`: P1の結果（before/after の秒・本数）を追記。

## 追加の注意（やらないこと）

- P1では「並列実行（--jobs）」の実装改修には入らない（別フェーズで）。
- テストロジックのハードコードや by-name 分岐はしない（profile の責務分離で解決する）。

