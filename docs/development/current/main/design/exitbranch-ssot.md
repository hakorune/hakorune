---
Status: SSOT
Scope: ExitBranch feature (compiler cleanliness / BoxShape)
Related:
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/joinir-design-map.md
- src/mir/builder/control_flow/plan/REGISTRY.md
---

# ExitBranch (SSOT)

Goal: If/BranchN/Loop 内の “exit 付きブランチ” を共通 feature として抽出し、
exit_if/match/loop の重複を減らす（完成品キット化を防ぐ）。

Non-goals:
- 受理形の拡大
- AST rewrite（見かけ等価変形）
- selfhost workaround

## Responsibility

- Exit を伴う branch の形を **共通表現**として返す。
- 呼び出し側（pipeline/ops）は ExitBranch の出力だけを見る（再判定しない）。
- 既定挙動は不変。strict/dev + planner_required のみで拡張する。

## Interface (concept)

ExitBranch should provide:
- exit kind (return/break/continue)
- payload (value / carrier inputs)
- join requirement (needs join / no join)

The caller owns:
- lowering / wiring
- verifier checks
- logging format (SSOT: planner entry guards)

## Logging

- `[plan/reject]` / `[plan/accept]` の契約に従う（新タグ追加はしない）
- 1行・固定タグのみ（debug guard）

## Gates

T0 (docs-only): no gate.
Implementation: `phase29bq_fast_gate_vm.sh` を必須。

## Implementation Inventory (T0 / docs-only)

目的: “exit 付きブランチ” の生成点が散っていると SSOT が崩れるため、追加点を 1 枚で辿れるように棚卸しする。

### CoreExitPlan 生成・利用ポイント（概観）

注: 行番号は drift しやすいので、基本は `rg "CoreExitPlan::|ContinueWithPhiArgs|LoopStepMode::"` で追う。

| 場所 | 行 | Exit種別 | 状態 |
|---|---|---|---|
| `parts/stmt.rs` | - | return prelude lowering | **(基準: prelude)** |
| `parts/exit.rs` | - | break/continue/return with PHI args | **(基準: exit)** |
| `features/exit_branch.rs` | - | (delegate-only) | 互換委譲のみ |
| `features/exit_if_map.rs` | - | (exit_branch経由) | 使用済み ✓ |
| `features/exit_map.rs` | 135,141 | Break, Continue | **移設済み ✓** |
| `composer/branchn_return.rs` | 53,63 | Return(Some) | **移設済み ✓** |
| `features/generic_loop_body.rs` | 201,218,262,880,907,908,916 | Break/Continue/Return (incl ExitIf) | **完全移設済み ✓** |
| `features/loop_true_break_continue_pipeline.rs` | 92,93 | Break, Continue | **移設済み ✓** |
| `features/loop_cond_break_continue_pipeline.rs` | 171,401 | ContinueWithPhiArgs ✓, Break ✓ | **Exit生成 完全移設済み ✓** |
| `features/loop_cond_continue_with_return_pipeline.rs` | 458,638 | Return ✓, ContinueWithPhiArgs ✓ | **Exit生成 完全移設済み ✓** |
| `features/loop_cond_return_in_body_pipeline.rs` | 139,320 | ContinueWithPhiArgs ✓, Return ✓ | **Exit生成 完全移設済み ✓** |
| `features/conditional_update_join.rs` | 273,287,374,380 | Continue, Break, ContinueWithPhiArgs | **移設済み ✓** |

#### 重複パターン分類

1. **Exit生成 完全移設済み**: exit_if_map, exit_map, branchn_return, generic_loop_body, conditional_update_join, loop_cond_break_continue, loop_cond_continue_with_return, loop_cond_return_in_body, loop_true_break_continue

### Decomposition Candidate (T1 memo)

**完了: `composer/branchn_return.rs` の return-only branch を ExitBranch へ移設**

- 目的: match-return の Return 生成を “共通 feature” 側に寄せ、将来の prelude 付き return 等の拡張点を 1 箇所に閉じる。
- 注意: 受理形拡大はしない（BoxShape のみ）。
