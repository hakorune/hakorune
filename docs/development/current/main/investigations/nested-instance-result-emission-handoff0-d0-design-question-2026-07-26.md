---
Status: accepted design / execution handoff
Date: 2026-07-27
Decision: NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-prime-r1
Closes: NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-D0
Selected bridge: constrained C-prime
First executable row: NESTED-INSTANCE-RESULT-EMISSION-CORRESPONDENCE0-P0
Precondition: NESTED-INSTANCE-RESULT-CONTRACT0-S0 closed
Blocked umbrella: CALLABLE-RESULT-NESTED-REP0-P0
Related:
  - docs/development/current/main/investigations/nested-instance-result-contract0-d0-design-question-2026-07-26.md
  - docs/development/current/main/investigations/stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md
  - src/mir/source_instance_result_contract/README.md
  - src/mir/builder/located_legacy_lowering.rs
  - src/mir/builder/calls/unified_emitter/post_success.rs
---

# Nested instance-result emission handoff

## Accepted decision

Select constrained C-prime:

```text
sealed source-only Integer contract
  + exact stack-scoped associated lowering input
  -> non-Clone prepared emission owner
  -> existing unified Call normalization
  -> finalized mir_call.dst
  -> existing physical Call writer
  -> success-only non-Clone emitted receipt
```

The existing `PreparedUnifiedCallPostSuccessV1` proves the required temporal
seam:

```text
finalized callee / arguments / mir_call.dst
  -> prepare Builder-free post-success payload
  -> emit MirInstruction::Call
  -> commit payload only after emit succeeds
```

The new handoff may consume an already-sealed opaque nested-result owner at
that seam. It may not look up, classify, or infer source facts there.

```text
physical Call writer                 = existing UnifiedCallEmitterBox
new physical Call writer             = 0
generic post-success source policy   = 0
type publication in this decision    = 0
```

### Rejected alternatives

```text
A:
  LocatedLegacyLoweringSessionV1 is site-aware but disconnected.
  Activating it as the Stage-B route would be a separate lowering cutover.

B:
  an independent physical Call wrapper duplicates normalization or loses the
  exact success/destination authority. Its safe subset collapses to C-prime.

unconstrained C:
  a generic emitter lookup, source map, or result classifier is forbidden.
```

### Durable products

Source association:

```rust
pub(crate) struct AssociatedNestedInstanceLoweringInputV1<'input> {
    caller: &'input CanonicalSameModuleCallableKeyV1,
    site: &'input SourceExprSiteV1,
    _seal: AssociatedNestedInstanceLoweringInputSealV1,
}
```

It has private route-owned factories only. Caller, site, callee spelling, and
owner names cannot be independently reconstructed by a consumer.

Prepared owner:

```rust
pub(crate) struct PreparedNestedInstanceResultEmissionV1<'site, 'catalog, 'input> {
    contract: SealedNestedInstanceResultContractV1<'site, 'catalog>,
    input: AssociatedNestedInstanceLoweringInputV1<'input>,
    _seal: PreparedNestedInstanceResultEmissionSealV1,
}
```

Its constructor co-seals the exact caller and `SourceExprSiteV1`. It owns no
`ValueId`, `MirType`, builder, type context, or publication capability.

Successful receipt:

```rust
pub(crate) struct EmittedNestedInstanceCallV1 {
    final_destination: ValueId,
    _seal: EmittedNestedInstanceCallSealV1,
}
```

Only the constrained emitter entry may construct it, and only after the
existing physical Call succeeds. Its destination comes exclusively from
finalized `mir_call.dst`, never from the caller's requested destination.

### File boundary

Keep the new builder-side transport outside the source-only contract module:

```text
src/mir/builder/calls/nested_instance_result_emission/
  README.md
  mod.rs
  association.rs
  prepared.rs
  post_success.rs
  receipt.rs
  rejection.rs
  tests.rs
```

The source contract may be imported by this builder-side adapter. The reverse
dependency is forbidden. Keep `unified_emitter.rs` as a thin caller of the new
module; do not place the owner implementation into that already-large file.

