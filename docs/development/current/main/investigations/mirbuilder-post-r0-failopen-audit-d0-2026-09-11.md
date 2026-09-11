---
Status: accepted__MirBuilderFailOpenRejectGates__2026-09-11
Date: 2026-09-11
Decision: MIRBUILDER-FAILOPEN-REJECT-GATES-D0
Parent: mir-callable-loop-local-completion-d0-2026-09-11
---

# MIRBUILDER-FAILOPEN-REJECT-GATES-D0

Implementation row: `MIRBUILDER-FAILOPEN-REJECT-GATES-R0`.

## Six-line brief

```text
Decision: close two fail-open paths at their existing owners; no incoming PHI
  repair/drop and no missing loop carrier may reach PHI construction.
Source authority + canonical issuer: CoreIfJoin/apply_if_joins owns PHI rows;
  phi_input_materializer::for_pred owns edge rematerialization; variable_ctx
  variable_map owns loop carrier initial values.
Non-authority: if_join pre_val substitution, branch exclusion, debug gating,
  and carrier_phis length as evidence of carrier completeness.
Fail-fast boundary: pass each reaching incoming unchanged to for_pred and reject
  its failure; reject a missing carrier in prepare before any PHI allocation.
Smallest next slice: two reject-only code edits, focused negative tests,
  REGISTRY/features/loop_true_break_continue README updates.
Non-claims: no new semantic receipt, fallback, route promotion, backend change,
  broad GenericLoop rewire, canonical_ssa BoxShape cleanup, or legacy deletion.
```

## Authority and bounded acceptance

`apply_if_joins` must preserve the `CoreIfJoin` incoming and reaching-branch
truth until `phi_input_materializer::for_pred` has accepted the exact edge. The
materializer may issue its existing canonical rematerialization; an impossible
or non-rematerializable incoming is an error before PHI publication. It must
not be replaced with `pre_val` or silently remove the branch. Missing reaching
predecessor information is likewise an error.

`LoopTrueBreakContinuePhiMaterializer::prepare` must reject a carrier absent
from `variable_ctx.variable_map` using the supplied `error_prefix`, before
allocating carrier or step PHI destinations. Later maps are not a completeness
receipt and remain downstream products of this preparation.

Focused acceptance is one negative for each boundary, one positive proving an
existing rematerializable edge remains accepted, and the existing
`phase29bq_loop_true_multi_break_planner_required_vm.sh` positive. Existing
strict/debug diagnostics remain diagnostics; they do not define the reject
boundary.

## Callable Local scope clarification

The R0 callable Local handoff constructs
`CompletedLocalBindingV1::new(ordinal, value, value)`: the plan value is reused
as the local value because this slice owns no physical Local copy. The existing
`CallableDynamicOriginLedger::record_local` rejects that shape whenever a
dynamic-origin local expectation exists (`local == initializer`). Therefore
the effective accepted scope is source-backed callable locals with no dynamic
origin expectation. Dynamic-origin locals are a named non-claim and must not be
promoted by inference or a fallback; a future slice needs an authority-issued
distinct physical local value before opening them.

## Recursive After condition rule

`recursive_after.rs` adopts the first `LoopOperationV1::CompareI64` row's left
operand as the canonical condition input (`operation_rows().iter().find_map`).
This is an explicit bounded recipe rule, not a search for the source loop
condition and not permission to pair an arbitrary later CompareI64. A broader
condition relation requires a new Facts/Recipe authority and design stop.

## Deferred structural candidate

The absence of a dedicated `canonical_ssa/` README/must-not contract and the
V1/V2 plus S6C state accumulation are recorded as a later BoxShape census. They
are outside R0 and do not authorize a rename, merge, or owner move here.

## R0 implementation receipt (2026-09-11)

`if_join::apply_if_joins` no longer computes a release-only dominance fallback:
each reaching incoming and predecessor is preserved until the existing
`phi_input_materializer::for_pred` accepts it. A missing predecessor now reaches
the existing error boundary, and a non-rematerializable non-dominating incoming
cannot be replaced by `pre_val` or dropped with its branch.

`LoopTrueBreakContinuePhiMaterializer::prepare` now consumes its
`error_prefix` and rejects a carrier missing from `variable_ctx.variable_map`
before allocating carrier or step PHI destinations. The registry, feature
README, and route README record these invariants.

Focused evidence is green: `if_join::tests` 6/6, the missing-carrier negative
1/1, the existing rematerializable edge positive 1/1, and the callable Local R0
positive/omission negative 1/1 each. `cargo test --profile quick --lib ...
--no-run --jobs 4` succeeds with the known 492 whole-library warnings.
The repository-wide `cargo fmt --all -- --check` still reports unrelated
pre-existing formatting drift; the three modified Rust files pass a targeted
`rustfmt --edition 2021 --check`.

The modified Rust files remain below the 760-line split threshold and 800-line
hard stop (`if_join.rs` 536; carrier materializer 209). No new receipt,
fallback, route promotion, backend change, canonical_ssa cleanup, dynamic
origin expansion, or legacy retirement is claimed.
