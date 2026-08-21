Status: accepted design; caller-zero C-prime task series selected
Task: MIR-EMIT-CANONICAL-STRICTNESS-D0
Date: 2026-08-22
Priority: Medium-High
Owner: canonical Loop operation physicalization and the ordinary MIR writer
NextCard: MIR-LOOP-OPERATION-EMITTER-SPLIT-S0
---

# MIRBuilder canonical CompareI64 strictness D0

## Six-line brief

Decision: Accept C-prime: admit only `CompareI64` operands already defined as
exact Integer values in the same canonical target block, then append the
Compare at that block's tail. General cross-block dominance remains closed.

Source authority + canonical issuer: the verified Loop operation program owns
the logical Compare and schedule; `CanonicalSsaFunctionSessionV2` must lend one
scoped service containing its owner, CFG, SSA, and PHI owners. Inside that loan,
private `CanonicalLoopCompareI64IssuerV1` is the sole co-sealer and immediate
consumer of the physical proofs.

Non-authority: `current_block`, `ensure_block_exists`, raw `ValueId`, type
facts alone, ledger order alone, names/AST, `compute_def_blocks`,
`compute_dominators`, generic Compare success, debug verification, and a test
canary cannot admit canonical emission or production selection.

Fail-fast boundary: open-target, full operand receipt, unique same-block
definition, exact Integer type, destination, Bool plan, strict append plan, and
vacant result slot are all checked before append. Result-slot reservation is
the final fallible action; append, Bool commit, and ledger commit are then
infallible inside the private issuer.

Smallest next slice: behavior-neutrally extract the pure Const/Binary/Compare
family from the 794-line `operation_emitter.rs`; preserve every call and error
boundary and reduce both owners below the 760-line split threshold.

Non-claims: no production caller, cross-block operand, general dominance,
parameter/inherited operand, Call/receiver, PHI redesign, Recipe change,
legacy retirement, backend, optimization, `EmitReceipt`, or main integration.

## Decision and current disposition

The selected shape is:

```text
CANONICAL-LOOP-COMPARE-I64-SAME-BLOCK-C-prime

verified Loop CompareI64 row
  + exact logical-to-physical target receipt
  + same canonical session's open-target proof
  + full Published lhs/rhs Loop receipts
  + unique physical lhs/rhs definitions already in that target block
  + exact Integer type
  + fresh destination capability
  + prepared Bool publication
  + prepared strict append
  + result slot reserved last
  -> one ordinary-writer append
  -> infallible Bool commit
  -> infallible Published ledger commit
```

This is narrower than general dominance, but it remains sound while a Loop
header is open. A definition already present in a block remains before an
instruction appended to that same block even if another predecessor is added
later. The design therefore does not need to predict the future predecessor
set.

The current disposition is `CallerZeroCanaryOnly`, not production I0. The
existing census at
`loop-recipe-physicalizer-caller-census-d0-2026-08-21.md` is still binding:

- `resolved_lowering/mod.rs:36-37` admits
  `generic_g0_physical_emitter_session` only under `#[cfg(test)]`;
- `resolved_lowering/mod.rs:141-142` admits the common Loop physicalizer only
  under `#[cfg(test)]`;
- every call to
  `with_generic_g0_physical_emitter_session_preflight()` is inside that file's
  test module;
- the callable and Generic G0 production-named canaries are tests, not named
  production consumers;
- `lower_loop_generalized` remains a distinct production route and is not
  claimed replaceable by this card.

Consequently the implementation ladder is `S0 -> P0 -> CONNECT0`. The token
`I0/R0` is reserved for a later card that names and switches a real non-test
caller and removes its exact old edge.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `PreparedLoopOperationProgramV1` and verified row | Compare operation, lhs/rhs/result keys, value class, Recipe order | physical IDs, target choice, dominance |
| `LoopPhysicalSegmentBlockReceiptV1` and `VerifiedLoopOperationTargetBlockV1` | exact logical/segment/role to physical-block relation | open/sealed state, operand definition |
| `CanonicalCfgSessionV1` | canonical block creation, open/sealed/terminated observation | operation meaning, operand type |
| `CanonicalSsaFunctionSessionV2` | function owner, canonical value allocation, scoped access to CFG/SSA/PHI owners | source reclassification, legacy repair |
| `LoopOperationValueLedgerV1` | operation result transport and one-shot result slot | physical truth by itself, Binding SSA |
| `PreparedCanonicalCompareBoolTypeV1` | prechecked Compare-result Bool publication | operand or placement admission |
| `CanonicalLoopCompareI64IssuerV1` | one atomic co-seal and immediate strict consumption | target selection, fallback, general dominance |
| ordinary MIR writer core | one checked instruction append and common post-success bookkeeping | repair, source inference, ledger publication |
| outer unpublished function session | discard of a failed candidate | retry in another route |

