# MIRBUILDER-SEARCH-LOOP-EXIT-RELATION-D0

Status: decided; S0 selected
Date: 2026-10-11
Scope: Normal exit source for the two ready returns of the original `SizeClassBox.size_to_bin`
Related: `mirbuilder-search-loop-early-return-home-s0-2026-10-11.md`

## Observed frontier

The unchanged `lang/src/hako_alloc/memory/size_class_box.hako` with a trivial
Main now crosses the selected Home and Static packet transport. Its physical
input stops at
`[freeze:contract][ordinary-new/local-commit/root-exit-source-missing]`
(`/tmp/hako-search-loop-home-s0-transport-physical-20261011.log`). This is
not a whole-source or EXE pass.

`validate_root_home_exit` requires a `normal_exit_projection_v1` for every
ready explicit exit. That projection requires a retained terminal relation.
`ordinary_new_coseal_issue.rs` currently filters child terminal relations
through `retain_child_terminal_relation`, with `I64Scalar` retained only when
`has_integer_return` is true. This may explain the new stop; it is a
read-only hypothesis, not a chosen implementation.

## Decision required before implementation

- Identify the exact missing exit and its original terminal relation and
  disposition; compare the before-filter and retained rows.
- Name the source authority and canonical issuer for an I64 local return at
  both early and tail exits. Determine whether the existing result-contract
  cohort already proves the class or a bounded source proof is missing.
- Keep unrelated child returns and unproved local I64 returns fail-closed.
  Do not retain all `I64Scalar` rows or infer a public contract from callers.
- Choose one bounded implementation slice with positive original-source
  first-stop and negative changed-class/exit evidence. Use a read-only worker
  before selecting the Decision.

Non-claims: no Home change, packet issuer change, generic search-loop support,
or mimalloc-lite EXE completion is selected here.

## Read-only audit and exact census

The Home terminal walker can issue the early `return bin` as `I64Scalar`.
The current child filter retains `I64Scalar` only for borrowed-formal integer
return uses; that may drop this selected local return. A bounded future fix
could join the existing Static I64 V2 product with the available `ExactI64`
result claim, then retain only its exact early source row. The later
`VerifiedStaticLoopI64ResultSourceV1` is issued after child retention, so it
cannot be used as early authority.

The observed error alone had no owner/exit. A disposable package census over
the unchanged source measured `size_to_bin` owner slot 5: early exit
`Body(2)/LoopBody(0)/IfThen(0)` has ready Home and no retained relation or
Normal projection; tail exit `Body(3)` has ready Home, a retained `Call` and
Normal projection (`/tmp/hako-search-loop-exit-d0-census-20261011.log`).
The diagnostic test was removed. The Home terminal walker issues the early
local integer return as `I64Scalar`; child retention then drops that class
unless the borrowed-formal integer-return flag is present. This source is a
selected Static Loop, not that borrowed-formal return family.

## Decision

Source authority + canonical issuer: keep the Home terminal walk as the sole
issuer of the exact `I64Scalar` relation. Join its owner, early return/value
site and proven `bin` binding with the preissued Static I64 V2 product and
the selected canonical caller's existing `ExactI64` result claim. The later
`VerifiedStaticLoopI64ResultSourceV1` stays a downstream result/ABI check.

Non-authority: method names, all child `I64Scalar` rows, all-caller class
inference, and physical return instructions do not authorize retention.

Fail-fast boundary: only the selected product's exact early row may survive
the child relation filter. Wrong owner, site, value, binding, result claim or
product refusal leaves it absent. Tail `Call` retention and unrelated child
returns keep their current behavior.

Smallest next slice: add the bounded child retention join; pin the unchanged
source's early `I64Scalar` and tail `Call` with both Normal projections,
negative changed source/class/exits and sibling refusal, then remeasure the
original physical first stop. Use existing test family and no new guard.

Non-claims: no generic local-I64 promotion, Home issuer change, whole-source
pass, or mimalloc-lite EXE pass.
