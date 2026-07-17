---
Status: S0/M0/I0 closed; G0 paused on the next independent loop-array-join carrier blocker
Date: 2026-07-17
Decision: Candidate B-prime
Baseline: 7424548356
Parent: hmi-s0-v0-r0-generic-loop-carrier-type-consultation-question-2026-07-17.md
Scope: GenericLoop carrier role and lowering-time representation projection
---

# HMI R0-I0 GenericLoop carrier type task

## Current progress

`R0-GENERICLOOP-CARRIER-TYPE0-S0` and the behavior-neutral M0 inventory are
closed with production representation consumers zero.

```text
facts-side role:
  GenericLoopCarrierRoleV1
  - NumericProgression
  - BodyManagedState

Builder-private pure decision:
  prepare_generic_loop_carrier_representation_v1

prepared product:
  init ValueId + exact MirType
  non-Clone
```

The decision accepts exact Integer for numeric progression, preserves exact
Integer/Bool/Box representations for body-managed state, and rejects missing
init, missing type, `Unknown`, Float numeric, and Box numeric inputs. It does
not receive `MirBuilder`, so decision failure cannot allocate blocks/values or
publish metadata. README and module boundaries name the facts/representation
split before code wiring.

M0 seals one machine-readable pre-activation inventory and extends the existing
GenericLoop guard. The exact production boundary is:

```text
role producers:
  V0 constructor = 1
  V1 StepResolution mapping point = 1

skeleton direct callers:
  normalizer = 1
  recipe composer V0/V1 = 2

pre-I0 representation consumer:
  0
```

The inventory also fixes two matched-facts error-transport seams: one nested
`.ok()` conversion and two top-level non-strict composer-error conversions.
After facts match, a carrier representation failure is an admitted-route
contract error; it must propagate and must not become `Ok(None)` or probe a
later route. The two post-compose verifier fallbacks and two post-compose
lowerer fallbacks are recorded as pre-existing broader debt, but I0 does not
authorize changing their policy. TYPE0 preparation happens before allocation,
so only its exact pre-allocation error transport is in scope.

Fresh isolated debug/release `vm-reference` builds agree. Numeric V1 routes and
returns 3; a new minimal numeric V0 route returns 4. The 182-row fast-gate
census finds 89 GenericLoop sources and no authoritative legitimate
missing/Unknown numeric transient fact. Final MIR alone is explicitly not used
to infer lowering-time absence. Current body-managed evidence instead shows
the hardcoded skeleton conflict against exact `Box(JsonScanner)` and exact
`String` carriers.

Focused 6/6, existing progression-role 18/18, the extended GenericLoop guard,
inventory reference, V0/V1 route pins, fresh-build census, formatting, pointer,
diff, and line-budget checks are green. Those results authorized I0 only; I0 is
now closed as recorded below.

I0 is now closed. V0 facts seal `NumericProgression`; V1 maps the existing
`use_body_managed_step` decision exactly once. All three skeleton callers pass
the sealed role. The skeleton prepares the selected init/type before block
allocation, retains the non-Clone prepared row, and allocates current/next plus
the selected V1 step-PHI from its exact representation. The independent V1
carrier family keeps its one existing init-derived fallback.

Matched-facts error transport is also closed:

```text
nested normalize `.ok()` swallows = 0
top-level GenericLoop compose swallows = 0
post-compose verifier fallbacks = 2 unchanged
post-compose lowerer fallbacks = 2 unchanged
```

Numeric V0/V1 debug/release routes still return 4/3. The HMI document-seal
canary now passes the former `JsonScanner` body-managed Box carrier conflict
and reaches a new independent failure:

```text
owner:
  loop_array_join

function:
  StringUtils.join/2

failure:
  PHI destination Integer vs incoming String
```

This is not a selected GenericLoop slot and must not widen TYPE0. G0 remains
paused until the loop-array-join carrier owner is inventoried and repaired or
classified. Temporary diagnostic traces were removed; no source workaround,
name special case, PHI weakening, ownership, runtime, backend, or fallback was
added.

## Decision

Select a role-sealed carrier law.

```text
GenericLoopCarrierRoleV1:
  NumericProgression
  BodyManagedState

facts authority:
  carrier role only

Builder representation authority:
  selected init ValueId + current transient type
```

`use_body_managed_step` is already a semantic extraction decision. It controls
whether the step remains in the body, which statements feed the body recipe,
and whether `loop_increment` is the body-managed current value. It must be
sealed once as a closed role instead of being discarded and later inferred
from the compatibility sentinel.

The role product owns no `MirType`. The skeleton projection combines the
sealed role with the already selected `loop_var_init` and current
`type_ctx.value_types`.

```text
NumericProgression:
  required init representation = exact Integer

BodyManagedState:
  required init representation = one exact non-Unknown MirType T
```

Missing type, `Unknown`, or a non-Integer numeric init rejects before block or
carrier allocation. There is no Integer default.

## Why not A or C

Candidate A would make canonical arithmetic and V0 carriers
representation-polymorphic without a separately accepted language boundary.
Adding an Integer exception to A would recreate the role distinction without
sealing its semantic reason.

