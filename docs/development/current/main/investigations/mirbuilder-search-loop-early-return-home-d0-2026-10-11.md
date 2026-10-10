# MIRBUILDER-SEARCH-LOOP-EARLY-RETURN-HOME-D0

Status: design accepted; bounded S0 ready to select after LoopCond closeout
Date: 2026-10-11
Source: unchanged `SizeClassBox.size_to_bin` search loop
Scope: source/Completion design for the selected search-loop Home exit
Related: `mirbuilder-negative-integer-return-home-s0-2026-10-11.md`

The owner has two exits. The early `return bin` inside the loop has
`TerminalNotCovered`; the trailing `return me.huge_bin()` has
`PrefixNotCovered` at the loop. Current Home loop loan is for the
variable-bound multiplication shape, so this search loop needs a separate
source/Completion decision. Preserve all-exit readiness and local-call
binding consumption. Do not make the loop generic return an implicit
consumer, and do not weaken `is_empty()`.

At selection, identify the existing loop semantic facts and canonical
issuer, exact source/physical caller, normal and fault/cleanup behavior,
and focused positive/negative acceptance before construction. The
disposable per-exit census is
`/tmp/hako-bool-entry-coverage-exits-20261011.log`.

The completed signed-immediate and LoopCond-after-PHI slices move the
unchanged SizeClassBox physical probe to `IncompleteOrdinaryNewCoverage`.
Disposable coverage diagnostics report owner 5 `all_ready=Some(false)` and
its single local-call site missing from emitted exit entries; owners 4, 7,
and 10 are ready (`/tmp/hako-loop-cond-after-phi-s0-coverage-20261011.log`).
This selects the search-loop source/Completion decision without widening
the physical PHI permission.

## Decision

Source authority + canonical issuer: the resolver's exact loop, branch,
binding, call and exit sites; existing Static I64 source loans for the entry,
header/body and tail calls in `ordinary_new_loop_static_source_loan.rs`; and
the transitive `VerifiedClosedStaticLoopHomeNeutralV1` proof. Issue the
existing `produce_static_i64_loop_semantic_v2` product once before Home scan,
using the same base `VerifiedFunctionCompletionV1` that Home then consumes.
That producer already owns the only Recipe keys, verifier and Join closure;
it reads no Home or physical MIR. Home borrows only source site/binding/call
identity from the verified product. The later Static V2 caller consumes the
same product, retaining the neutral and tagged-entry corroborations. Static
source loans must be issued before Home and moved into this product once,
instead of re-issued or taken again after scanning. The previous proposal
to split its topology checks into a second source-role issuer is superseded:
it would divide one existing authority and require a new identity join.

Non-authority: raw CFG, LoopCond physical publication, aggregate coverage,
the variable-bound multiplication Loop role, method names, and a generic
single-predecessor PHI rule. None may issue the Home source role.

Fail-fast boundary: require the exact root search loop, one early `return bin`
branch and I64 `bin + 1` step, the entry/header/body/tail Static source claims
with their selected targets and sites, **exactly** the early and tail resolver
exits with no implicit exit, and the closed
transitive Home-neutral proof. Extra exits, break/continue, changed binding,
missing or non-I64 claims, or ownership-changing callees remain unready.
The existing `TerminalNotCovered`/`PrefixNotCovered` result remains the stop;
do not weaken `is_empty()` or synthesize a return.

Smallest next slice: issue and transport the one verified Static I64 product
before Home scan. Give the Home Loop arm a selected walker: observe the header condition
and early-return branch with existing scalar/If walkers, prove the I64 step
preserves Home, then continue after the loop to the existing tail terminal
walker. Each exit keeps its own `RootHomeExitV1` and covered call sites.
Normal exits are proved separately; Fault propagation keeps the existing
call boundary and neutral Home effect. No physical LoopCond changes belong
to this slice.

Acceptance: unchanged `SizeClassBox.size_to_bin` has both early and tail
exit Home rows ready with the right covered call sites. The original
SizeClassBox probe moves to a later named first stop. Focused negatives
reject an extra exit/break/continue, wrong binding or step, missing Static
claim or non-I64 target, Home-changing callee, and mismatched owner/site.
The existing all-exit and unrelated loop refusal checks remain green.

Non-claims: no whole SizeClassBox or mimalloc-lite EXE success; Static V2
physical publication remains a separate possible frontier.

## S0 transport clarification (2026-10-11)

The selected product owns the entry, body and tagged source loans after
preissue. Existing packet and body-scalar issuers run after that move and
formerly enumerated the depleted loan maps. They must also borrow the
selected product's original loans and run their existing issuance checks.
The product remains the sole source owner; packet and body-scalar proofs
remain separate canonical issuers. Reject duplicate owner/site keys or a
product whose owner, loop site or tagged source disagrees. This repairs the
S0 transport regression before treating any later physical stop as a new
frontier.
