# JoinIR Pattern Selection Shadow SSOT

## Overview

このモジュールは **shadow-only / diagnostic** であり、挙動を変えない。

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

```
[plan/trace] pattern_shadow pick=<rule> from <N> candidates
```

例:
```
[plan/trace] pattern_shadow pick=loop/pattern2_break from 2 candidates
```

## Implementation

### Files

- `src/mir/builder/control_flow/plan/planner/pattern_shadow.rs` - shadow判定ロジック
- `src/mir/builder/control_flow/plan/trace.rs` - ログ出力（SSOT）
- `src/mir/builder/control_flow/plan/planner/candidates.rs` - 呼び出し元

### Priority Table (Diagnostic Only)

priority表は参考情報であり、SSOTではない。実際の選択はpush順序で決まる。

| TIER | Priority | Examples |
|------|----------|----------|
| 1 | 10-19 | scan_with_init, split_scan |
| 2 | 20-29 | pattern2_break, pattern3_ifphi, pattern4_continue |
| 3 | 30-39 | pattern8_bool_predicate_scan, pattern9_accum_const_loop |
| 4 | 40-49 | pattern1_* variants |
| 5 | 50-59 | v0 fallbacks (flag_exit, scan_phi_vars, etc.) |
| 6 | 60-69 | recipe block (loop_cond_break_continue, scan_methods_v0) |
| 7 | 70-79 | loop control variants |
| 8 | 80-89 | generic fallbacks |
| Unknown | 255 | any unrecognized rule |

## Activation

```bash
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune program.hako 2>&1 | grep "pattern_shadow"
```

## Drift Check

### ファイル存在確認

```bash
rg -n "pattern_shadow" src/mir/builder/control_flow/plan/planner
```

### ログ出力確認

```bash
HAKO_JOINIR_DEBUG=1 ./target/release/hakorune program.hako 2>&1 | grep "pattern_shadow"
```

## Reference

- Push順序SSOT: `src/mir/builder/control_flow/plan/planner/build.rs`
- Trace SSOT: `src/mir/builder/control_flow/plan/trace.rs`