Candidate C is incompatible with existing accepted shapes. V1 tests
deliberately admit receiver-managed scanner steps, and `json_native` currently
contains nine receiver-state loops across scanner/tokenizer sources. The
selected `JsonScanner.read_identifier` loop has no alternate admitted route.

## Authority boundary

```text
source/control observation:
  GenericLoop V0/V1 facts extraction

carrier role:
  GenericLoopCarrierRoleV1

selected init identity:
  current variable_map[loop_var]

lowering-time representation:
  current type_ctx.value_types[loop_var_init]

slot allocation:
  GenericLoop skeleton allocator

PHI conflict law:
  unchanged TYPE-PUBLISH0
```

V0 seals `NumericProgression`. V1 maps the existing extraction decision once:

```text
use_body_managed_step = false -> NumericProgression
use_body_managed_step = true  -> BodyManagedState
```

The compatibility `loop_increment = Variable(loop_var)` sentinel may remain
as the body-managed next-value expression, but Lower must not infer the role
from that AST spelling. Tests guard role/sentinel consistency until a later
sentinel-retirement cleanup.

The test-only `facts/progression_role` inventory is not the production owner
and is not promoted into one.

## Slot correspondence

One prepared representation row owns the selected loop variable slots.

```text
loop_var_init:
  exact source fact checked, not reallocated

loop_var_current:
  prepared exact representation

loop_var_next:
  same prepared exact representation

V1 loop_var_step_phi:
  same prepared exact representation

header PHI destination:
  loop_var_current, therefore same representation
```

Other independent carrier variables keep their existing init-derived law and
are outside this row. The PHI publisher validates the resulting facts; it does
not repair or overwrite them.

## Task order

```text
R0-GENERICLOOP-CARRIER-TYPE0-S0
  -> R0-GENERICLOOP-CARRIER-TYPE0-M0
  -> R0-GENERICLOOP-CARRIER-TYPE0-I0
  -> R0-GENERICLOOP-CARRIER-TYPE0-G0
  -> clean HMI-S0-V0-R0-I0 rewrite
```

### S0 — disconnected role and representation decision

Production behavior delta is zero.

Add a closed facts-side role and one Builder-private pure preparation product,
for example:

```rust
enum GenericLoopCarrierRoleV1 {
    NumericProgression,
    BodyManagedState,
}

struct PreparedGenericLoopCarrierRepresentationV1 {
    init: ValueId,
    exact_type: MirType,
}
```

The preparation verifier receives role, init identity, and current transient
type. It owns typed errors for missing loop variable, missing/Unknown type,
and numeric representation mismatch. Production consumers remain zero.

Required focused fixtures:

```text
Numeric + Integer -> prepared Integer
Numeric + Float/Box -> reject
BodyManaged + Integer/Bool/Box -> preserve exact type
missing init -> reject
missing/Unknown transient type -> reject
decision failure allocates no block or ValueId
```

### M0 — exact producer/consumer and fallback inventory (closed)

Production behavior delta is zero.

Seal the following inventory:

```text
role producers:
  V0 extraction = NumericProgression
  V1 extraction = exactly one mapping from StepResolution

skeleton callers:
  generic_loop v1 normalizer
  recipe composer v0
  recipe composer v1

selected slots:
  init/current/next/v1 step-PHI/header-PHI
```

Run a fresh debug/release census of admitted GenericLoop candidates. A
legitimate Numeric candidate with missing/Unknown init type is a stop, not an
authorization to restore the Integer default.

M0 must also classify `try_lower_generic_loop_v1_nested`. It currently turns a
matched-facts lowering error into `None` through `.ok()`. A representation
contract failure after facts match must propagate as an error and must never
probe a later route. If no narrow no-match/error split is possible, stop I0.

The narrow split is confirmed implementable:

```text
no GenericLoop facts match:
  Ok(None)

facts matched and normalization/representation fails:
  Err(exact failure)
```

The same law applies at the two top-level composer selections. The I0 change
must carry the prepared representation failure through those existing entry
points without widening unrelated post-compose fallback policy.

The V1 selected loop-variable step-PHI and the independent carrier family are
counted separately. I0 retires only the selected loop-variable `Unknown`
fallback; the independent carrier fallback remains one unchanged row outside
this slice. Body-managed String is an exact pass case alongside Integer, Bool,
and Box; neither String nor Box is special-cased.

Record normalized pre-change evidence for:

```text
integer V0/V1 MIR parity
receiver-managed JsonScanner init = Box(JsonScanner)
current hardcoded Integer conflict
current/next/step-PHI correspondence
```

### I0 — one production projection (closed)

This row has the narrow compiler behavior delta.

1. Seal the role in V0/V1 facts producers.
2. Prepare the carrier representation before `LoopBlocksStandard5::allocate`.
3. Store one prepared exact representation in the skeleton.
4. Allocate current and next from that row.
5. Allocate/verify the V1 loop-variable step-PHI from the same row.
6. Propagate matched-facts representation failures through the nested and
   top-level matched GenericLoop entries; no-match alone may return `None`.

