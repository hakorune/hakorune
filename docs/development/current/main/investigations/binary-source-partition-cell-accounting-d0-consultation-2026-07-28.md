---
Status: parked behind structural budget D0
Date: 2026-07-28
Decision: pending
Question: may one production replacement cell own one source selector partition with two disjoint semantic owners
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# Binary source-partition cell accounting D0

## Why this consultation exists

The sixth Return cell is closed. A four-worker bounded census leaves Binary as
the only clean seventh-edge candidate, but exposes one accounting boundary:

```text
one ASTNode::BinaryOp production selector
two mutually exclusive semantic owners
one dead predecessor selector chain
```

The code deletion is mechanically bounded. The unresolved question is whether
the replacement manifest should credit the source partition as one cell or
credit Ordinary Binary and ShortCircuit as separate semantic cells.

No source, test, guard, or manifest row may change until this D0 is resolved.

This accounting decision is now subordinate to:

```text
docs/development/current/main/investigations/
mirbuilder-structural-budget-d0-consultation-2026-07-28.md
```

Binary selection resumes only after the final owned-footprint caps are
accepted. The cap may change which accounting option is structurally valid.

## Exact current production graph

There is exactly one raw/default `ASTNode::BinaryOp` selector:

```text
src/mir/builder/raw_expression_dispatch/mod.rs

ASTNode::BinaryOp
  -> And / Or
     -> RawLegacyShortCircuitInputV1
     -> drive_short_circuit_expression_v1

  -> every other operator
     -> RawLegacyBinaryInputV1
     -> drive_ordinary_binary_expression_v1
```

Caller census:

```text
ordinary generic owner:
  raw/default caller                    = 1
  detached located caller               = 1

ShortCircuit generic owner:
  raw/default caller                    = 1
  detached located caller               = 1

detached located production root        = 0
fallback / retry / reselection           = 0
```

The located calls are in `located_legacy_lowering.rs` and remain inactive at
the production root.

## Exact obsolete predecessor chain

Three production symbols remain:

```text
MirBuilder::build_binary_op
drive_raw_ordinary_binary_expression_v1
drive_raw_short_circuit_expression_v1
```

Their graph is:

```text
build_binary_op                         callers = 0
  -> ordinary raw facade               callers = 1, dead shell only
  -> ShortCircuit raw facade           callers = 1, dead shell only
```

Neither raw facade has a production or test consumer outside the dead shared
facade. The complete chain can be physically deleted without changing the live
selector.

Bounded gross production deletion is approximately 51 lines:

```text
shared facade and stale owner docs       about 29
ordinary raw facade                      about 10
ShortCircuit raw facade                  about 10
unused raw-port imports                  about 2
```

Exact closeout authority would be final `src/**/*.rs` numstat.

## Why the owners are semantically distinct

Ordinary Binary owns:

```text
left child exactly once
-> right child exactly once
-> build_binary_op_from_values
```

It rejects `And` and `Or` before ordinary child effects.

ShortCircuit owns:

```text
lhs first
-> existing logical CFG/PHI owner
-> RHS only inside the eval-RHS block
```

It rejects ordinary operators.

They have different input types, child-demand laws, completion owners, focused
tests, and parity references. A combined cell must not claim they are one
semantic owner.

## Why the physical cutover is one partition

The live raw/default selector is one `ASTNode::BinaryOp` match and the obsolete
predecessor is one shared `build_binary_op` selector. Deleting the shared shell
necessarily removes both of its dead calls.

The Return T1 precedent allowed one exact source partition to retain a
value-bearing owner and install a distinct Void leaf in one cell. Binary is
even narrower operationally: both target owners are already live and no
interface changes.

This supports accounting the responsibility as:

```text
ASTNode::BinaryOp operator-family source partition
```

rather than claiming a shared Binary semantic owner.

## Existing proof surfaces

Ordinary:

```text
binary_expression_descent_tests
binary_expression_raw_tests
binary_expression_parity_tests
```

ShortCircuit:

```text
short_circuit_expression_descent_tests
short_circuit_expression_raw_tests
short_circuit_expression_parity_tests
```

The raw suites deliberately cross-check the selector partition: the ordinary
raw suite sends And/Or through the production ingress, and the ShortCircuit raw
suite sends an ordinary operator through it.

The public EXPR0 structural helper is stale and currently fails first at:

```text
BIN0-I0 raw implementation: expected=1 actual=0
```

It still requires both raw facades and the dead ops-root selector. A selected
implementation must preserve both semantic proof sets while retiring only the
old-facade existence assertions.

## Rolling LOC boundary

After the sixth cell:

```text
cell 3    +44
cell 4    -52
cell 5    -77
cell 6   -141
          ----
base     -226
```

The seventh cell may be at most `+226` while keeping the five-cell rolling
budget non-positive. The bounded facade deletion should be negative; no
unrelated cleanup is authorized to inflate repayment.

## Option A — one source-partition cell

```text
BINARY-SOURCE-PARTITION-CUTOVER0-I0-R0
Pack: DESCENT-SPINE0
Ceremony: T0
```

One row owns the exact `ASTNode::BinaryOp` operator-family partition:

```text
new owners:
  drive_ordinary_binary_expression_v1
  drive_short_circuit_expression_v1

delete:
  build_binary_op
  drive_raw_ordinary_binary_expression_v1
  drive_raw_short_circuit_expression_v1
```

Acceptance records two disjoint owners and two independent parity suites. It
does not merge their semantics.

Advantages:

```text
one live selector = one manifest responsibility
one dead predecessor chain is removed atomically
no orphan raw facade remains between commits
shared guard can assert the complete source partition
```

Risk:

```text
the manifest may be interpreted as crediting two semantic replacements in one
cell, weakening one-responsibility accounting
```

## Option B — three accounting cells

First:

```text
BINARY-DEAD-SHARED-SELECTOR-RETIRE0
```

Delete only `build_binary_op`, leaving both now-orphaned raw facade definitions.

Then:

```text
ORDINARY-BINARY-DESCENT-CUTOVER0
SHORT-CIRCUIT-DESCENT-CUTOVER0
```

Each credit row deletes its own raw facade and guards its own semantic owner.

Advantages:

```text
one semantic owner per credit row
separate parity and acceptance matrices
```

Costs:

```text
the first row has no live replacement owner
temporary orphan facades remain by design
three commits describe one already-completed production switch
the shared-shell row may not satisfy production replacement cell policy
```

## Option C — first family absorbs the shared shell

Credit one family first, delete `build_binary_op` and that family's raw facade,
then credit the sibling and delete its orphan raw facade.

This avoids a proof-only preliminary row but makes the first family responsible
for incidental deletion of its sibling's old caller. The accounting is
asymmetric and commit order becomes policy.

## Recommendation

Recommend Option A, with a narrow claim:

```text
one source-partition responsibility
two explicitly disjoint semantic owners
two independent parity gates
one atomic predecessor-chain deletion
```

This matches the physical production selector and eliminates all obsolete
authority without inventing orphan phases. Acceptance and manifest wording must
say “operator-family source partition”; they must not say Ordinary and
ShortCircuit share semantics.

If policy defines a cell strictly as one semantic owner rather than one exact
production responsibility, reject A and choose B. Do not silently choose C.

## Rejected seventh candidates

### Statement-position If

The live statement route is:

```text
block_stmt ASTNode::If
-> drive_raw_if_statement_with_port_v1
-> drive_if_statement_v1
```

The with-port adapter is not obsolete. It owns the borrowed-child boundary and
unknown-span Program shell. Previously selected old symbols are already absent,
so a T0 credit would have no delete target and zero production LOC.

Expression-position and canonical resolved If are separate routes. Removing the
adapter or joining those routes requires at least T1.

### Non-Program module root

The edge:

```text
other => self.build_expression(other)
```

is live compatibility authority, not dead fallback.

Non-test direct `build_module` callers include:

```text
MirCompiler::compile_legacy_candidate
runtime/mirbuilder_emit.rs AST-JSON bridge
```

Bare scalar, `BoxDeclaration`, and static `Main` roots have different behavior.
Replacing the edge with the invocation child port rejects bare Main as a nested
Main and changes non-Main Box publication. Scalar tests do not prove Box/Main
parity.

Additionally, `module_lifecycle.rs` is 799 lines. Any correct new owner or test
seam first requires a physical BoxShape split. This is T1 only after a root
source-partition decision; changing bare Box/Main acceptance or AST-JSON grammar
is T2.

## Hard stops for either Binary answer

```text
do not change operator vocabulary
do not change eager-vs-conditional child demand
do not change build_binary_op_from_values
do not change logical CFG/PHI completion
do not activate located lowering
do not add compatibility facade, retry, or route probing
do not touch If, non-Program root, runtime, backend, language, or selfhost
do not repair unrelated proof families
do not create a manifest row before this D0 is resolved
```

## Decision requested

Choose one:

```text
A: one Binary source-partition cell with two explicit semantic owners
B: dead shared-selector retirement plus two semantic credit cells
```

If neither is accepted, specify the replacement-cell accounting law that the
seventh edge must satisfy. Source implementation remains stopped.
