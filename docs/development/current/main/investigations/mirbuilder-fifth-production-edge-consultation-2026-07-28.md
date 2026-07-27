---
Status: accepted and taskized
Date: 2026-07-28
Decision: VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0-I0-R0
Parent:
  - docs/development/current/main/investigations/
    mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/
    mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/
    mirbuilder-inplace-replacement-current.md
---

# MirBuilder fifth production edge consultation

## Consultation request

Please perform a read-only bounded census on the latest checkout and select
exactly one fifth in-place MirBuilder production replacement cell.

Return one implementation-ready task card. Do not edit source, tests, guards,
the manifest, or current pointers during this consultation.

The selected cell must replace one existing production edge and retire that
selected old edge in the same bounded series. A disconnected substrate,
inventory-only row, compatibility wrapper, fallback, retry, or second
production route is not an acceptable answer.

## Current closed evidence

Four production cells are closed:

```text
1. CALLABLE-DRAFT-PORT-CUTOVER0
   production Rust LOC = -202

2. CALLABLE-DRAFT-COLLECTOR-CUTOVER0
   production Rust LOC = +153

3. MODULE-CANDIDATE-SESSION-CUTOVER0
   production Rust LOC = +44

4. LOCAL-STATEMENT-DESCENT-CUTOVER0
   production Rust LOC = -52

four-cell cumulative production Rust LOC = -57
```

The fourth cell established:

```text
raw/default ASTNode::Local selector              = 1
raw/default drive_local_statement_v1 caller      = 1
detached located driver caller                   = 1
detached located production root ingress         = 0
build_local_statement call-shaped sites          = 0
drive_raw_local_statement_v1 call-shaped sites   = 0
fallback / retry                                 = 0
```

Do not reopen or redo Local.

## Source authority

Use these current files as authority:

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/design/
  mirbuilder-inplace-replacement-policy-ssot.md
docs/development/current/main/design/fixtures/
  mirbuilder-inplace-replacement-v1.tsv
docs/development/current/main/workstreams/
  mirbuilder-inplace-replacement-current.md
docs/development/current/main/investigations/
  mirbuilder-inplace-replacement0-task-map-2026-07-28.md
docs/development/current/main/investigations/
  local-statement-descent-cutover0-i0-r0-task-2026-07-28.md
src/mir/builder/README.md
src/mir/builder/stmts/README.md
src/mir/builder/control_flow/plan/REGISTRY.md
tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Inspect source and existing tests directly. Historical phase names, old task
cards, comments claiming a cutover, and disconnected fixtures are not proof
of a live production edge.

## Production definition

For this consultation, a production caller is a non-test path reachable from
the current default Legacy compile request:

```text
MirCompiler::compile_with_source*
-> current candidate module session
-> current MirBuilder module/expression ingress
```

If proposing a different caller family, name it explicitly and prove that it
is already a supported production family. Do not count:

```text
#[cfg(test)] callers
parity references
detached located adapters
candidate-only proof sessions
feature-only reference lanes
prepared owners with production consumer zero
```

## Required bounded census

Start with exact symbol and caller searches for the remaining historical live
replacement families:

```text
Variable-target Assignment
value-bearing Return
statement-position If
Binary
ShortCircuit
```

For each family, identify:

```text
current production selector
current selected owner
old facade / branch / symbol still present
all non-test consumers of that old edge
cfg(test) consumers
detached callers
existing focused parity fixtures
existing guard assertions
gross and net Rust LOC repayment
```

Also inspect the known non-Program root edge:

```rust
other => self.build_expression(other)
```

Treat it only as a candidate. It may be selected only if the census proves one
bounded production caller family and one safe replacement owner. Compiler
direct `build_module` callers and non-Program `BoxDeclaration` / `Main`
meaning must not be inferred from the branch spelling.

Use call-shaped searches rather than naive word counts. Debug strings and
historical comments are not callers.

## Candidate ranking law

Prefer the first candidate satisfying all of:

```text
new owner already exists
existing production caller >= 1
selected old edge can become 0
fallback / retry / route reselection = 0
semantic interface change = 0
new detached asset = 0
focused production-ingress fixture already exists or is minimal
shared guard can absorb the proof
production Rust LOC delta < 0
five-cell rolling production Rust LOC <= 0
all touched source/check files < 800 lines
```

Tie-break in this order:

```text
1. smallest exact old-edge consumer set
2. strongest existing production-ingress parity
3. largest structural deletion
4. least guard drift outside the selected family
5. earliest DESCENT-SPINE0 responsibility
```

Do not select a candidate merely because its new owner has a suggestive name.

## BoxCount / BoxShape decision

Classify the selected work explicitly.

Expected classification is:

```text
BoxShape:
  credit an already-live replacement
  remove obsolete facade/branch authority
  migrate tests to the real ingress
  transfer proof to the shared guard
```

Hard stop instead of selecting a cell if every candidate requires:

```text
a new accepted source shape
a new lowering recipe
a semantic interface change
a new source/identity/failure authority
a compatibility wrapper
a second route
cross-family changes
```

## Required answer

Return one Markdown execution task containing:

```text
Decision and ceremony T0/T1/T2
exact cell_id and pack
responsibility
production caller before and after
new owner
old edge/symbol/branch to delete
exact production/cfg(test)/detached caller census
test migration plan
focused parity gate
shared guard changes
README / SSOT changes
manifest row
measured or bounded production Rust LOC
five-cell rolling LOC result
atomic commit boundary
acceptance matrix
gate order
Hard stop conditions
explicit non-claims
recommended commit message
```

If no candidate satisfies T0, do not manufacture one. Return a design-stop
result naming the shortest missing evidence and whether the next legal work is
`REPLACEMENT-LEDGER0` census, a T1 responsibility-boundary decision, or a
separate policy consultation.

## Guard caveat

The private Local helper is green and now proves the fourth cell. The public
EXPR0 parent still has unrelated pre-existing Binary guard drift before it
reaches Local:

```text
BIN0-I0 raw implementation: expected=1 actual=0
```

Do not repair Binary incidentally. If Binary is selected as the fifth cell,
the answer must distinguish:

```text
stale proof repair
actual production edge replacement
old edge deletion
```

A guard-only repair is not a production replacement cell.

## Global prohibitions

```text
no source edit during consultation
no fifth manifest row before selection
no new macro pack
no per-cell shell guard
no fallback / retry / route probing
no AST rewrite
no Stage-B special activation
no Ownership, language, runtime, backend, or selfhost work
no Local reopening
no file at or above 800 lines
```

## One-line question

```text
Which exact existing production edge should become the fifth in-place
MirBuilder replacement cell, and can its selected old edge be physically
retired in one bounded negative-LOC commit without semantic widening?
```

## Resolution

Accepted after a four-worker latest-main audit:

```text
VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0-I0-R0
```

Execution authority:

```text
docs/development/current/main/investigations/
variable-assignment-descent-cutover0-i0-r0-task-2026-07-28.md
```

The audit fixed the caller contract at two raw/default sites—exact Variable
target and GroupedAssignmentExpr—plus one detached located site. Both
raw/default sites already construct the same input and call the same owner.
The task card explicitly denies Grouped historical snapshot parity and stops
if the two ingresses require different semantic interfaces.
