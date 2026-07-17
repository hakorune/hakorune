---
Status: External consultation required; one-shot packet
Date: 2026-07-17
Baseline: dfe1fc9ca8
Evidence: hmi-s0-v0-r0-declfield-phi0-type-publish0-consultation-question-2026-07-17.md
Scope: one coherent lowering-time PHI type-publication decision
---

# TYPE-PUBLISH0 one-shot D0 decision packet

## Why this packet exists

Three read-only worker audits found that the current selfhost dependency chain
has exactly one genuinely open consultation:

```text
R0-DECLFIELD-PHI0-TYPE-PUBLISH0-D0
```

Older HMI consultation documents with `Design consultation stop` or
`External consultation required` headers are answered, superseded, or landed
history. They are not parallel work queues.

The detailed evidence card remains the source for code locations, lifecycle
inventory, existing raw/final semantics, fixtures, counters, and stop laws.
This packet compresses its open questions into four decisions that must be
answered together.

## Fixed evidence

```text
receiver-equivalence proof:
  closed

accepted provenance:
  exact current receiver parameter 0
  ordinary Copy
  finite acyclic non-loop Phi

selected transient failure:
  %37 = Copy(%19)
  %19 = Phi(%0, %0)
  %0  = exact current receiver parameter 0
  proof result at FieldGet use site = SeedTypeMissing

raw Builder Phi:
  has permissive pre-emission type+origin propagation

final/patch/batch lifecycle Phi:
  bypass raw propagation

final TypePropagationPipeline:
  too late and intentionally more permissive/corrective
```

The receiver proof must continue to observe an already-published exact type.
It must not infer or backfill one.

## Decision 1 — candidate type law

Select the exact lowering-time candidate law.

Recommended law:

| Input type set | Candidate result |
| --- | --- |
| every input present, same concrete type, not `Unknown`/`Void` | `Publish(type)` |
| one or more input missing | `NoPublication(MissingInputType)` |
| one or more input `Unknown` | `NoPublication(UnknownInputType)` |
| one or more input `Void` | `NoPublication(VoidInputType)` |
| two or more different concrete types | `NoPublication(HeterogeneousInputTypes)` |
| provisional Phi with zero inputs | `NoPublication(ProvisionalEmpty)` |

`NoPublication` is a typed decision, not an arbitrary type selection and not
automatically a source compilation error. Existing dynamic/mixed PHI grammar
must not be narrowed merely because no exact transient type is published.

The decision owner must not use:

```text
field or method names
receiver equivalence
runtime tags
final metadata
final PhiTypeResolver
type_hint as a candidate generator
```

## Decision 2 — destination and hint constraints

Select how an exact candidate interacts with an existing destination type and
an explicit instruction `type_hint`.

Recommended law:

| Candidate | Existing destination | Explicit hint | Result |
| --- | --- | --- | --- |
| exact `T` | absent/`Unknown` | absent | publish `T` |
| exact `T` | exact `T` | absent | preserve idempotently |
| exact `T` | absent/`Unknown` | exact `T` | publish `T` |
| exact `T` | exact `T` | exact `T` | preserve idempotently |
| exact `T` | concrete non-`T` | any | typed conflict before mutation |
| exact `T` | any | concrete non-`T` | typed conflict before mutation |
| no candidate | any | any | do not infer from destination or hint |

The answer must define whether `Unknown` and `Void` are placeholders or
concrete constraints in each column. The recommendation treats `Unknown` as
an absent publication fact and `Void` as a non-candidate input; an explicit
concrete destination/hint cannot manufacture a candidate when the input set
does not prove one.

Recommended typed vocabulary:

```text
PhiTransientTypeDecisionV1:
  Publish { ty }
  NoPublication { reason }

PhiTransientTypeConflictV1:
  ExistingDestinationMismatch { dst, candidate, existing }
  ExplicitHintMismatch { dst, candidate, hint }
```

Exact names may change, but one typed decision/error owner is required.

## Decision 3 — authorized producer set

Select which entries consume the single decision owner.

Recommended table:

| Entry | Classification | Publication point |
| --- | --- | --- |
| raw `MirBuilder::emit_instruction(Phi)` | authorized Builder consumer | after successful raw emission |
| `define_phi_final_with_type_hint` | authorized Builder consumer | after successful complete insertion |
| `define_provisional_phi` | non-publisher | never while inputs are empty |
| `patch_phi_inputs` | authorized Builder consumer | after successful patch completion |
| `define_phi_batch_prepend` | authorized Builder consumer | after successful atomic batch insertion |
| function-level `define_phi_final_fn*` | explicit non-consumer | no Builder `type_ctx` authority |
| post-create input/block rewriters | preserve-premises observer in M0 | no new publication authority |