## Executable task series

This is one T2 handoff series. It has four buildable implementation commits
after this docs closeout. I0 is explicitly outside the series.

### Commit 1 — correspondence proof

```text
row = NESTED-INSTANCE-RESULT-EMISSION-CORRESPONDENCE0-P0
behavior delta = 0
production constructor = 0
```

Add a test-only observation product for the two actual source sites:

```text
Body(3).Value.Argument(1)
Body(4).LoopBody(5).Value.Argument(1)
```

For each site record and compare:

```text
exact SourceExprSiteV1
actual physical route family
current basic block
finalized mir_call.dst
emitted MirInstruction::Call.dst
emitted call ordinal
pre-existing Integer type write count
```

The probe is observation-only. Names may appear in diagnostic output but may
not select the site or route.

Stop before P0-A if any site:

```text
does not reach the generic unified Call seam
reaches legacy / BoxCall / special rewrite fallback
maps to zero or multiple physical Calls
cannot expose the exact final destination
already receives an unrelated Integer type authority
```

## P0 correspondence outcome — 2026-07-27

**Stopped before P0-A.**  The required correspondence does not hold in the
current tree, so no association, receipt, emitter API, or type publication was
implemented.

| Exact source site | Observed route | Status | Type authority after successful Call |
| --- | --- | --- | --- |
| `Body(3).Value.Argument(1)` | raw statement descent -> standard unified `Method` Call | production-shaped prefix harness; not the predicted `MeLoweredGlobal` terminal | none in this fixture because the raw port has no function-header lookup |
| `Body(4).LoopBody(5).Value.Argument(1)` | located GenericLoop claim -> unified `Global` Call | explicitly disconnected; no production route or claim consumer | `emit_selected_exact_i64` immediately writes `type_ctx[dst] = Integer` |

The first route reaches the existing unified Call seam, but it is a distinct
route family from the card's `MeLoweredGlobal` prediction.  The second route
does reach the seam in focused tests, but
`generic_loop_located_composer.rs` explicitly states that it has no production
route or claim consumer.  Its selected-call emitter also performs the exact
type publication that this series forbids.

The pre-loop route has a separate authority failure: its production raw
MethodCall input carries receiver/method/arguments but **no
`SourceExprSiteV1`**.  The existing location-carrying method input belongs to
the explicitly disconnected `LocatedLegacyLoweringSessionV1`.  Consequently,
P0-A cannot truthfully construct the planned exact `{ caller, site }`
association from the real route without forbidden source re-walk/ordinal
reconstruction or a separately designed location-carrying raw descent
contract.

The P0 structural stop has therefore fired:

```text
one actual production Stage-B route for each selected site = not proven
pre-loop exact route-owned SourceExprSiteV1                = absent
loop-refresh type_ctx write                               = existing 1
P0 type_ctx write zero                                    = false
P0-A / P0-B / P0-C / P0-G0 authorization                  = 0
```

The unified emitter itself remains a valid physical-success observation point:
it creates `mir_call`, prepares `PreparedUnifiedCallPostSuccessV1` from
`mir_call.dst`, emits `MirInstruction::Call`, then commits existing
post-success work only after emission succeeds.  That fact does not authorize
putting source policy in the emitter or activating a disconnected loop route.

The next owner is the explicit design stop
`NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1`.  Its question must choose
the real Stage-B route(s) and the owner of the pre-existing loop result write
before this handoff series can resume.  It must not silently relabel the
located proof route as production, move the existing type write, or force the
pre-loop route through the header-aware global terminal.

### Commit 2 — exact source association

```text
row = CALLABLE-RESULT-NESTED-REP0-P0-A
```

Implement:

```text
AssociatedNestedInstanceLoweringInputV1
PreparedNestedInstanceResultEmissionV1
NestedInstanceResultAssociationStageV1
RejectedNestedInstanceResultAssociationV1
```

Acceptance:

```text
exact contract caller == exact input caller
exact contract site   == exact input site
foreign/equal-looking input rejects by identity
unselected site rejects before Call preparation
ValueId / MirType / type_ctx access = 0
production emitter connection = 0
```

