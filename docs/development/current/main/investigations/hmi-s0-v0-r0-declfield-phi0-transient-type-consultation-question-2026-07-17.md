---
Status: External consultation required
Date: 2026-07-17
Baseline: ad49cf77d0
Parent: hmi-s0-v0-r0-declfield-phi0-same-root-task-2026-07-17.md
Scope: transient receiver Copy/Phi type authority before declared FieldGet
---

# R0-DECLFIELD-PHI0 transient type consultation

## Late response classification

An external response received after `R0-DECLFIELD-PHI0-P0` selected the same
Candidate A-prime contract already implemented by S0/P0:

```text
one bounded use-site Copy/Phi proof
finite acyclic non-loop nested PHIs
every terminal root = exact current implicit receiver parameter 0
persistent provenance/type/origin publication = 0
```

That response is accepted as confirmation of the existing proof authority,
fixture matrix, claims, non-claims, and stop conditions. It does not create a
new task row because:

```text
R0-DECLFIELD-PHI0-S0:
  already closed

R0-DECLFIELD-PHI0-P0:
  already closed

R0-DECLFIELD-PHI0-I0:
  attempted and stopped before landing

stop reason:
  SeedTypeMissing at the exact transient FieldGet use site
```

The response does not select the transient type-publication authority, define
the PHI destination/type-hint conflict law, or identify the exact
finalization-stage publisher of the downstream Copy type. Therefore it does
not authorize resuming `R0-DECLFIELD-PHI0-I0`.

Current task classification remains:

```text
accepted and closed:
  R0-DECLFIELD-PHI0-S0
  R0-DECLFIELD-PHI0-P0

blocked:
  R0-DECLFIELD-PHI0-I0

parked behind I0:
  R0-DECLFIELD0-G0
  clean HMI-S0-V0-R0-I0 rewrite

next action:
  answer this consultation's Questions for decision
  then task-fix the selected transient type row
```

## Why implementation is stopped

`R0-DECLFIELD-PHI0-S0/P0` proved the accepted receiver identity grammar:

```text
finite acyclic non-loop Copy/Phi graph
every terminal root = exact current implicit receiver parameter 0
```

P0 also compiled the actual DECLFIELD0 app and proved, from each final
`MirFunction`, the exact normalized field-base shapes:

```text
A1 = R
A2 = P[R,R]
A3 = R
A4 = R
```

The first I0 connection preserved the existing direct-origin route and added
the same-root proof as the sole fallback in
`declared_field_type_for_value`. The selected A2 access still did not recover
its declared type.

A test-only transient rejection observer measured the exact reason:

```text
function:
  DeclaredFieldOwnerV1.declfield_probe_v1_after_validation/2

field base:
  %37

proof rejection:
  SeedTypeMissing
```

The final MIR is:

```text
%19 = phi [%0 from bb10, %0 from bb11]
%37 = copy %19
%38 = field.get %37 .items
```

After function finalization:

```text
type(%0)  = Box(DeclaredFieldOwnerV1)
type(%19) = Box(DeclaredFieldOwnerV1)
type(%37) = Box(DeclaredFieldOwnerV1)
```

But at the exact `FieldGet` lowering use site:

```text
type_ctx.value_types[%37] = missing
```

The type appears only after the function-finalization propagation sequence:

```text
finalize_function_draft
  -> TypePropagationPipeline
       Copy -> BinOp -> Copy -> Phi
```

That sequence is too late to select the declared-field and Known method
route. Its final step is PHI, so it does not by itself explain a downstream
Copy becoming typed. The exact final publisher or repeated pipeline entry
must be identified before implementation claims are fixed.

The I0 WIP is preserved only as evidence:

```text
stash hash:
  49bafef37c6a110b1fd98338bd3f95a38ebd6891

apply/pop/restore:
  forbidden
```

No failed I0 code is landed.

## Contract tension

Candidate A-prime explicitly required:

```text
every traversed value:
  value_types[value] = Box(current owner)
```

Therefore I0 cannot silently replace `SeedTypeMissing` with:

```text
same-root identity implies type
```

without changing the accepted authority.

Likewise, final MIR metadata cannot be read backwards during lowering. The
final type is evidence that the graph is type-recoverable, not proof that the
type was available at the required time.

## Existing authorities

```text
receiver identity:
  implicit parameter 0 co-validation

receiver equivalence:
  VerifiedSameRootReceiverValueV1

transient value types:
  builder.type_ctx.value_types

final whole-function type propagation:
  TypePropagationPipeline

field declaration:
  user_box_field_decls

final function validity:
  MirVerifier
```

## Non-authorities

```text
final MIR metadata read backwards during Lower
FieldGet result type
method route result
runtime class tags
current_static_box
function/method/field name heuristics
HMI source shape
Python root scanner
stash contents
```

## Observed lifecycle bypass

The worker audit found a concrete construction asymmetry:

```text
raw MirBuilder::emit_instruction(Phi):
  invokes origin::phi::propagate_phi_meta
  -> unanimous input type may be published

If/Binding-SSA lifecycle Phi:
  define_phi_final_with_type_hint
  -> insert_phi_at_head_spanned_with_type_hint
  -> bypasses MirBuilder::emit_instruction
  -> unanimous type publication is skipped
```

The selected chain is:

```text
typed receiver param0
  -> lifecycle Phi(receiver, receiver)
       transient dst type missing
  -> LocalSSA Copy
       source type missing, so Copy dst type missing
  -> FieldGet
       same-root proof rejects SeedTypeMissing
```