The new issuer creates no semantic fact. It combines existing semantic,
placement, and session-local physical authorities for one operation. Its
prepared capability is private and is not returned or stored.

## Same-session service boundary

The current `LoopOperationDispatchServicesV1::new(builder, identity, phis)`
can manually pair sibling borrows and does not carry the canonical CFG owner.
The strict path must instead be entered through a scoped loan issued by the
full canonical session, conceptually:

```rust
CanonicalSsaFunctionSessionV2::with_loop_operation_dispatch_services(
    builder,
    |services| { /* prepare and immediately consume strict Compare */ },
)
```

The service borrows the one session's owner, `cfg`, `identity`, and `phis`.
The constructor is private to the session. Open-target and operand witnesses
do not escape the callback, so no digest, pointer, raw owner ID, or public
session token is needed to re-pair them later.

`CanonicalCfgSessionV1` currently records only sealed blocks and its
`create_block()` takes `&self`. C-prime requires it to record the exact blocks
it created and to issue an open-instruction-target witness only when:

```text
block is in this session's created set
block exists in the current function
block is absent from the session sealed map
the MIR block is unsealed
the MIR block is unterminated
the Loop target receipt names that exact block
```

The function entry/preheader is not silently admitted as session-created. The
first cohort targets only segment blocks allocated through
`LoopPhysicalServicesV1 -> CanonicalCfgSessionV1::create_block()`.

## Operand proof

The ledger must return the full `LoopOperationValueReceiptV1`, not only the raw
`ValueId`. The private operand issuer checks:

```text
receipt owner == canonical session owner
receipt class == I64
receipt physical block == open Compare target
type_ctx[value] == Integer
value is not a function parameter
exactly one physical definition of value exists in the function
that definition is a Phi head or ordinary instruction in the target block
that definition is already present before the tail append
```

The issuer validates lhs and rhs in one function scan and captures the target
tail ordinal before that scan. `lhs == rhs` is an admitted alias and requires
one unique definition, not two. The definition locator uses
`MirInstruction::dst_value()` and retains the block plus
`PhiHead | Instruction(ordinal)` lane. It must detect zero and duplicate
definitions; it must not call the existing block-only
`compute_def_blocks()` map, which overwrites duplicates and loses instruction
order.

This one-scan-per-Compare design is acceptable only in caller-zero P0
evidence. It is not a
compile-time performance keeper. Before production I0, an unbounded Compare
count or measured repeated-scan cost requires a function-owned definition
index or producer-issued physical-definition receipt with an explicit
mutation boundary. Pass fusion and a general dominance cache are not opened by
this card.

## One-shot ledger and commit order

The selected segment dispatcher owns `LoopOperationValueLedgerV1` by value and
returns it only in `CompletedLoopSegmentProgramV1`. That is the bounded owner
for this design. The older dispatcher that accepts a caller-owned `&mut`
ledger is not connected to strict Compare by this series.

The strict ledger state is:

| Before | Operation | After | Fallible |
| --- | --- | --- | --- |
| absent | `reserve_result()` | `Reserved` plus affine pending token | yes |
| `Published` | `reserve_result()` | unchanged reject | yes |
| `Reserved` | `reserve_result()` | unchanged reject | yes |
| `Reserved` | pending `commit(definition)` | `Published` | no |
| `Reserved` | pending token dropped | `Poisoned` | no |
| `Poisoned` | read or reserve | terminal reject | yes |

`PendingLoopValuePublishV1` is non-Clone, holds the exact mutable slot, and
uses an internal `Option` only to distinguish its own consumed Drop state.
`commit(mut self, definition)` takes the slot, writes `Published`, and cannot
revalidate or fail. An unconsumed token poisons the ledger; it never rolls the
slot back to vacant. Recovery is disposal of the outer unpublished candidate.

