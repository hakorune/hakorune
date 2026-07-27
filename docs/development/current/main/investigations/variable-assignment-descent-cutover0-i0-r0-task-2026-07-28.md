---
Status: accepted execution task
Date: 2026-07-28
Decision: VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0-I0-R0
Pack: DESCENT-SPINE0
Ceremony: T0
BoxShape: credit-live-replacement-and-retire-obsolete-authority
Commits:
  - one SSOT selection commit
  - one immediately-following atomic I0/R0 implementation commit
Parent:
  - docs/development/current/main/investigations/
    mirbuilder-fifth-production-edge-consultation-2026-07-28.md
Policy:
  - docs/development/current/main/design/
    mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/
    mirbuilder-inplace-replacement-current.md
---

# VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0-I0-R0

## Decision

Credit the already-live variable-name binding reassignment descent owner
across its two bounded raw/default syntax ingresses, delete the two obsolete
Legacy facades left behind by that historical cutover, remove the sole
facade-only test, correct stale Assignment proof authority, and transfer the
production-edge proof to the shared in-place replacement guard.

This cell does not create a second Assignment implementation or admit a new
source shape.

## Four-worker audit correction

The 2026-07-28 read-only audit corrected two claims from the consultation
answer:

```text
selected raw/default callers are not globally one
generic external call-shaped sites are not globally one
```

The exact current graph is:

```text
exact Variable-target raw/default caller = 1
GroupedAssignmentExpr raw/default caller = 1
raw/default callers total                = 2

detached located caller                  = 1
detached located production root ingress = 0

post-retirement generic external sites   = 3
```

One policy audit recommended limiting the cell to exact Variable-target
Assignment. The final bounded decision keeps Grouped in the same cell because
both raw/default surfaces already construct the same
`RawLegacyVariableAssignmentInputV1`, invoke the same generic owner, perform
the same variable-name binding reassignment responsibility, and leave one
zero-caller Grouped facade plus false guard/README authority if separated.

This is one BoxShape cleanup of one driver-input responsibility, not two new
accepted shapes. If implementation reveals different semantic interfaces or
parity requirements between the two ingresses, Hard stop rather than widening
the cell.

## Exact replacement cell

```text
cell_id:
  VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0

responsibility:
  variable-name binding reassignment RHS associated-input descent

raw/default syntax ingresses:
  1. src/mir/builder/raw_expression_dispatch/statement_surface.rs
     exact ASTNode::Variable target inside ASTNode::Assignment

  2. src/mir/builder/raw_expression_dispatch/mod.rs
     ASTNode::GroupedAssignmentExpr

new owner:
  src/mir/builder/stmts/variable_assignment_descent.rs
  drive_variable_assignment_v1

shared input:
  RawLegacyVariableAssignmentInputV1

completion owner:
  MirBuilder::build_assignment_from_value

old symbols to delete:
  src/mir/builder/stmts/variable_assignment_descent.rs
    drive_raw_variable_assignment_v1

  src/mir/builder/builder_build.rs
    MirBuilder::build_grouped_assignment

preserved sibling owners:
  field assignment
  index assignment
  compound assignment

preserved detached caller:
  src/mir/builder/located_legacy_assignment.rs
  drive_variable_assignment_v1
  production root ingress = 0

fallback / retry / reselection:
  forbidden
```

## Bounded caller census

Latest-main exact evidence:

```text
drive_variable_assignment_v1 definition:
  stmts/variable_assignment_descent.rs:82

exact Variable-target production call:
  raw_expression_dispatch/statement_surface.rs:204-206

GroupedAssignmentExpr production call:
  raw_expression_dispatch/mod.rs:179-181

detached located call:
  located_legacy_assignment.rs:42-53

drive_raw_variable_assignment_v1:
  definition                 = 1
  production callers         = 0
  cfg(test) callers          = 1

build_grouped_assignment:
  definition                 = 1
  repository callers         = 0

generic owner:
  raw/default callers        = 2
  detached callers           = 1
  external non-test sites    = 3

fallback / retry / probing   = 0
```

The generic owner definition and focused owner tests are not external caller
sites.

## Structural boundary

`drive_variable_assignment_v1` already owns:

```text
observe variable name once
-> declared-binding preflight
-> request RHS input once
-> lower RHS once through shared recursive child descent
-> build_assignment_from_value completion
```

Do not change this interface or `build_assignment_from_value`.

Keep:

```text
RawLegacyVariableAssignmentInputV1
VariableAssignmentSyntaxViewV1
VariableAssignmentDescentPortV1
blanket RawAstChildLoweringPortV1 implementation
drive_variable_assignment_v1
AssignmentResolverBox::ensure_declared
build_assignment_from_value
```

Field, Index, and Compound Assignment remain on their existing owners.

## Atomic implementation

### 1. Reconfirm the exact census

```bash
rg -n -P '\b(?:fn\s+)?drive_raw_variable_assignment_v1\s*\(' \
  src --glob '*.rs'

rg -n -P '\b(?:fn\s+)?build_grouped_assignment\s*\(' \
  src --glob '*.rs'

rg -n -P '\bdrive_variable_assignment_v1\s*\(' \
  src --glob '*.rs'
```

