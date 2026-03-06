# JoinIR Pattern Selection Shadow SSOT

> Status: retired (2026-03-05, commit `0df74eaa5`)
>
> `planner/pattern_shadow.rs` と `planner/candidates.rs` は single-plan 境界化で撤去済み。
> 本書は履歴参照用。現行の planner trace は `trace_try_take_planner` を参照。

## Overview

このモジュールは **shadow-only / diagnostic** だった（現在は撤去済み）。

- 既存の push順序は維持（push順序がSSOT）
- priority表は診断用（未知ruleは255でOK、ドリフトしても問題なし）
- ログは `[plan/trace]` タグを使用（新タグ追加禁止）
- ログは `trace.rs` 経由で `joinir_trace_enabled()` ガード
- 1行出力のみ・安定フォーマット

## Design Rationale

### Why Shadow-Only?

パターン選択の挙動を変えずに、将来の自動解決のための可観測性を追加する。

- **挙動不変**: 実際の選択は `build.rs` の push順序で決まる
- **診断目的**: 複数候補（ambiguous）時に「priority表ならどれを選ぶか」をログ出力
- **ドリフト許容**: priority表は厳密である必要がない（未知ruleは255で良い）

### Log Format

retired（現行では出力しない）。

## Implementation

### Files

- removed: `src/mir/builder/control_flow/plan/planner/pattern_shadow.rs`
- removed: `src/mir/builder/control_flow/plan/planner/candidates.rs`
- current trace entry: `src/mir/builder/control_flow/plan/trace.rs` (`trace_try_take_planner`)

### Priority Table (Diagnostic Only)

priority表は参考情報であり、SSOTではない。実際の選択はpush順序で決まる。

| TIER | Priority | Examples |
|------|----------|----------|
| 1 | 10-19 | scan_with_init, split_scan |
| 2 | 20-29 | loop_break, if_phi_join, loop_continue_only |
| 3 | 30-39 | bool_predicate_scan, accum_const_loop |
| 4 | 40-49 | pattern1_* variants |
| 5 | 50-59 | v0 fallbacks (flag_exit, scan_phi_vars, etc.) |
| 6 | 60-69 | recipe block (loop_cond_break_continue, scan_methods_v0) |
| 7 | 70-79 | loop control variants |
| 8 | 80-89 | generic fallbacks |
| Unknown | 255 | any unrecognized rule |

## Activation

```bash
# retired: no output expected
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune program.hako 2>&1 | grep "pattern_shadow" || true
```

## Drift Check

### ファイル存在確認

```bash
rg -n "pattern_shadow|candidates.rs" src/mir/builder/control_flow/plan/planner
```

### ログ出力確認

```bash
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune program.hako 2>&1 | grep "stage=try_take_planner"
```

## Reference

- Planner trace SSOT: `src/mir/builder/control_flow/plan/trace.rs`
