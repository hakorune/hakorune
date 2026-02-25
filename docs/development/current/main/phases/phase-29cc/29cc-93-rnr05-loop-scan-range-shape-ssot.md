---
Status: Done (RNR-05 min1/min2/min3)
Decision: provisional
Date: 2026-02-25
Scope: RNR-05 で固定する parser/plan 共通の loop-scan range shape 契約（1 shape）
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-92-non-plugin-rust-residue-task-set.md
  - docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md
  - src/parser/statements/control_flow.rs
  - src/mir/builder/control_flow/plan/loop_scan_v0/facts.rs
---

# 29cc-93 RNR-05 Loop-Scan Range Shape SSOT

## Purpose

- RNR-05 の BoxCount 増分を 1 shape に固定する。
- parser 側（min1）と plan 側（min2）の判断源を同じ shape 定義に揃える。
- fail-fast の reject/accept 境界を先に文書化し、後段実装で迷走しないようにする。

## Target Shape (single)

- shape id: `rnr05.loop_scan.range_v0`
- source form (parser-facing):
  - `while i <= n - 1 { ... }`
- canonical intent (plan-facing):
  - `loop(i < n)` と等価な 1-counter upper-bound 形

## Parser Contract (min1)

- 受理する最小形:
  - `while` の condition が `i <= n - 1` を保持すること
- 期待 AST（観測契約）:
  - `While.condition = BinaryOp(LessEqual, Variable(i), BinaryOp(Subtract, Variable(n), Int(1)))`
- 非目標:
  - parser で `<= n - 1` を `< n` に rewrite しない
  - parser 側で別の range 形（`<= n`, `<= n - 2`, `>=` など）を同時追加しない

## Plan Contract (min2)

- `loop_scan_v0` facts が受理する condition は次の2形のみ:
  1. `i < n`
  2. `i <= n - 1`
- reject する代表形:
  - `i <= n`
  - `i <= n - 2`
  - `i < n - 1`
- fail-fast 方針:
  - 受理外 condition は `loop_scan_v0` facts で `None` を返し、上位 router に委譲する
  - silent fallback で別 shape を混ぜない

## Gate / Test Pin

- parser pin:
  - `cargo test parser_loop_scan_range_shape_preserves_lte_n_minus_one_ast -- --nocapture`
- plan pin:
  - `cargo test loop_scan_v0_accepts_lte_n_minus_one_shape -- --nocapture`
  - `cargo test loop_scan_v0_rejects_lte_n_without_minus_one -- --nocapture`
- fixture pin:
  - `apps/tests/phase29bq_joinir_scan_loop_range_lte_minus1_min.hako`
  - `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` case id:
    - `scan_loop_v0_lte_n_minus1_min`
- lane quick:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only scan_loop_v0_lte_n_minus1_min`
  - `cargo check --bin hakorune`
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## Commit Boundary (RNR-05)

1. min1: parser shape pin（AST観測契約のみ）
2. min2: plan single-point extension（facts condition 1箇所のみ）
3. min3: fast gate + fixture pin（lane gate 同期）
