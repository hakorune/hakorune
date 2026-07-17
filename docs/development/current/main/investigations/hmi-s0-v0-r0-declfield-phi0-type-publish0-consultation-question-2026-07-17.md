---
Status: External consultation required
Date: 2026-07-17
Baseline: d3c4473728
Parent: hmi-s0-v0-r0-declfield-phi0-transient-type-consultation-question-2026-07-17.md
Scope: canonical lowering-time PHI destination type publication
---

# R0-DECLFIELD-PHI0-TYPE-PUBLISH0 consultation

## Response classification

The received A-prime-TV response preserves the accepted receiver-equivalence
law:

```text
accepted provenance:
  exact current receiver parameter 0
  ordinary Copy
  finite acyclic non-loop Phi

required representation fact:
  every traversed value already has transient Box(current owner)

forbidden:
  derive a missing type inside the receiver proof
  read finalized metadata backwards
  origin/type backfill from the receiver proof
```

That contract is accepted. Its proposed standalone `TV0` and replay of
`PHI0-S0/P0` are not selected as the next rows, because the current
implementation already:

```text
reads:
  MirBuilder.type_ctx.value_types

checks:
  exact current-function parameter/instruction membership
  exact Box(current owner) on every traversed value
  missing / mismatch / foreign origin

owns:
  the complete P1-P9 / R1-R24 matrix
  exact real-fixture A1-A4 proof
```

`CurrentFunctionTransientTypeViewV1` would encapsulate existing reads but
would not add the missing type fact. It may be folded into the future
producer-proof facade or landed later as behavior-neutral cleanup, but it is
not an unblocker or a new semantic authority.

## Stop condition already reached

The A-prime-TV response says to open a producer bug row when an admitted
Copy/Phi value lacks a transient type. That condition is already proven:

```text
selected function:
  DeclaredFieldOwnerV1.declfield_probe_v1_after_validation/2

transient field base:
  %37

definition:
  %37 = Copy(%19)
  %19 = Phi(%0, %0)
  %0  = exact current receiver parameter 0

exact use-site proof result:
  SeedTypeMissing

finalized MIR only:
  %19 and %37 later carry Box(DeclaredFieldOwnerV1)
```

Therefore the next authority is not another receiver/type view. It is the
canonical producer and timing law for unanimous PHI destination types.

## Existing construction asymmetry

```text
raw MirBuilder::emit_instruction(Phi):
  origin::phi::propagate_phi_meta
  -> may publish unanimous input type

Binding-SSA / If PHI lifecycle:
  define_phi_final_with_type_hint
  -> direct insert at block head
  -> bypasses raw emit_instruction(Phi)
  -> no shared unanimous type publication

LocalSSA Copy:
  copies source type only when already present
  -> cannot type %37 when lifecycle Phi %19 is untyped
```

This is producer-entry drift. The receiver proof is correctly failing fast.

## Selected architectural direction

The provisional direction is one neutral type-only unanimous-PHI policy and
publication helper shared by every canonical Builder PHI lifecycle completion
entry and raw Phi emission.

```text
input:
  current transient input types
  optional explicit type hint
  optional existing destination type

output:
  one exact publication decision

publication target:
  type_ctx.value_types[phi_dst]

non-publication:
  value_origin_newbox
  MirFunction.metadata.value_types
  receiver equivalence
  field facts
```

The helper must be a producer policy, not a declared-field special case.

## Required task order

```text
R0-DECLFIELD-PHI0-TYPE-PUBLISH0-D0
  decision lock; no code

R0-DECLFIELD-PHI0-TYPE-PUBLISH0-S0
  disconnected type-only decision/helper
  production consumers = 0

R0-DECLFIELD-PHI0-TYPE-PUBLISH0-M0
  raw/final/provisional/patched/batch producer inventory
  exact selected %19 -> Copy %37 timing proof

R0-DECLFIELD-PHI0-TYPE-PUBLISH0-I0
  connect every authorized canonical PHI completion entry

R0-DECLFIELD-PHI0-TYPE-PUBLISH0-G0
  producer/consumer/parity/zero-origin guards

resume existing R0-DECLFIELD-PHI0-I0
  one declared_field_type_for_value fallback consumer

existing R0-DECLFIELD0-G0
  close MAPFIELD-R0-DECLFIELD0

clean HMI-S0-V0-R0-I0 rewrite
```

Do not recreate or rename the already closed receiver `PHI0-S0/P0` rows.
Their matrices become regression gates after producer wiring.