Hard stop if either old symbol has a new non-test caller or if the three
external generic sites cannot be classified exactly as two raw/default plus
one detached located.

### 2. Delete the obsolete raw facade

Remove from `variable_assignment_descent.rs`:

```text
drive_raw_variable_assignment_v1
now-unused RawLegacyChildLoweringPortV1 import
stale "Disconnected" module wording
```

The module header must describe the live raw/default variable-name Assignment
owner without claiming located production activation.

### 3. Delete the dead Grouped facade

Remove from `builder_build.rs`:

```text
MirBuilder::build_grouped_assignment
its stale pre-ASN0 documentation
```

Do not move or modify `build_assignment_from_value`.

### 4. Preserve both live production dispatchers

Do not change behavior in:

```text
statement_surface exact Variable target
  -> RawLegacyVariableAssignmentInputV1::new
  -> drive_variable_assignment_v1

raw_expression_dispatch GroupedAssignmentExpr
  -> RawLegacyVariableAssignmentInputV1::new
  -> drive_variable_assignment_v1
```

Only the false comment saying Grouped remains outside ASN0 may be corrected.
No new selector, facade, adapter, fallback, or retry is allowed.

## Test migration

### Delete the facade-only fixture

Delete:

```text
variable_assignment_descent_tests::
  raw_facade_reuses_recursive_binary_rhs_and_existing_completion
```

It is the sole caller of `drive_raw_variable_assignment_v1`. Remove any helper
or import used only by that fixture after exact compiler confirmation.

Do not replace it with a compatibility test helper.

The real exact-Variable production ingress is already covered by:

```text
variable_assignment_raw_tests::
  raw_variable_assignment_selects_owned_descent_and_recursive_rhs
```

### Rename the Grouped production-ingress fixture

Rename:

```text
grouped_assignment_remains_on_its_legacy_facade
```

to:

```text
grouped_assignment_selects_owned_descent_through_production_ingress
```

Its body already enters through
`builder.build_expression(ASTNode::GroupedAssignmentExpr)`, observes binding
replacement, and checks one `ReleaseStrong`. No behavior change or duplicate
fixture is required.

### Preserve the parity domain

Keep all five `variable_assignment_parity_tests` fixtures unchanged.

They compare the exact Variable-target selected path with a cfg(test)-only
pre-I0 reference. That reference intentionally rejects Grouped. Therefore:

```text
exact Variable-target snapshot parity = claimed
Grouped exact historical snapshot parity = not claimed
Grouped production-ingress behavior = claimed
```

Do not widen the historical reference to Grouped in this cell.

## Repair the private ASN0 helper

Update:

```text
tools/checks/lib/
callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment.py
```

The helper is already stale on clean main:

```text
reads deleted src/mir/builder/exprs.rs
expects a concrete RawLegacyChildLoweringPortV1 implementation
requires drive_raw_variable_assignment_v1
expects generic external callers = 1
expects raw facade callers = 1
expects Grouped to call build_grouped_assignment
```

Replace those assertions with:

```text
blanket raw-port implementation = 1

statement_surface:
  exact Variable driver caller = 1
  exact Variable RawLegacy input constructor = 1

raw_expression_dispatch/mod:
  Grouped driver caller = 1
  Grouped RawLegacy input constructor = 1

located_legacy_assignment:
  detached driver caller = 1

raw/default callers total = 2
detached callers = 1
external non-test driver sites = 3

drive_raw_variable_assignment_v1 call-shaped sites = 0
build_grouped_assignment call-shaped sites = 0
```

The helper must not count words in comments or debug strings.

Preserve:

```text
syntax -> preflight -> RHS input -> RHS descent -> completion order
undeclared target rejection before RHS effects
completion-time declaration recheck
RHS failure preserves previous binding
field / index / compound owners
exact local-contract reassignment
typed-array reassignment
same-Builder reuse
parity snapshots
located AssignmentValue role and inactive proof
no AST reconstruction
no retry / fallback
stack-scoped and <800-line boundaries
```

Direct focused helper execution must become green. The public EXPR0 parent
remains outside this cell because it is red earlier on unrelated Binary proof
drift:

```text
BIN0-I0 raw implementation: expected=1 actual=0
```

Do not repair Binary or claim the full parent guard green.

## Shared replacement guard

Do not create a new guard.

Extend:

```text
tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Require:

```text
manifest closed Assignment row                       = 1

statement_surface:
  drive_variable_assignment_v1 call-shaped           = 1
  RawLegacyVariableAssignmentInputV1::new             = 1

raw_expression_dispatch/mod:
  Grouped drive_variable_assignment_v1                = 1
  Grouped RawLegacyVariableAssignmentInputV1::new     = 1

located_legacy_assignment:
  drive_variable_assignment_v1                        = 1

