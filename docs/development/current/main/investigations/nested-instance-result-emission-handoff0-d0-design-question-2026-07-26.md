---
Status: open design question
Date: 2026-07-26
Decision sought: NESTED-INSTANCE-RESULT-EMISSION-HANDOFF0-D0
Precondition: NESTED-INSTANCE-RESULT-CONTRACT0-S0 closed
Blocked umbrella: CALLABLE-RESULT-NESTED-REP0-P0
Related:
  - docs/development/current/main/investigations/nested-instance-result-contract0-d0-design-question-2026-07-26.md
  - docs/development/current/main/investigations/stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md
  - src/mir/source_instance_result_contract/README.md
  - src/mir/builder/located_legacy_lowering.rs
  - src/mir/builder/calls/unified_emitter/post_success.rs
---

# Design question: exact handoff for a nested instance-call result

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