Failure exposes only `stage()`, `cause()`, and `discard(self)`. There is no
`into_owner`, retry, resume, or alternate route.

### Commit 3 — constrained success bridge

```text
row = CALLABLE-RESULT-NESTED-REP0-P0-B
```

Add:

```text
PreparedNestedInstancePostSuccessV1
EmittedNestedInstanceCallV1
RejectedNestedInstanceCallEmissionV1
UnifiedCallEmitterBox::emit_unified_call_with_nested_instance_receipt_v1
```

The specialized entry delegates all normalization and physical emission to
the existing unified Call implementation. It may only:

```text
carry the opaque prepared owner
pair it with finalized mir_call.dst
commit the nested receipt after emit_instruction succeeds
```

It may not construct `MirInstruction::Call`, resolve a callee, finalize
operands, inspect source facts, or write a type independently.

Acceptance:

```text
successful Call -> exactly one receipt
Call preparation failure -> retained owner, receipt 0
physical emission failure -> retained owner, normal post-success 0, receipt 0
alternate emission route -> typed reject
type_ctx write -> 0 in every branch
ordinary unified Call API behavior -> unchanged
```

### Commit 4 — actual two-site adapters and closeout

```text
rows =
  CALLABLE-RESULT-NESTED-REP0-P0-C
  CALLABLE-RESULT-NESTED-REP0-P0-G0
```

Connect exactly the two correspondence-proven route adapters:

```text
pre-loop exact method-call adapter = 1
loop-refresh exact located-claim adapter = 1
shared source contract owner = 1
shared physical success bridge = 1
shared emitted receipt type = 1
```

Do not connect a route merely because the consultation predicted it. The P0
correspondence product must identify the exact production-shaped seam first.

Focused parity:

```text
pre-loop -> receipt(final dst), type write 0
loop refresh -> receipt(final dst), type write 0
unselected/foreign site -> Call 0, receipt 0
physical Call failure -> receipt 0, type write 0
failure -> fresh compiler/function transaction success
GenericLoop remains read-only
no builder field or persistent source-site map
```

At G0, stop at the next design decision:

```text
CALLABLE-RESULT-NESTED-REP0-I0-D0
```

That decision must separately authorize the sole consumption of
`EmittedNestedInstanceCallV1` into `type_ctx.value_types[final_destination] =
Integer`. This card does not authorize it.

## Verification

Minimum per-code-commit gates:

```bash
cargo build --release --bin hakorune
cargo test -q source_instance_result_contract --lib
cargo test -q callable_result_representation --lib
cargo test -q nested_instance_result_emission --lib
python3 tools/checks/lib/callable_result_i64_catalog_s0.py
bash tools/checks/core_method_contract_manifest_guard.sh
bash tools/checks/generic_loop_progression_role_v0_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Do not add a per-row shell guard. Use focused Rust tests, the existing static
catalog guard, and the reusable GenericLoop lane guard. If a new structural
check cannot fit without pushing an existing source/check file to 800 lines,
place it in one reusable owner-level Python guard under 250 lines and index it
once.

## Structural closeout gate

```text
source Integer contract producer                   = existing 1
prepared emission producer                         = 1
physical Call writer                               = existing 1
second physical Call writer                        = 0

generic emitter source lookup/classification       = 0
opaque prepared-owner consumer                     = 1
post-success final destination authority            = 1
emitted nested Call receipt producer                = 1

type_ctx write in correspondence/P0 series          = 0
MirType::Integer publication consumer               = 0
Builder source-association field                    = 0
persistent source-site -> ValueId map               = 0

LocatedLegacyLoweringSession production activation = 0
pre-loop route adapter                              = exact 1
loop-refresh route adapter                          = exact 1
shared success bridge                               = 1

