---
Status: Design consultation stop
Date: 2026-07-17
Decision: pending
Baseline: c455853ac4
Parent: hmi-s0-v0-r0-clean-register-storage-task-2026-07-16.md
Scope: GenericLoop skeleton carrier representation before PHI completion
---

# HMI R0-I0 GenericLoop carrier type consultation

## Current stop

Clean `HMI-S0-V0-R0-I0` reached its required producer-backed fixture, but the
already-landed whole-document seal no longer compiles with a fresh current
compiler.

```text
fixture:
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako

normal first failure:
  JsonScanner.read_identifier/0
  generic_loop_v0 unsupported_condition

HAKO_JOINIR_DEBUG=1 deeper failure:
  [freeze:contract][phi_type_publication/concrete_fact_conflict]
  dst=%33
  ExistingDestination = Integer
  Incoming(pred=390, value=%26) = Box(JsonScanner)
```

This is a compiler regression outside the new register source. The register
WIP is stored only as:

```text
wip/hmi-s0-v0-r0-i0 producer-backed seal hits generic-loop carrier type conflict
```

Do not apply, pop, restore, or copy it. Resume from a clean tree after this
compiler prerequisite closes.

## Fresh-build evidence

Required execution conditions are unchanged:

```bash
cargo build --release --features vm-reference --bin hakorune
HAKO_EMIT_EXE_CACHE=0 target/release/hakorune --backend mir \
  tools/hako_shared/hmi/tests/s0_document_seal_test.hako
```

No additional environment switch is required or authorized.

Worker audits established:

```text
preserved pre-regression binary + current fixture:
  [hmi/s0-t0-s0] ok

fresh current binary:
  compile reject

scanner/control-flow source delta since T0 closeout:
  0

register WIP in the S0 import graph:
  0

fresh archived f3a540e066 and 8c8beea27a builds:
  current fixture compiles successfully

first bad production boundary:
  58c54bf254 DECLFIELD0 same-root field lookup activation
```

The old runtime green was vulnerable to a stale release binary. Closeout must
therefore include one fresh-build proof; a pre-existing target binary is not
evidence.

TYPE-PUBLISH0 is not the regression source. DECLFIELD0 makes the scanner's
late current-receiver field facts visible, which changes the admitted loop
lowering surface and exposes the older Integer-only skeleton assumption.
TYPE-PUBLISH0 then correctly diagnoses that newly reached inconsistency.

## Exact producer drift

The PHI decision is correct. It found a false type fact created earlier.

```text
JsonScanner.read_identifier
  loop(condition uses receiver methods)
  body advances the receiver

GenericLoop V1 facts:
  condition canonicalization yields no scalar progression candidate
  body candidate collection admits receiver method calls
  selected body-managed carrier = me

Builder state:
  loop_var_init = %26
  transient type = Box(JsonScanner)

GenericLoop skeleton:
  loop_var_current = %33 allocated as Integer
  loop_var_next allocated as Integer

final PHI row:
  dst = %33
  incoming = %26
```

The stale fact is introduced by:

```text
src/mir/builder/control_flow/plan/skeletons/generic_loop.rs
  alloc_generic_loop_v0_skeleton
  alloc_typed(MirType::Integer) for current and next
```

The incoming row is produced correctly by the existing V1 carrier finalizer.
This path does not use canonical Binding SSA and does not reuse a ValueId.

## Authority that must remain unchanged

```text
PHI concrete fact conflict:
  remains fail-fast

TYPE-PUBLISH0:
  must not overwrite Integer with Box(JsonScanner)
  must not demote the conflict to no-publication

receiver same-root proof:
  unchanged

JsonScanner source:
  no workaround or expression rewrite

HMI fixture:
  strict ingress + whole-document seal only
  no fake VerifiedHmiFunctionView
```

## Decision question

Which durable carrier-representation law should GenericLoop own?

### Candidate A — exact-init representation

```text
loop_var_init:
  resolve from current variable_map

carrier representation:
  exact current transient type of loop_var_init

loop_var_current / loop_var_next:
  allocate with that exact type

missing or Unknown init type:
  typed pre-allocation reject
```

This makes the skeleton representation-polymorphic for every admitted
GenericLoop carrier. Facts continue to decide control/progression shape;
Builder state supplies representation only after the selected initial value
is known.

### Candidate B-prime — role-sealed representation

Retain the already-computed `use_body_managed_step` decision as a durable
facts-side role:

```text
NumericProgression:
  existing Integer representation law

BodyManagedState:
  exact transient representation of loop_var_init
```