TYPE-PUBLISH0, PHI origin publication, receiver same-root proof, scanner/HMI
source, runtime, ownership, and backends remain unchanged.

### G0 — closeout

Production behavior delta is zero.

Prefer extending an existing reusable GenericLoop/proof entry. Add a new shell
guard only if the structural checks cannot live in an existing stable guard.

Required closeout:

```text
one carrier role definition
one role-to-representation decision owner
one skeleton allocation consumer
three skeleton callers
zero Lower-side role inference
zero Integer default
zero selected slot mismatch
zero PHI conflict weakening
```

After the independent loop-array-join blocker closes, build genuinely fresh
debug and release `vm-reference` binaries with an isolated temporary
`CARGO_TARGET_DIR`, then run both on:

```text
tools/hako_shared/hmi/tests/s0_document_seal_test.hako
```

Require `[hmi/s0-t0-s0] ok`. Reusing an existing target binary is not evidence.

Also keep integer GenericLoop parity, compiler reuse after rejection, pointer,
formatting, diff, and source/check line-budget gates green.

## Required pass matrix

```text
V0 numeric Integer progression: unchanged MIR/runtime
V1 numeric Integer progression: unchanged MIR/runtime
V1 body-managed receiver: exact Box(owner) on all selected slots
body-managed Integer state: exact Integer
body-managed String state: exact String
receiver-managed scanner tests: accepted
JsonScanner.read_identifier: no concrete PHI fact conflict
fresh debug/release document seal: green
failed compile followed by valid compile: green
```

## Required rejection matrix

```text
missing loop variable
missing transient init type
Unknown transient init type
NumericProgression with non-Integer init
role/sentinel inconsistency
selected slot representation mismatch
mixed representation PHI
matched nested facts followed by swallowed lowering error
second carrier role/type owner
```

## Counters

```text
GenericLoopCarrierRoleV1 definitions = 1
role producers = V0 exactly 1 + V1 exactly 1
role inference consumers after facts = 0

carrier representation decision owners = 1
skeleton allocation consumers = 1
skeleton direct callers = 3

selected current/next/step-PHI representation mismatch = 0
Numeric non-Integer acceptance = 0
missing/Unknown Integer defaults = 0

facts-side MirType storage = 0
value_origin/type-map backfill = 0
PHI destination overwrite/fallback = 0

function/field/method/HMI-name conditions = 0
runtime type-tag reads = 0
final metadata reads during lowering = 0
new persistent ValueId maps = 0

scanner/HMI source delta = 0
ownership/backend/runtime delta = 0
fallback/retry/legacy probing = 0
stash restore/apply/pop/copy = 0
source/check files >= 800 lines = 0
```

## Claims and non-claims

After G0, implementation may claim:

```text
GenericLoop facts seal numeric versus body-managed carrier role once
numeric progression remains exact Integer
body-managed state uses the exact selected init representation
all selected slots share one prepared representation
TYPE-PUBLISH0 validates rather than repairs the carrier facts
fresh strict JSON/HMI producer fixture compiles without weakening PHI law
```

It must not claim:

```text
all GenericLoop carriers are representation-polymorphic
general Float/Box numeric progression
general loop-carried ownership
arbitrary union/coercion PHIs
type inference from AST/name/runtime tags
loop/backedge receiver-equivalence widening
HMI register completion
runtime/backend widening
```

## Stop laws

Stop before I0 if any implementation requires:

1. weakening or bypassing PHI concrete-fact conflict;
2. retaining an Integer default for missing/Unknown init facts;
3. inferring role again from `loop_increment`, names, or downstream MIR;
4. storing `MirType` in facts;
5. reading finalized metadata instead of transient types;
6. a second persistent ValueId-to-type/owner map;
7. accepting non-Integer numeric progression in this row;
8. swallowing a matched-facts representation error and probing another route;
9. scanner/HMI source rewriting;
10. fallback, legacy LoopBuilder, environment route selection, ownership, or
    backend widening;
11. a source/check file reaching 800 lines.

## Final lock

> Candidate B-prime is selected. GenericLoop facts seal exactly one closed
> carrier role, `NumericProgression` or `BodyManagedState`, and never own a MIR
> representation. The skeleton is the sole projection owner: numeric state
> requires exact Integer, while body-managed state requires and preserves the
> selected init ValueId's exact non-Unknown transient type. One prepared row is
> shared by current, next, V1 step-PHI, and the header PHI destination. Missing,
> Unknown, mismatch, and matched-route lowering failures reject before effects
> without default, retry, or fallback. S0, M0, and I0 are closed. M0 fixed the
> complete caller/slot inventory and no-match/error transport law; I0 projects
> the sealed role once before allocation and removes only the selected-slot
> defaults and matched-route error swallowing. The fresh HMI canary now reaches
> the independent `loop_array_join` String-carrier conflict, so G0 remains paused
> while that owner is inventoried. TYPE-PUBLISH0, receiver proof, scanner/HMI
> source, ownership, runtime, and backend authorities remain unchanged.