GenericLoop type producer/import/receipt consumer   = 0
callee/ParserBox name policy                        = 0
metadata/runtime/annotation recovery                = 0
pre-success or failed-call type write               = 0
fallback / retry                                    = 0
all modified/new source/check files                 < 800 lines
```

## Proof budget and sunset

This series extends the already-issued nested-result proof family. It does not
issue a second sunset.

```text
ceremony_tier = T2 physical success handoff authority
sunset_id = CALLABLE-RESULT-NESTED-REP0-PROOF-SUNSET-001
new durable transport products = prepared emission + emitted receipt
new standalone proof family = 0
sunset row = CALLABLE-RESULT-NESTED-REP0-RETIRE0-S0
retire_when = generalized canonical result authority covers this exact source
  contract, associated input, physical success, and destination relation
```

## Required closeout

```text
Decision:
  NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-prime-r1

Status:
  accepted

Selected bridge:
  constrained C-prime

Exact Stage-B seam:
  UnifiedCallEmitterBox::
    emit_unified_call_with_nested_instance_receipt_v1

Prepared owner:
  PreparedNestedInstanceResultEmissionV1

Successful receipt:
  EmittedNestedInstanceCallV1
  stores final_destination only

type publication in this series:
  0

first executable row:
  NESTED-INSTANCE-RESULT-EMISSION-CORRESPONDENCE0-P0

next design stop after G0:
  CALLABLE-RESULT-NESTED-REP0-I0-D0
```

## Consultation input (closed)

We have closed the source-only half of one bounded Stage-B repair.  The next
step is **not** to publish a type yet.  We must select the sole one-shot bridge
from an exact source call site to the final destination of one successfully
emitted physical Call.

Please recommend one option, identify any missing evidence, and give a bounded
task sequence.  Do not propose broad type inference or a general emitter
rewrite.

## Already closed

`NESTED-INSTANCE-RESULT-CONTRACT0-S0` landed as:

```text
source MethodCall site
  -> canonical `me` receiver
  -> same-owner instance declaration lookup
  -> opaque existing callable body proof
  -> ExactI64 with empty required-argument set
  -> SealedNestedInstanceResultContractV1
```

The actual source facts are two occurrences of the same shape in
`ParserBox.static_const_parse_add/2`:

```hako
me.static_const_eval_pos(ret)
```

```text
pre-loop site    = Body(3).Value.Argument(1)
loop-refresh site= Body(4).LoopBody(5).Value.Argument(1)
```

The source-only owner has no `MirBuilder`, `ValueId`, `MirType`, `type_ctx`,
emission, runtime, or publication authority.  The existing static callable
result catalog remains static-only.

## Exact observed failure

The real Stage-B path later reaches:

```text
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(28) }
```

`GenericLoop` is already fixed as a consumer/verifier only.  It must not infer
or backfill this type.

The desired eventual law is narrowly scoped:

```text
one sealed nested source result contract
  + one exact associated lowering input
  + successful physical Call with final destination after remap
  -> exactly one `MirType::Integer` write
  -> existing GenericLoop reads it
```

No write may occur before physical call success.  A failed call must leave no
type write.  The result must not become a persistent source-site-to-`ValueId`
map.

## Current implementation evidence

There are two relevant but non-equivalent seams.

### Located legacy terminal

`LocatedLegacyLoweringSessionV1` carries a method-call input in its port API,
but every current value terminal currently takes it as `_input` and discards
it.  For example, the global and `me`-lowered global terminals do this:

```rust
fn emit_me_lowered_global_value_terminal(
    &mut self,
    builder: &mut MirBuilder,
    _input: &Self::MethodCallInput,
    ...
) -> Result<ValueId, String>
```

This proves that a site-aware port exists, but does **not** prove that this is
the actual Stage-B emitter for the failing ParserBox path, nor that it retains
the exact input through successful Call commit.

### Unified emitter success hook

The unified call emitter has an existing post-success area after Call emission.
It naturally sees an emitted destination, but it does not currently own the
exact source-site contract.  Giving it a general source-result lookup or a
persistent site map would widen it beyond this bounded row.

## Non-negotiable boundaries

```text
GenericLoop type production                         = 0
static callable-result catalog instance rows        = 0
new expression/body walker                          = 0
callee-name / ParserBox-name policy                 = 0
source annotation authority                         = 0
metadata / runtime type recovery                    = 0
Builder-stored source-site map                      = 0
persistent source-site -> ValueId map               = 0
type write before Call success                      = 0
post-failure type write                             = 0
generic unified-emitter source policy               = 0
fallback / retry / alternate lowering route         = 0
Hako source workaround                              = 0
```

The exact source contract remains the only result authority.  The physical
call receipt remains the only destination authority.

## Options to decide

### A — extend the exact existing located terminal (preferred if reachable)

Add a one-shot associated-source input only to the exact terminal that the
real Stage-B ParserBox call path demonstrably uses.

```text
sealed source contract
  + exact located MethodCall input
  -> PreparedNestedInstanceResultEmissionV1
  -> existing successful physical Call
  -> final destination receipt