Thin aliases and `PhiTxn` do not become policy owners.

The existing raw unanimous-origin behavior remains a separate owner. New
lifecycle consumers publish type only:

```text
new lifecycle value_origin_newbox writes = 0
```

Function-level APIs must not gain hidden Builder state or eager
`function.metadata.value_types` writes in this row.

## Decision 4 — timing and atomicity

Select one transaction law for raw, final, patch, and batch entries.

Recommended order:

```text
1. normalize/materialize every incoming value
2. compute every type decision
3. validate every destination/hint conflict
4. mutate the Phi instruction or complete batch
5. publish every prevalidated type non-fallibly
6. return success
```

Required failure law:

```text
decision/conflict failure:
  instruction delta = 0
  transient type delta = 0

low-level insertion/patch failure:
  transient type delta = 0

batch decision/conflict/insertion failure:
  batch instruction delta = 0
  batch transient type delta = 0

successful lifecycle completion:
  destination type visible before the next Copy/FieldGet/method route
```

Raw origin publication must also leave no partial metadata when raw emission
fails. If preserving its current ordering is required, explain the exact
transaction boundary rather than copying its pre-emission side effect into
new entries.

## Requested answer format

Please return only these four coordinated outputs:

1. selected candidate-law table;
2. exact typed decision/error taxonomy and destination/hint precedence;
3. authorized producer/non-consumer table;
4. confirmed task and transaction order.

A concise answer such as `accept recommendations 1-4` is sufficient if every
recommended table and stop law is accepted unchanged.

## Pre-authorized implementation runway

After D0 is answered, no separate external consultation is required at every
checkpoint. Proceed serially:

```text
TYPE-PUBLISH0-S0
  disconnected decision/error product
  production consumers = 0

TYPE-PUBLISH0-M0
  exact entry/rewriter/timing inventory
  selected %19 -> Copy %37 proof

TYPE-PUBLISH0-I0
  connect all and only authorized Builder consumers

TYPE-PUBLISH0-G0
  decision/consumer/origin/atomicity guards

existing PHI0-I0
  one declared-field fallback consumer

existing DECLFIELD0-G0
  publish MAPFIELD-R0-DECLFIELD0

clean HMI-S0-V0-R0 implementation
  resume the already decision-locked register/snapshot task
```

S0/M0/I0/G0 are checkpoints in one semantic row. Do not reopen design review
between them unless a listed stop condition is observed.

The following downstream contracts are already decided and must not trigger
new consultation without new contradictory evidence:

```text
receiver Copy/Phi proof grammar
declared-field one-consumer fallback
HMI scalar register/snapshot/state shape
HMI exact scalar opcode subset
strict MIR JSON document seal
common-domain Rust-oracle parity
```

## Evidence-gated future decisions — do not ask now

These require later implementation evidence and are not part of this packet:

```text
HMI-S1 exact normalized event carrier/API
SSA-I1-O1 first exact BoxRef source producer/profile
HMI-C0 product cutover caller/capability wiring
HMI-X0 first expansion family
HMI-R1/R2 physical Rust caller retirement
next selfhost MirBuilder/parser acceptance row
```

Broad laws for those families are already documented. Their exact choices are
consumer- or fixture-driven and cannot be selected responsibly now.

## Out of scope

Do not decide in this answer:

```text
function-level MirFunction PHI type authority
final PhiTypeResolver redesign
general origin propagation
loop/backedge PHI semantics
general union/dynamic typing
HMI handler/JSON/parser semantics
ownership/view/share grammar
backend or product cutover
general selfhost parser migration order
```

## Shared stop conditions

Stop the implementation runway only if it requires:

1. narrowing accepted source grammar by rejecting every non-publishable Phi;
2. per-entry type policy or a second persistent `ValueId -> type` map;
3. writing final metadata or lifecycle `value_origin_newbox`;
4. running final type propagation during expression lowering;
5. deriving type from receiver/field/HMI names or runtime tags;
6. granting function-level APIs hidden Builder type authority;
7. changing CFG/SSA/loop/backedge semantics in the same row;
8. partial batch mutation/publication without rollback;
9. fallback, retry, legacy probing, or stash restoration;
10. a source/check file reaching 800 lines.

## Decision lock requested

> Select one coherent D0 for candidate formation, destination/hint
> constraints, authorized Builder producers, and prevalidate-mutate-publish
> atomicity. Once selected, authorize the fixed S0/M0/I0/G0 runway and the
> already-decided PHI0-I0/DECLFIELD0/HMI-R0 continuation without repeated
> consultation, unless an explicit stop condition is observed.