All fallible work is ordered as follows:

```text
operation/target relation
-> same-session open target
-> full lhs/rhs receipts
-> unique same-block Integer definitions
-> wrapped fresh destination
-> prepared Bool publication
-> prepared strict writer input
-> reserve result slot                # last fallible action
-> append Compare                     # no Result
-> commit Bool                        # no Result
-> commit ledger                      # no Result
```

No result type reread, duplicate publication check, receipt-count check,
legacy retry, or ordinary Compare helper call is permitted after append.

## Finite state table

| State | Owner | Meaning | Allowed next state | MIR effect |
| --- | --- | --- | --- | ---: |
| `Unprepared` | Loop row owner | operation not physically co-sealed | `TargetReady` or reject | 0 |
| `TargetReady` | canonical CFG session | exact target is session-created, open, and unterminated | `OperandsReady` or reject | 0 |
| `OperandsReady` | canonical SSA session | lhs/rhs are unique same-block Integer definitions | `ResultReady` or reject | 0 |
| `ResultReady` | canonical SSA/type/writer preparation | destination, Bool plan, and append plan complete | `LedgerReserved` or reject | destination reservation only |
| `LedgerReserved` | affine ledger token | result key is reserved; no fallible operation remains | `Committed` | 0 instructions |
| `Committed` | ordinary writer plus infallible commits | one Compare, one Bool fact, one Published row | terminal | 1 instruction |
| `RejectedBeforeEffect` | named preparation owner | missing, foreign, stale, duplicate, or unsupported input | outer discard | 0 instructions |
| `Poisoned` | dropped pending token | internal prepared sequence was abandoned | outer discard only | continuation forbidden |

The destination counter may advance during preparation inside the unpublished
function draft. A later pre-append reject therefore still requires outer draft
discard; it never permits local continuation or fallback.

## Sole ordinary-writer connection

The strict path must not call `emit_instruction_at()`, because that path can
select `current_block`, call `ensure_block_exists`, materialize MethodCall
receivers, normalize PHI inputs, and create branch targets.

`builder_emit.rs` must expose one private ordinary append core used by both:

```text
legacy repair-capable front door --prepared by existing behavior--\
                                                               -> append core
strict Compare front door -------PreparedCanonicalCompare-----/
```

The append core remains the only ordinary `add_instruction_with_span` owner
and shares the required Compare metadata/origin post-success behavior. Existing
specialized canonical CFG marker/terminator writers are outside this ordinary
instruction scope and are not duplicated or migrated here.

## Alternatives

| Choice | Decision | Reason |
| --- | --- | --- |
| A: general cross-block dominance | defer | an open target can gain predecessors; safe issuance needs a complete future-edge contract, CFG epoch, and stable definition provenance |
| B: Loop-only dominance/verifier utility | reject | creates a shadow physical authority; current def/dominator helpers lose duplicate and same-block order information |
| C-prime: unique same-block definition plus tail append | accept | stable for open headers, matches current caller-zero recipes, and closes one bounded physical law |

## Ordered task series

| Order | Row | Class | Bounded change | Exit evidence |
| ---: | --- | --- | --- | --- |
| 1 | `MIR-LOOP-OPERATION-EMITTER-SPLIT-S0` | BoxShape | extract the existing pure Const/Binary/Compare owner; no behavior change | parent and child below 760; focused parity |
| 2 | `MIR-LOOP-COMPARE-SESSION-TARGET-P0` | caller-zero physical contract | scoped session service, CFG-created set, and private open-target witness | foreign/uncreated/sealed/terminated targets reject pre-effect |
| 3 | `MIR-LOOP-COMPARE-SAME-BLOCK-OPERANDS-P0` | caller-zero physical contract | one atomic lhs/rhs scan, unique same-block I64 witnesses, and wrapped destination | missing/duplicate/cross-block/parameter/type negatives reject pre-append |
| 4 | `MIR-LOOP-COMPARE-LEDGER-RESERVATION-P0` | caller-zero physical contract | add reserve/commit/poison and store operation+target as one prepared segment row | reserve is last; row/target zip and post-append count Result are zero |
| 5 | `MIR-LOOP-COMPARE-STRICT-WRITER-P0` | caller-zero physical contract | one ordinary append core, prepared Bool, and private strict Compare input | strict route has no repair and all post-append commits are infallible |
| 6 | `MIR-LOOP-COMPARE-SAME-BLOCK-CONNECT0` | caller-zero connection | switch only the segment Compare leaf; leave older caller-owned dispatcher and generic helpers unchanged | selected old segment Compare edge zero; no fallback; canaries green |
| 7 | `MIR-LOOP-COMPARE-I0-R0` | future production replacement | name one non-test caller, prove its cohort, switch it, and retire its exact old edge | production caller one, old edge zero, fallback zero |

