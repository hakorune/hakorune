# 291x-747 Compiler Cleanliness Worker Inventory Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- worker inventory for the remaining compiler-cleanliness shelves after 291x-746
- `docs/development/current/main/CURRENT_STATE.toml`
- private current JoinIR architecture notes that still described removed normalizers as live
- first low-risk cleanup pass: JoinIR lowering `common` dead helper surface

## Why

291x-746 marked the end of the obvious dead-shelf burst. Before deleting more
code, the remaining surfaces were inventoried by area so cleanup does not mix
real structure vocabulary with unused helper APIs.

## Worker Inventory Result

| Area | Immediate cleanup | Structural hold / reconcile |
| --- | --- | --- |
| JoinIR lowering | `condition_pattern`, `common::dual_value_rewriter`, tiny common helpers, test-only `JoinValueSpace` helpers | `condition_lowering_box`, `condition_to_joinir`, `update_env`, `JoinValueSpace` wrapper API decision |
| Planner facts | `CleanupKindFacts::{Break,Continue}`, possibly old cfg-test trim/condition helpers | `LoopFacts::condition_shape`, `SplitScanFacts::shape`, `CleanupKindFacts::Return`, `SkeletonKind::{If2,BranchN}` |
| VM bridge / routing | stale env/comment names, strict-env split, LowerOnly strict semantics | target registries are live policy SSOTs even though they live under bridge dispatch |
| Docs/checks | public current docs/checks have no required stale references | private current architecture notes needed deleted-normalizer sync |

## Decision

Start with the low-risk JoinIR lowering common-helper prune:

- delete `common::dual_value_rewriter`
- delete `common::has_array_method`
- delete `common::has_loop_increment`
- update the common README and stale comments that pointed at those helpers

Do not include `condition_pattern`, `update_env`, planner fact vocabulary, or
bridge route semantics in this same card. Those need separate reconcile cards.

## Landed

- Removed `src/mir/join_ir/lowering/common/dual_value_rewriter.rs`.
- Removed `pub mod dual_value_rewriter` from `common.rs`.
- Removed unused `has_array_method` and `has_loop_increment` wrappers.
- Removed stale comments that pointed future work at the deleted array-method helper.
- Updated `common/README.md` to mark name-based dual-value rewrites as retired.

## Remaining Queue

Near-term deletion/test-surface cards:

1. `condition_pattern` test-only or delete decision
2. `JoinValueSpace` extra helper narrowing
3. `CleanupKindFacts::{Break,Continue}` deletion
4. cfg-test trim/condition helper quarantine

Structural cards:

1. `condition_lowering_box` trait ownership vs `ExprLowerer + condition_lowerer`
2. `condition_to_joinir` facade imports vs direct module ownership
3. `update_env` vs `ScopeManager`
4. planner condition/skeleton vocabulary and FlowBox observability
5. bridge strict/LowerOnly return semantics and env naming

## Proof

- `rg -n "dual_value_rewriter|has_array_method|has_loop_increment|rewrite_break_condition_insts|try_derive_looplocal_from_bodylocal_pos|try_derive_conditiononly_is_from_bodylocal_pos" src tests docs/development/current/main docs/private/development/current/main -g '*.rs' -g '*.md'`
  - current hits are phase history/archive or this card only
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