```

Requirements:

```text
the real failing path reaches this terminal          = proven
input identity survives until Call success           = proven
receipt is non-Clone and consuming                    = yes
no type_ctx write in this option                      = yes
no generic port widening                              = yes
```

Reject A if the observed terminal is merely a legacy/test route or cannot
preserve the exact input without broadening every terminal.

### B — create a narrow associated-source Call wrapper at the real emitter

Introduce a small, route-scoped wrapper immediately around the actual physical
Call emission site.  It owns neither a builder-wide registry nor generic
source policy; it only pairs one exact lowering input with one call request and
returns a receipt after the call succeeds.

```text
PreparedNestedInstanceCallV1
  - exact source contract
  - exact associated lowering input
  - one physical call request
        ↓ successful existing emitter
EmittedNestedInstanceCallV1
  - final remapped destination only
```

Requirements:

```text
one physical Call writer remains existing authority  = yes
wrapper cannot publish a type                         = yes
call failure retains/discards the prepared owner      = typed
no generic `unified_emitter` source classification    = yes
```

### C — use a generic post-success emitter hook (rejected unless stricter)

This would pass source metadata through the generic unified emitter and act in
its post-success hook.  It is currently presumed too broad because that layer
would gain source-result policy.  It is acceptable only if the answer can show
that the hook consumes an already sealed, route-scoped receipt and cannot
classify or look up source facts itself.

## Questions requiring an explicit answer

1. Which option is the cleanest first bridge: A, B, or a narrowly constrained
   C?  State why the other two are rejected.
2. What exact product should be created before the call, and what exact product
   should exist only after successful emission?  Give minimal Rust-shaped
   types and consuming terminals.
3. Where should the source-associated input be carried so that it survives
   nested argument lowering but cannot become a builder-wide map?
4. How must Call failure retain or discard the prepared owner while guaranteeing
   `type_ctx` writes remain zero?
5. What read-only probe proves the chosen seam is the actual failing Stage-B
   path before code changes?  It must identify source site, physical Call
   route, and final destination identity without using names as policy.
6. What focused fixtures demonstrate:

```text
pre-loop success
loop-refresh success
unselected source rejection
Call failure -> no type write
fresh compiler reuse after failure
GenericLoop remains a consumer only
```

7. Should the first executable row be `CALLABLE-RESULT-NESTED-REP0-P0`, or is
   a short read-only correspondence row required first?  Do not authorize I0
   or any type publication in this decision.

## Required decision closeout

```text
Decision:
  NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-prime-r1

Selected bridge:
  A | B | constrained C

Exact Stage-B seam:
  <one named terminal/wrapper only>

Prepared owner:
  <non-Clone, source contract + associated lowering input>

Successful receipt:
  <non-Clone, final destination only>

type publication in this row:
  0

first executable row:
  <P0 or one read-only correspondence row>

Forbidden:
  builder-wide source map
  persistent source-site -> ValueId map
  generic source inference in emitter
  pre-success or failed-call type write
  fallback/retry
```

## Non-claims

```text
general instance-call result inference
general source-associated Call metadata
static catalog widening
all MethodCall/FunctionCall typing
GenericLoop changes
MirType/type_ctx publication
VM/backend changes
parser/Hako changes
ownership grammar activation
```