The facts product owns only the carrier role, not `MirType`. A Builder-side
projection combines the role with the selected init ValueId. This preserves
the current numeric rule while opening Box carriers only for the already
admitted body-managed shape.

### Candidate C — reject non-Integer candidates

Restrict GenericLoop facts to Integer progression and reject receiver method
state as a loop variable. A separate receiver-state loop route would then be
required before strict JSON can compile again.

## Recommendation

Prefer Candidate B-prime if `NumericProgression = Integer` is already a
durable GenericLoop semantic law. Prefer Candidate A if GenericLoop is meant
to be a representation-neutral carrier skeleton.

Do not select C merely to avoid the Box representation: current V1 facts
deliberately admit body-managed receiver method progression, so C requires a
new route owner rather than a local rejection patch.

The consultation must decide these exact points:

1. whether all GenericLoop carriers or only body-managed carriers derive from
   the init representation;
2. whether missing/`Unknown` init representation rejects or retains an
   Integer default under a separately sealed numeric role;
3. whether GenericLoop V0 remains Integer-only;
4. where the role/request is sealed so facts do not become a MIR type owner;
5. whether current/next/step-PHI slots must all share one representation row.

## Proposed implementation order

```text
R0-GENERICLOOP-CARRIER-TYPE0-S0
  disconnected carrier role/representation decision
  production behavior delta = 0

R0-GENERICLOOP-CARRIER-TYPE0-M0
  exact v0/v1/composer/normalizer consumer inventory
  fresh-binary T0 regression fixture
  production behavior delta = 0

R0-GENERICLOOP-CARRIER-TYPE0-I0
  one skeleton allocation connection
  no PHI decision change

R0-GENERICLOOP-CARRIER-TYPE0-G0
  consumer/representation/fresh-build guards
  strict document seal debug/release green

then:
  clean HMI-S0-V0-R0-I0 rewrite
```

The next code-facing row is `R0-GENERICLOOP-CARRIER-TYPE0-S0` only after the
candidate law is selected.

## Required fixtures

Pass after I0:

```text
Integer progression loop:
  representation and MIR parity unchanged

body-managed current receiver loop:
  init/current/next/PHI = exact same Box(owner)

JsonScanner.read_identifier:
  no concrete type conflict

s0_document_seal_test:
  fresh debug/release vm-reference binaries green

failed compile followed by independent valid compile:
  compiler reuse green
```

Reject:

```text
exact init type conflicts with sealed carrier representation
missing init ValueId
foreign/stale ValueId
missing/Unknown type when selected law requires exact representation
mixed representation inputs at PHI
second carrier type owner
```

## Counters and guards

```text
GenericLoop carrier representation decision owners = 1
skeleton carrier allocation consumers = 1

current/next/step-PHI representation mismatch = 0
PHI concrete conflict fallback = 0
PHI destination overwrite = 0

facts-side MirType storage = 0
function-name/field-name/HMI-name conditions = 0
runtime type-tag reads = 0
final metadata reads during lowering = 0
new persistent ValueId -> type maps = 0

JsonScanner source delta = 0
HMI source delta during TYPE0 = 0
fake verified view constructors = 0
fallback/retry/legacy route probing = 0
ownership opcode delta = 0
source/check files >= 800 lines = 0
```

## Implementation may claim

After the selected row is green:

```text
GenericLoop allocates every slot in one selected carrier row with one exact
representation

body-managed receiver state no longer receives an Integer destination fact

TYPE-PUBLISH0 validates the resulting PHI without weakening its conflict law

strict JSON producer-backed HMI fixtures compile on a fresh current binary
```

## Implementation must not claim

```text
general loop-carried ownership
general Box mutation semantics
arbitrary union/coercion PHIs
general type inference from AST names
all GenericLoop candidates are representation-polymorphic unless selected
loop/backedge receiver-equivalence support
HMI register completion
runtime/backend widening
```

## Stop conditions

Stop if implementation requires:

1. weakening or bypassing the PHI concrete-fact conflict;
2. overwriting a concrete destination type after PHI completion;
3. inferring type from `JsonScanner`, field, method, or HMI names;
4. storing `MirType` in syntax-only facts without a selected boundary law;
5. reading finalized metadata instead of current transient type facts;
6. a second persistent ValueId-to-type map;
7. rewriting scanner/HMI source to fit an existing loop recipe;
8. fallback, retry, legacy LoopBuilder, or environment route selection;
9. ownership syntax/opcodes or backend widening;
10. any source/check file reaching 800 lines.

## Requested answer

Please select A, B-prime, C, or a corrected candidate, and fix:

```text
carrier role authority
representation authority
missing/Unknown law
V0/V1 boundary
slot/PHI correspondence
exact task order and stop laws
```