The proof failure is therefore consistent with an existing PHI lifecycle
entry bypass rather than evidence that receiver identity should infer type.

## Candidate A-prime — repair transient type publication

Extract one neutral unanimous-PHI-type helper from the existing combined
`origin::phi::propagate_phi_meta` policy, and call it from every canonical
Builder PHI lifecycle completion entry.

Required shape:

```text
inputs:
  all input types present
  all input types exactly equal

publication:
  type_ctx.value_types[phi_dst]

non-publication:
  value_origin_newbox
```

The existing LocalSSA Copy metadata propagation should then type the selected
post-PHI Copy without a new Copy policy.

Advantages:

```text
keeps the accepted receiver-proof A-prime unchanged
one transient type SSOT remains
all later consumers see the same fact
repairs an existing raw-PHI/lifecycle-PHI entry asymmetry
```

Risks:

```text
behavior delta is broader than declared-field lookup
raw/final/provisional/patched/batch PHI entries must share one decision source
existing concrete/type-hint conflict law must be fixed
unrelated route selection may change when types become available earlier
```

This candidate requires a disconnected generic proof before any production
repair.

Required proof matrix:

```text
receiver param0 -> lifecycle Phi(receiver, receiver):
  dst typed before finalization

LocalSSA Copy(Phi):
  copy dst typed immediately

nested unanimous PHIs:
  typed

missing / heterogeneous / foreign input:
  no publication

existing concrete dst or type hint conflict:
  exact decision-locked failure/preservation law

raw / final / provisional / patched / batch PHI entries:
  one policy and normalized parity

value_origin_newbox writes:
  0

selected A2:
  transient same-root proof succeeds before finalization

unrelated routes:
  inventoried; unexpected route widening stops the row

early type and final propagation result:
  exact parity
```

## Candidate B — co-seal type in the same-root proof

Change the proof law to:

```text
terminal receiver parameter:
  exact Box(owner)

ordinary Copy/Phi closure:
  missing type allowed
  explicit foreign/mismatched type rejected

proof result:
  receiver identity + Box(owner) representation
```

The proof remains ephemeral and one-consumer, and still writes no type/origin
metadata.

Advantages:

```text
narrow declared-field behavior delta
no persistent type map mutation
identity and representation are proven from the same bounded graph
```

Risks:

```text
changes Candidate A-prime's value-level admission
receiver-equivalence proof becomes a type authority
could duplicate TypePropagationPipeline reasoning
```

## Candidate C — bounded read-only type propagation view

Build an ephemeral type view over the current incomplete function using the
same canonical Copy/Phi rules as `TypePropagationPipeline`, then require:

```text
view type = Box(current owner)
same-root receiver proof = accepted
```

Advantages:

```text
does not mutate transient type_ctx
can reuse a shared propagation policy
```

Risks:

```text
creates a second timing-specific type product
the full pipeline is designed for completed functions
incomplete CFG/function state may invalidate its assumptions
larger authority than one blocker
```

Running the full `TypePropagationPipeline` mutably in the middle of expression
lowering is not proposed.

## Candidate D — infer from field or downstream route

```text
current receiver shape
  -> field registry
  -> assume base Box(owner)
```

Reject.

This is circular: the field lookup would provide the type required to
authorize the field lookup.

## Questions for decision

1. Is the missing transient type a bug in an existing publication/lifecycle
   owner, or may same-root receiver identity itself imply `Box(owner)`?
2. If Candidate A-prime is selected, must every canonical Builder PHI
   lifecycle entry share one unanimous type publication helper with raw Phi
   emission?
3. Should the helper publish only when destination type is missing/Unknown,
   and what is the exact fail-fast law for a conflicting concrete type or
   type hint?
4. If Candidate B is selected, may missing types be admitted only when every
   terminal root is the exact typed receiver, while explicit mismatches and
   foreign origins still reject?
5. Does Candidate B improperly duplicate `TypePropagationPipeline`, or is
   identity-preserving Copy/Phi representation a valid part of the same sealed
   receiver proof?
6. Should Candidate C remain parked until a second independent mid-lowering
   consumer needs the same bounded type view?
7. Which exact finalization-stage owner types the final downstream Copy, given
   that the documented pipeline ends with PHI?
8. What is the exact next task order and first code-facing row?

## Recommended provisional order

Do not resume PHI0-I0 directly.

If Candidate A-prime:

```text
R0-DECLFIELD-TYPEPUB0-S0
  -> M0
  -> I0
  -> G0
  -> resume R0-DECLFIELD-PHI0-I0
```

If Candidate B:

```text
R0-DECLFIELD-PHI0-T0
  -> extend disconnected proof matrix
  -> resume R0-DECLFIELD-PHI0-I0
```

## Stop conditions

Stop if any proposal requires:

1. reading final MIR metadata backwards during Builder lowering;
2. field/method/HMI-name special cases;
3. runtime class tags or downstream method-route backfeed;
4. mutating `value_origin_newbox`;
5. a second persistent `ValueId -> type/owner` map;
6. running the whole mutable finalization pipeline mid-expression;
7. accepting Select, CopyOwned, Call, FieldGet, or loop PHIs;
8. using the stash as implementation authority;
9. fallback or retry after proof failure;
10. a source/check file reaching 800 lines.

## Exact consultation request

Select Candidate A, B, C, or another exact alternative, and lock:

```text
type authority
proof admission
publication timing
consumer count
task order
pass/reject fixtures
claims/non-claims
stop conditions
```

No compiler implementation is authorized by this consultation document.
