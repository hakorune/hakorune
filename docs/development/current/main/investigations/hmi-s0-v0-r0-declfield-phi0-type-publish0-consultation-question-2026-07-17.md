---
Status: Evidence complete; question entry superseded by one-shot D0 packet
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

## Exact current entry inventory

Read-only code inventory identifies five lifecycle shapes, but only four have
access to the current Builder transient type authority.

| Entry | Mutation | Current type publication | Classification |
| --- | --- | --- | --- |
| `MirBuilder::emit_instruction(Phi)` | append raw Phi | combined type+origin before append | existing Builder producer |
| `define_phi_final_with_type_hint` | insert complete Phi at head | none | missing Builder completion producer |
| `define_phi_batch_prepend` | prepend complete Phi batch | none | missing Builder completion producer |
| `define_provisional_phi` then `patch_phi_inputs` | insert empty Phi, later replace inputs | none | patch is missing Builder completion producer |
| `define_phi_final_fn_with_type_hint_and_tag` | mutate a supplied `MirFunction` | no access to `MirBuilder.type_ctx` | function-level non-consumer |

Thin aliases do not create additional policy owners:

```text
define_phi_final:
  define_phi_final_with_type_hint(..., None)

define_current_block_phi_final:
  block-selection facade over define_phi_final

define_current_block_phi_final_with_type_hint:
  block-selection facade over define_phi_final_with_type_hint

define_phi_final_fn:
  function-level facade over define_phi_final_fn_with_type_hint_and_tag

PhiTxn:
  transaction facade over provisional define / patch / rollback
```

Observed production caller families:

```text
complete Builder final:
  If/Binding join
  effect emission
  ordinary if/peek/loop APIs
  resolved lowering

Builder provisional + patch:
  function-owned Binding SSA
  CorePlan loop lowering
  JoinIR exit PHIs

Builder batch:
  JoinIR loop-header PHIs

function-level final:
  EdgeCFG emission
  JoinIR VM bridge conversion
```

`ssa::phi_input_materializer::function_repair` and JoinIR instruction
rewriters may rewrite already-created PHI input/block structure. They are not
new destination-type producer authorities in this row; M0 must verify that
their rewrites preserve the selected type decision's premises or stop.

The function-level final API cannot borrow or publish the current Builder
`type_ctx` because it receives only `&mut MirFunction`. D0 must classify it as
an explicit non-consumer for this lowering-time row. Eager writes to
`function.metadata.value_types` are still forbidden. If function-level PHI
typing later becomes necessary, it requires a separate authority rather than
smuggling Builder state through this API.

## Existing type semantics are not a strict conflict authority

The current raw and finalization paths have different permissive policies.
Neither owns the strict D0 conflict law requested by this row.

### Raw `origin::phi::propagate_phi_meta`

Current type behavior:

```text
all input map entries present and exactly equal:
  publish that MirType

one input missing or concrete inputs differ:
  no publication

all inputs Unknown:
  publish Unknown

all inputs Void:
  publish Void

existing destination type:
  overwritten without conflict check

instruction type_hint:
  not read

typed failure:
  none
```

The helper also owns a separate unanimous-origin write and currently runs
before the raw instruction append completes.

### Final `TypePropagationPipeline` PHI step

`PhiTypeResolver` traverses Copy/Phi definitions to base values. Its policy is:

```text
type_hint:
  not read

missing base type:
  ignored

Unknown / Void base type:
  ignored

remaining distinct concrete base types = 1:
  infer that type

remaining distinct concrete base types = 0 or >= 2:
  no inference

existing destination type differs:
  overwrite as a final correction
```

Therefore finalization is not exact unanimous-input validation. It is a
completed-function recovery/correction pass, and cannot be reused as the
lowering-time producer decision.

### Consequence for D0

There is no existing strict owner for:

```text
destination concrete type conflict
type_hint conflict
unanimous Unknown/Void admission
missing input admission
heterogeneous input failure timing
mutation/publication atomicity
```

D0 must select these laws explicitly. It must not describe the new helper as
mere behavior-neutral extraction of either current raw semantics or final
pipeline semantics.

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

The existing raw route currently combines type and origin publication in
`origin::phi::propagate_phi_meta`. The minimal structural split is:

```text
neutral unanimous type decision:
  one owner

raw Builder Phi:
  consume type decision
  retain existing unanimous origin behavior through a separate owner

lifecycle final / patch / batch:
  consume type decision only
  new origin publication = 0
```

This preserves raw origin behavior without granting lifecycle PHIs a new
origin authority.

## Publication transaction law to decide

Type publication must not survive a failed PHI mutation. The provisional
transaction shape is:

```text
complete final:
  materialize/normalize inputs
  compute type decision and validate conflicts
  insert complete Phi
  publish prevalidated type non-fallibly

provisional define:
  insert empty Phi
  publish no unanimous input type

patch:
  normalize inputs
  compute type decision and validate conflicts
  patch existing Phi inputs
  publish prevalidated type non-fallibly

batch:
  normalize every item
  compute every decision and validate every conflict
  atomically insert the complete batch
  publish all prevalidated types non-fallibly

raw emission:
  compute/validate before instruction mutation
  emit instruction
  publish prevalidated type after successful emission
  preserve existing origin behavior without partial metadata on failure
```

D0 must accept or replace this ordering. In particular, no entry may mutate
one PHI or publish one type before a later item in the same batch reports a
conflict.

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

### 6. Failure atomicity

Lock the exact transaction boundary:

```text
conflict discovered before PHI mutation:
  instruction delta = 0
  transient type delta = 0

low-level insertion/patch failure:
  transient type delta = 0

successful insertion/patch:
  selected type publication is non-fallible

batch conflict/failure:
  instruction delta = 0
  transient type delta = 0
```

The existing raw route's pre-emission metadata write must not be copied into
new lifecycle entries without this decision.

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
  exact parity for D0-admitted unanimous concrete cases

conflict/missing/Unknown/Void cases:
  exact D0 result
  final pipeline behavior is observation only, not parity authority

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
authorized Builder publication consumers = exact D0 count
function-level transient publication consumers = 0

receiver proof type inference rules = 0
receiver proof type_ctx writes = 0
value_origin_newbox writes from new helper = 0
function.metadata eager writes = 0

field/method/HMI-name conditions = 0
runtime tag reads = 0
current_static_box inference = 0
final metadata fallback during lowering = 0
mid-lowering TypePropagationPipeline calls = 0
partial type publication after failed PHI mutation = 0
partial batch instruction/type publication = 0
type_hint reads in final pipeline as D0 authority = 0
final PhiTypeResolver conflict decisions reused by D0 = 0

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
failure atomicity for raw/final/patch/batch entries
```

No compiler implementation is authorized until those decisions are locked.
