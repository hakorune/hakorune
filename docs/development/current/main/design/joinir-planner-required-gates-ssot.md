---
Status: SSOT
Scope: planner-required gate の入口・TSV 契約・allow_rc の扱いを 1 枚に固定する
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/phase-29bp/README.md
- docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
- tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv
- tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh
- tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
---

# JoinIR planner-required gates (SSOT)

## Goal

- `HAKO_JOINIR_PLANNER_REQUIRED=1` を strict/dev gate で常時有効化しても、既知ケースが planner-first で通ることを固定する。
- stdout を SSOT として扱い、exit code の 0–255 丸めは `allow_rc` で個別に吸収する（意味論と実行系都合を分離）。
- 既定挙動（release default）は不変。

## Boundary with De-Rust Done

- planner-required gate の緑は compiler meaning の健全性証跡であり、de-rust transfer lane done 判定の代替にはしない。
- de-rust done 判定は `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md` の
  X32/X33/X34/X35 matrix replay を必須とする。

## Entry points (SSOT)

- planner-required dev gate v4:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- Phase 29bq fast iteration gate:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- selfhost planner-required entry (opt-in):
  - `SMOKES_ENABLE_SELFHOST=1 ./tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - quick/probe knobs (default behaviorは不変):
    - `SMOKES_SELFHOST_FILTER=<substring>`（fixture/planner_tag/reason を部分一致で絞る）
    - `SMOKES_SELFHOST_MAX_CASES=<N>`（先頭 N 件だけ実行）
    - `SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS=<sec>` / `SMOKES_SELFHOST_JSON_TIMEOUT_SECS=<sec>`（phase別 timeout）
    - `SMOKES_SELFHOST_PROGRESS=0|1`（進捗ログのON/OFF）
- JoinIR regression pack (integration):
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Master list (SSOT)

master list は TSV で固定する。

- File: `tools/smokes/v2/profiles/integration/joinir/planner_required_cases.tsv`
- Columns:
  - `fixture`（repo root 相対）
  - `expected`（stdout の SSOT。空stdoutは `__EMPTY__`）
  - `allow_rc`（許容 exit code の空白区切り。省略時は `0`）
  - `planner_tag`（必須。例: `[joinir/planner_first rule=LoopBreakRecipe]`）
    - pre_plan の shadow adopt が正規経路の場合は
      `[flowbox/adopt box_kind=Loop features=<...> via=shadow]` を使ってよい
  - `reason`（任意。読みやすさのための 1 行メモ）

## allow_rc policy (SSOT)

- stdout が一致していることが最優先（stdout が SSOT）。
- exit code は OS 都合で `0..255` に丸められるため、stdout が正しいが rc が非 0 のケースは `allow_rc` をケース別に持つ。
- `timeout`（`timeout` の 124）は即 FAIL（`allow_rc` で許容しない）。

## Logging policy (SSOT)

- gate は失敗時に「最後の 1 行」に `LOG: <path>` を出す（以後の切り分けの SSOT）。
- 追加の診断（dev-only）は `docs/reference/environment-variables.md` に定義し、phase doc に環境変数の仕様を複製しない。

## Release binary precondition (SSOT)

- gate は `tools/smokes/v2/lib/test_runner.sh` の `NYASH_BIN`（通常 `./target/release/hakorune`）を実行対象として使う。
- 「直したのに変わらない」事故を避けるため、gate 前に `cargo build --release --bin hakorune` を必須とする。
- Phase 29bq fast gate は冒頭で `NYASH_BIN` と `--version` を 1 行だけ表示する（デバッグ距離短縮）。