## D0 decisions required before code

### 1. Existing destination type

When the destination already has a concrete type and unanimous inputs agree:

```text
same concrete type:
  preserve idempotently?

different concrete type:
  fail-fast?
  preserve existing and reject publication?
```

The exact owner and typed error must be selected.

### 2. Explicit type hint

When `type_hint` is present:

```text
hint == unanimous input type:
  publish exact type?

hint != unanimous input type:
  fail-fast?
  which fact has precedence?
```

Do not let individual lifecycle call sites decide independently.

### 3. Missing, Unknown, or heterogeneous inputs

Select one exact law for each case:

```text
one or more input type missing
one or more input type Unknown
input concrete types disagree
```

Candidate choices are no-publication or typed failure at a specified seal
boundary. Silent arbitrary selection is forbidden.

### 4. Canonical producer set

Inventory and classify at least:

```text
raw emission
final insertion
provisional insertion
input patching
batch prepend/finalization
function-level variants
```

Each entry must either consume the single policy or be explicitly proven
non-publishing with a reason and retirement boundary.

### 5. Publication timing

The exact contract must guarantee:

```text
lifecycle Phi destination type published:
  before a following LocalSSA Copy is materialized

Copy destination type published:
  before a following FieldGet/method route reads it
```

Final `TypePropagationPipeline` remains only the completed-function backstop.

## Required producer proof matrix

Pass/decision fixtures:

```text
typed receiver inputs -> lifecycle Phi:
  destination typed before finalization

immediate LocalSSA Copy(Phi):
  Copy destination typed at emission

nested unanimous lifecycle PHIs:
  every destination typed at its completion boundary

raw/final/provisional/patched/batch entries:
  normalized policy parity

pretyped destination + matching inputs/hint:
  exact selected idempotence law

early transient type vs final propagation result:
  exact parity

selected A2:
  same-root proof changes only SeedTypeMissing -> accepted
```

Reject/no-publication fixtures:

```text
missing input type
Unknown input type
heterogeneous concrete input types
conflicting pretyped destination
conflicting explicit type hint
foreign or malformed PHI rows
```

The exact result of the first five rows is owned by D0, not inferred during
implementation.

Regression-only reuse:

```text
existing PHI0 P1-P9 / R1-R24
existing real A1=R / A2=P[R,R] / A3=R / A4=R
existing final MirVerifier parity
```

## Counters and guards

```text
unanimous PHI type decision owners = 1
canonical PHI completion entries inventoried = exact expected count
authorized publication consumers = exact D0 count

receiver proof type inference rules = 0
receiver proof type_ctx writes = 0
value_origin_newbox writes from new helper = 0
function.metadata eager writes = 0

field/method/HMI-name conditions = 0
runtime tag reads = 0
current_static_box inference = 0
final metadata fallback during lowering = 0
mid-lowering TypePropagationPipeline calls = 0

new persistent ValueId -> type/owner maps = 0
fallback / retry / legacy probing = 0
ownership operation delta = 0
backend/runtime/HMI source delta = 0

source/check files >= 800 lines = 0
```

## May claim after the full producer row

```text
all authorized canonical Builder PHI completion entries share one exact
type-only publication decision

unanimous PHI destination types become available at the lowering boundary
required by immediately following Copy and field/method consumers

the receiver proof continues to observe rather than infer types

finalized metadata, origins, ownership, runtime, and backend authorities are
unchanged
```

## Must not claim

```text
general PHI origin propagation
general final-MIR type inference
all missing types are recoverable
receiver equivalence implies representation
loop/backedge receiver PHI support
property getter or field-contract recovery
HMI register completion
ownership or backend widening
```

## Stop conditions

Stop if implementation requires:

1. a conflict law not selected by D0;
2. per-entry PHI type decisions instead of one neutral owner;
3. writing `value_origin_newbox` or finalized function metadata;
4. running the whole propagation pipeline mid-lowering;
5. deriving type inside the receiver proof;
6. reading field/method/HMI names or runtime tags;
7. a second persistent value-type map;
8. changing PHI CFG/SSA semantics in the same row;
9. reopening receiver `PHI0-S0/P0` as new implementation work;
10. fallback, retry, or stash restoration;
11. a source/check file reaching 800 lines.

## Exact consultation request

Select the D0 law for:

```text
existing destination concrete type
explicit type hint
missing input type
Unknown input type
heterogeneous input types
canonical PHI completion producer set
publication timing and typed failure boundary
```

No compiler implementation is authorized until those decisions are locked.