Only row 1 is selected now. Later cards are created when their immediate
predecessor closes; this table is the order SSOT, not permission to batch all
changes into one commit.

## Line budget

Current measured owners require physical separation before growth:

| File | Lines at Decision | Rule |
| --- | ---: | --- |
| `operation_emitter.rs` | 794 | row 1 must split before semantic edits |
| `canonical_cfg/session.rs` | 718 | new proof logic goes in a child module; parent stays below 760 |
| `canonical_ssa/session.rs` | 727 | new proof logic goes in child modules; parent stays below 760 |
| `operation_dispatcher.rs` | 636 | tests and new prepared-row owner stay separate before 760 |
| `builder_emit.rs` | 507 | split at 760, hard stop at 800 |

No compression, diagnostic shortening, or unrelated cleanup may be used to
cross the limit.

## Acceptance and structural guards

The completed caller-zero series must prove:

```text
non-test Loop physicalizer caller                          = 0
strict segment Compare -> emit_compare_i64_at              = 0
strict segment Compare -> general emit_instruction_at      = 0
strict path -> ensure_block_exists / LocalSSA / PHI repair = 0
strict path -> compute_def_blocks / compute_dominators     = 0
canonical reject -> legacy fallback/retry                  = 0
result reservation                                        = 1
result commit                                             = 1
post-append fallible type/ledger/count checks              = 0
ordinary append core                                      = 1
AST/name/raw-pointer/Recipe reconstruction                 = 0
touched Rust source                                       < 760 lines
hard stop                                                 < 800 lines
```

Positive evidence covers Header-local provisional PHI or read plus
Header-local Const, followed by one Header Compare, Bool fact, Published row,
and instruction. Negative evidence covers missing/duplicate/cross-block/
parameter/wrong-type definitions, foreign owner/target, uncreated/sealed/
terminated target, duplicate/reserved/poisoned result, Bool conflict, and
unconsumed pending token. Every preparation rejection checks unchanged
instruction count, type facts, and non-reserved ledger state.

## NoSafeSlice triggers

Stop before the affected row when any of these is true:

- a real selected caller requires a parameter, inherited, or cross-block
  operand;
- a Header-local Binding read does not physically define a PHI/read result in
  the target before Compare;
- the target cannot be tied to the same CFG session's created set;
- the scoped service cannot prevent sibling session re-pairing;
- the result slot cannot be reserved last or committed infallibly;
- strict Compare must enter `emit_instruction_at()` or hidden repair;
- any type, ledger, schedule, metadata, or receipt check remains fallible after
  append;
- a canonical reject can reach legacy, retry, or another profile;
- the segment ledger ceases to be owned by the unpublished candidate;
- a touched source reaches 760 without a named split or 800 at all;
- production promotion would make repeated definition scans an unbounded
  compile-cost path without a definition-index/receipt decision.

If production later needs cross-block operands, return to A with a complete
future-edge plan, CFG epoch, stable definition provenance, and a general
dominance witness. Do not weaken C-prime or route through B.

## Non-claims and references

This Decision does not activate the test-only physicalizer, replace
`lower_loop_generalized`, add a source shape, or claim performance. It does
not change Script A/C, assignment, Recipe/Join, backend, publication, or main
integration.

References:

- `loop-recipe-physicalizer-caller-census-d0-2026-08-21.md`
- `mirbuilder-post-audit-follow-up-queue-2026-08-21.md`
- `src/mir/builder/resolved_lowering/canonical_cfg/README.md`
- `src/mir/builder/README.md`