external generic driver sites                         = 3
drive_raw_variable_assignment_v1 call-shaped sites    = 0
build_grouped_assignment call-shaped sites            = 0
retry / fallback in Assignment owner                  = 0
all touched source/check files                        < 800
```

Count exact files and call shapes. The two raw/default callers and detached
located caller are separate facts.

## README and SSOT correction

Update `src/mir/builder/stmts/README.md` to state:

```text
exact Variable-target statement surface
  -> RawLegacyVariableAssignmentInputV1
  -> drive_variable_assignment_v1

GroupedAssignmentExpr raw expression surface
  -> the same input
  -> the same owner

field / index / compound
  -> separate existing owners

old raw and Grouped facades
  -> retired

located adapter
  -> same generic owner
  -> detached, production root ingress zero

historical exact-Variable parity reference
  -> cfg(test)-only
```

Remove stale authority claims for deleted `exprs.rs`, the raw facade, and the
Grouped legacy facade.

## Manifest row

Selection state:

```tsv
cell	VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0	DESCENT-SPINE0	raw_expression_dispatch:Variable-target+GroupedAssignmentExpr	stmts/variable_assignment_descent.rs:drive_variable_assignment_v1	variable_assignment_descent::drive_raw_variable_assignment_v1+builder_build::build_grouped_assignment	cargo-test:variable-assignment	-	active
```

Closeout changes only the final field to `closed`.

## LOC budget

Bounded production deletion before implementation:

```text
drive_raw_variable_assignment_v1 = -9
build_grouped_assignment         = -14
gross production Rust deletion   = -23

current four-cell cumulative      = -57
bounded five-cell cumulative      <= -80
```

Test/helper/import formatting may change the measured `src/**/*.rs` delta.
Record the exact value from the final diff; do not claim an exact closeout
number before measurement.

The sixth cell will drop the first cell's `-202` from its rolling window.
This fifth-cell result must not be described as broadly restoring future
growth budget.

## Acceptance

```text
exact Variable-target raw/default caller              = 1
GroupedAssignmentExpr raw/default caller              = 1
raw/default callers total                             = 2

detached located caller                               = 1
detached located production root ingress              = 0
external generic driver sites                         = 3
detached asset delta                                  = 0

drive_raw_variable_assignment_v1 definition           = 0
drive_raw_variable_assignment_v1 call-shaped sites    = 0
build_grouped_assignment definition                   = 0
build_grouped_assignment call-shaped sites            = 0

field assignment owner                                = unchanged
index assignment owner                                = unchanged
compound assignment owner                             = unchanged

declared preflight before RHS                         = preserved
RHS evaluation                                        = exactly once
completion declaration recheck                        = preserved
previous binding on RHS failure                       = preserved
exact local contract reassignment                     = preserved
typed-array reassignment                              = preserved
ReleaseStrong timing                                  = preserved
same-Builder reuse                                    = green

exact Variable snapshot parity                        = green
Grouped production-ingress behavior                   = green
Grouped historical snapshot parity                    = not claimed

accepted source shape delta                           = 0
semantic interface delta                              = 0
fallback / retry / route reselection                  = 0

production Rust LOC delta                             <= -23
five-cell cumulative production Rust LOC              <= -80
new per-cell guard                                    = 0
all modified/new source/check files                   < 800
```

## Gate order

```bash
cargo check -q
cargo test -q variable_assignment --lib
cargo test -q located_legacy_assignment --lib
cargo test -q grouped_assignment --lib

PYTHONPATH=tools/checks/lib python3 - <<'PY'
from pathlib import Path
from callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment import check_asn0_s0

print(check_asn0_s0(Path(".")))
PY

bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The clean-main baseline for `cargo test -q variable_assignment --lib` is
18/18 green. A zero-test filter is not positive evidence; resolve actual test
module names before closeout.

## Atomic implementation commit

The next commit must contain:

```text
old facade deletion
facade-only test deletion and unused helper/import cleanup
Grouped production-ingress fixture rename
private ASN0 helper correction
shared guard extension
Assignment README correction
manifest active -> closed
task card accepted -> closed
CURRENT_STATE closeout
workstream counter 4 -> 5
task-map fifth closeout
measured LOC closeout
```

Do not insert another cell, consultation, proof-only change, or cleanup between
this selection commit and the implementation commit.

Recommended implementation commit:

```text
refactor(mir): retire legacy variable assignment facades
```

## Hard stop

Stop and return to a short D0 if:

```text
either old facade has a non-test caller
raw/default caller set is not exactly Variable + Grouped
external generic driver sites are not exactly those two plus located
located production root ingress becomes non-zero
the two raw/default ingresses require different semantic interfaces
build_assignment_from_value must change
field / index / compound Assignment must change
test migration needs a compatibility facade
private helper cannot become green through Assignment-only assertions
Grouped needs a new historical parity authority
production Rust LOC delta is greater than -23
five-cell cumulative becomes positive
fallback / retry / route probing is required
```

## Explicit non-claims

```text
no Return interface split
no statement If replacement
no Binary / ShortCircuit guard repair
no non-Program root cutover

no Local reopening
no Stage-B special activation
no Ownership work
no language / runtime / backend change
no selfhost migration

no new AST acceptance
no new Assignment recipe
no new source / identity / failure authority
no second production route
no Grouped historical snapshot parity claim
```
