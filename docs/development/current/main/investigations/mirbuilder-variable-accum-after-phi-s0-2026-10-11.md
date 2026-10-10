# MIRBUILDER-VARIABLE-ACCUM-AFTER-PHI-S0

Status: superseded by physical route evidence; do not land this wiring
Date: 2026-10-11
Decision: `mirbuilder-bin-size-single-predecessor-phi-d0-2026-10-11.md`
Scope: canonical variable-accumulation loop after role at physical capture
Related: `mirbuilder-loop-cond-after-phi-s0-2026-10-11.md`

The unchanged `SizeClassBox.bin_size/1` has a loop header at
block 29 and after continuation at block 32. The after block begins with
three one-input PHIs from block 29 and then contains the final source
calculation/Call. Existing `PhysicalBoundary` refuses a recorded block
with such a PHI unless it is the separately selected Static Call loop body.

Issue one private-field owner+source-loop-site+sealed-after-block role from
the existing loop physicalizer continuation. `variable_accum::lower` loans
it once to the package `OrdinaryNewClaimLedgerV1` after successful source
consumption. The role includes the exact PHI prefix issued by canonical
SSA writeback, including each destination and `(predecessor, value)` row.
At selected child validation, allow only its exact after block and compare
that sealed PHI prefix before the physical boundary's CFG/finished-projection
checks. The block alone does not authorize arbitrary one-predecessor PHIs.
No generic one-predecessor permission, loop-site reconstruction from MIR,
or second authority. Multiple loops remain distinct by owner/site.

Positive acceptance: original `bin_size` passes this PHI boundary and
reaches a later named frontier; existing physical boundary and unchanged
source tests pass. Negative acceptance: absent/duplicate role,
owner/site/block drift, wrong PHI predecessor/input, mid-block PHI,
unselected loop and finished edge drift refuse. Reuse existing PHI tests;
add only independent missing cases. The `size_to_bin` search-loop Home
frontier and parent signed-immediate S0 physical acceptance remain open.

## Current evidence and correction

The first wiring records the sealed continuation role in the existing
ordinary-new ledger. A read-only audit found that block+predecessor alone
would admit a wrong input `ValueId`; the role now snapshots the canonical
PHI prefix and the capture compares it exactly. The focused negative also
mutates the input value while keeping the predecessor unchanged.
`cargo check --profile quick -p nyash-rust --lib` passes
(`/tmp/hako-loop-after-phi-s0-check-2-20261011.log`). The unchanged-source
physical probe still refuses block 32. Disposable instrumentation shows no
variable-accum role was issued and the allowed block set is empty
(`/tmp/hako-loop-after-phi-s0-physical-3-20261011.log`). Source inspection
confirms `bin_size` has leading branches and a multiply update, outside the
specialized VariableAccum route. The actual generalized loop route and
canonical PHI issuer must be identified before selecting a replacement S0.
This WIP stays uncommitted and is not a completed slice.
