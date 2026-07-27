---
Status: accepted next execution task; blocked on minimal structural ratchet
Date: 2026-07-28
Decision: BINARY-SOURCE-PARTITION-CUTOVER0-I0-R0; Option A accepted
Responsibility: raw/default ASTNode::BinaryOp operator-family source partition
Prerequisite:
  - MIRBUILDER-STRUCTURAL-BUDGET0-CLOSEOUT
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# Binary source-partition cell accounting D0

## Accepted decision

Option A is accepted:

```text
BINARY-SOURCE-PARTITION-CUTOVER0-I0-R0
Pack: DESCENT-SPINE0
Ceremony: T0

one raw/default source selector
two disjoint semantic owners
one atomic obsolete predecessor chain
```

This decision does not activate source edits or a seventh manifest row.
Execution remains blocked until the minimal four-metric structural ratchet is
closed.

## Why this consultation exists

The sixth Return cell is closed. A four-worker bounded census leaves Binary as
the only clean seventh-edge candidate, but exposes one accounting boundary:

```text
one ASTNode::BinaryOp production selector
two mutually exclusive semantic owners
one dead predecessor selector chain
```

The code deletion is mechanically bounded. This consultation originally asked
whether the replacement manifest should credit the source partition as one
cell or credit Ordinary Binary and ShortCircuit as separate semantic cells.

The accounting question is now resolved. No Binary source, test, guard, or
manifest row may change until the structural-ratchet prerequisite closes.

Execution of the accepted accounting decision is blocked by:

```text
docs/development/current/main/investigations/
mirbuilder-structural-budget-d0-consultation-2026-07-28.md
```

Binary execution activates immediately after the minimal four-metric ratchet
is installed. Structural size is a result metric and did not decide the
semantic cell-accounting answer.

Execution authority for that prerequisite:

```text
docs/development/current/main/investigations/
mirbuilder-structural-budget0-closeout-task-2026-07-28.md
```

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

## Accepted accounting

Option A is accepted with this narrow claim:

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

Option B and Option C are rejected. Ordinary and ShortCircuit remain separate
semantic owners and may not be credited again by later replacement cells.

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
raw/default ASTNode::BinaryOp selector is not exactly one
build_binary_op has an external executable caller
either raw facade has a consumer other than the dead build_binary_op chain
generic owner, completion owner, or located adapter interface must change
resolved/Facts Binary routes must change
compatibility facade is required
final production Rust LOC delta is non-negative

do not change operator vocabulary
do not change eager-vs-conditional child demand
do not change build_binary_op_from_values
do not change logical CFG/PHI completion
do not activate located lowering
do not add compatibility facade, retry, or route probing
do not touch If, non-Program root, runtime, backend, language, or selfhost
do not repair unrelated proof families
do not create a manifest row before the structural ratchet closes
```

## Execution contract

The prerequisite closeout moves the current pointer directly to this task.
The Binary implementation is one atomic I0/R0 commit; no additional selection
card or per-cell guard is created.

Keep unchanged:

```text
raw_expression_dispatch::ASTNode::BinaryOp
  And / Or
    -> RawLegacyShortCircuitInputV1
    -> drive_short_circuit_expression_v1

  all remaining operators
    -> RawLegacyBinaryInputV1
    -> drive_ordinary_binary_expression_v1
```

Delete:

```text
MirBuilder::build_binary_op
drive_raw_ordinary_binary_expression_v1
drive_raw_short_circuit_expression_v1
unused ASTNode / RawLegacyChildLoweringPortV1 imports
stale facade-owner documentation
```

The current exact source shape predicts:

```text
production Rust LOC delta = -54
```

Closeout authority is the final `src/**/*.rs` numstat.

Update in the same commit:

```text
existing EXPR0 helper:
  remove old-facade existence authority
  require the two live raw/default branches
  require the blanket owner-port implementations
  require external non-test generic sites = 2 per owner
  remove stale ops-root selector/order assertions
  update stale README phrases and printed summary
  retain independent semantic-order proofs

existing shared replacement guard:
  Binary manifest row closed = 1
  raw/default owner callers           = 1 each
  raw/default input constructors       = 1 each
  detached located callers             = 1 each
  external non-test generic sites      = 2 per owner
  obsolete call-shaped source symbols  = 0
  owner retry / fallback               = 0

ops README / module comments:
  describe the live partition, not the dead selector chain
```

The zero-symbol assertions apply to executable `src/**/*.rs` call-shaped
sites. Historical docs and generated evidence may retain symbol names as
history; they are not production authority.

No new test file is required. Keep all six existing owner/raw/parity suites.

Focused gates:

```bash
cargo test -q binary_expression --lib
cargo test -q short_circuit_expression --lib
cargo test -q located_legacy_lowering --lib
cargo test -q located_short_circuit_lowering --lib
cargo check -q
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Acceptance:

```text
raw/default Binary selector                  = 1
ordinary raw/default generic caller          = 1
ShortCircuit raw/default generic caller      = 1
ordinary detached located caller             = 1
ShortCircuit detached located caller         = 1
ordinary external non-test generic sites     = 2
ShortCircuit external non-test generic sites = 2
detached production root                     = 0

build_binary_op                               = 0
drive_raw_ordinary_binary_expression_v1       = 0
drive_raw_short_circuit_expression_v1         = 0

ordinary parity / failure / reuse             = green
ShortCircuit parity / failure / reuse         = green
wrong-family production-ingress checks        = green
operator / demand / CFG / PHI delta           = 0
fallback / retry / reselection                = 0
new proof file                                = 0
production Rust LOC delta                     < 0
all touched source/check files                < 800 lines
```
