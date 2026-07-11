---
Status: Instructions
Scope: Phase 29ay P1
---

# P1: Unwind “noop code integration” (strict/dev verify only)

目的: Unwind は “予約だけ” で終わらせず、コード側にも **最小の整合チェック** を入れて契約を破れないようにする。
ただし release 既定の意味論/ログ/エラー文字列は不変。

参照SSOT:
- `docs/development/current/main/design/unwind-cleanup-effect-integration-ssot.md`
- `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- `docs/development/current/main/design/effect-classification-ssot.md`
- `docs/development/current/main/design/flowbox-observability-tags-ssot.md`

## 非目的

- unwind の実行経路（例外/stack unwinding）の実装
- 新しい env var の追加
- release 既定のログ増加

## 実装方針（Fail-Fast）

- **strict/dev only** で verify を強化し、Unwind が現れたときに “どの層で扱うべきか” を局所で示す。
- release は no-op（現状のまま通す/生成しない）。

## Step 1: EdgeCFG ExitKind::Unwind の整合チェック（strict/dev）

対象:
- `src/mir/builder/control_flow/edgecfg/api/verify.rs`

やること（例）:
- `ExitKind::Unwind` が `Frag.exits` に含まれる場合:
  - その ExitStub が未配線（or catch 受け口が未定義）なら Fail-Fast（strict/dev only）
  - エラーメッセージは `"[edgecfg/unwind] ..."` の安定タグで識別できること

## Step 2: Cleanup wrapper の ExitKind 網羅性チェック（strict/dev）

対象:
- `src/mir/builder/control_flow/edgecfg/api/compose/cleanup.rs`（または compose 共通）

やること:
- cleanup 合成が `Normal/Return/Break/Continue/Unwind` の全 ExitKind で “同一規則” を満たすことを verify で固定。
- 未実装の分岐（Unwind）は strict/dev で Fail-Fast、release は現状のまま（Unwind が出ない前提）。

## Step 3: FlowBox tag schema への ExitKind/feature 接続（strict/dev）

対象:
- `src/mir/builder/control_flow/plan/observability/flowbox_tags.rs`

やること:
- feature_set に `unwind` を追加できるよう語彙を用意（未使用でもOK）。
- タグは strict/dev の raw output のみ（filter は維持）。

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git commit -m "phase29ay(p1): add strict unwind contract verification"`
