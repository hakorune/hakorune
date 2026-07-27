---
Status: closed
Date: 2026-07-28
Decision: RETURN-SOURCE-PARTITION-CUTOVER0-I0-R0
HistoricalCredit:
  - value-bearing Return
Pack: DESCENT-SPINE0
Ceremony: T1
BoxShape: eliminate-optional-return-facade-and-retire-raw-compatibility
Commits:
  - one SSOT selection / short-design-note commit
  - one immediately-following atomic implementation commit
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# RETURN-SOURCE-PARTITION-CUTOVER0-I0-R0

## Decision

Credit the already-live raw/default value-bearing Return descent owner,
replace the residual optional Return facade with one exact Void-return leaf,
delete the obsolete raw compatibility facade, remove facade-only tests,
correct stale Return proof authority, and transfer the production-edge proof
to the shared in-place replacement guard.

This is a T1 responsibility-interface split. It introduces no new language
semantics, source authority, Return ABI, Match policy, defer policy, failure
owner, or production route.

## Exact replacement cell

```text
cell_id:
  RETURN-SOURCE-PARTITION-CUTOVER0

responsibility:
  exact Return source-shape partition

sole raw/default production selector:
  src/mir/builder/raw_expression_dispatch/statement_surface.rs
  ASTNode::Return branch

value-bearing owner:
  src/mir/builder/stmts/return_statement_descent.rs
  drive_value_return_statement_v1

void-return owner:
  src/mir/builder/stmts/return_stmt.rs
  build_void_return_statement

shared completion owner:
  emit_return_from_value

old symbols to delete:
  return_stmt::build_return_statement
  return_statement_descent::drive_raw_value_return_statement_v1

preserved detached caller:
  located_legacy_return.rs
  drive_value_return_statement_v1
  production root ingress = 0

source-route fallback / retry / reselection:
  forbidden
```

## Latest-main census

The source partition is already live:

```text
ASTNode::Return
  -> build_return_with_port_v1

Some(value)
  -> RawLegacyValueReturnInputV1::new
  -> drive_value_return_statement_v1

None
  -> build_return_statement(builder, None)
```

The mixed facade is not the production value-bearing route. Its `Some` arm is
only obsolete compatibility authority.

```text
drive_value_return_statement_v1:
  definition                         = 1
  raw/default production caller      = 1
  detached located caller            = 1
  raw-facade internal caller          = 1
  external non-test sites after cut  = 2

drive_raw_value_return_statement_v1:
  definition                         = 1
  live raw/default caller            = 0
  compatibility caller               = 1
  cfg(test) direct callers           = 2

build_return_statement:
  definition                         = 1
  live non-test caller               = 1, None only
  cfg(test) caller                   = 1, None only

located production root ingress      = 0
source fallback / retry / probing     = 0
```

Hard stop if a fresh census finds another non-test caller or more than one
raw/default `ASTNode::Return` selector.

## Responsibility split

### Exact Void owner

Add one narrow leaf:

```rust
pub(in crate::mir::builder) fn build_void_return_statement(
    builder: &mut MirBuilder,
) -> Result<ValueId, String> {
    ensure_return_allowed(builder)?;
    let value = crate::mir::builder::emission::constant::emit_void(builder)?;
    emit_return_from_value(builder, value)
}
```

It may own only:

```text
ensure_return_allowed
-> emit_void
-> emit_return_from_value
```

It must not accept an `ASTNode`, `Option`, child-lowering port, source receipt,
or Match input.

### Production partition

Keep `Some(value)` unchanged and replace only the `None` call:

```text
Some(value)
  -> RawLegacyValueReturnInputV1
  -> drive_value_return_statement_v1

None
  -> build_void_return_statement
```

The selector remains exactly one.

### Retire obsolete authority

Delete:

```text
build_return_statement
its Option<Box<ASTNode>> boundary
its dormant Some compatibility branch
drive_raw_value_return_statement_v1
the raw facade's now-unused import
```

Keep:

```text
ensure_return_allowed
try_apply_match_return_optimization
adopt_match_return_coreplan
emit_return_from_value
RawLegacyValueReturnInputV1
ReturnStatementSyntaxViewV1
ReturnStatementDescentPortV1
drive_value_return_statement_v1
```

## Match(None) proof

The old Void branch calls:

```text
try_apply_match_return_optimization(builder, None, true)
```

On current main this is effect-free on every branch. With active defer it
returns `Ok(None)` immediately; otherwise mandatory-value destructuring returns
`Ok(None)` before facts extraction, CorePlan selection, emission, diagnostics,
or state mutation.

The exact Void owner therefore removes this phantom observation. The private
Return helper and focused ingress tests must prove:

```text
Void cleanup rejection occurs before Void emission
Void emits no expression child
Void emits exactly the existing ConstValue::Void representation
Void reuses the existing completion owner
Match authority remains value-bearing only
```

## Completion-policy boundary

Do not modify `emit_return_from_value`.

Its configured-defer behavior remains:

```text
Copy to defer slot
-> metadata propagation
-> Jump when unterminated
-> return_deferred_emitted
```

Its existing direct-Return behavior when defer configuration is incomplete is
an existing completion policy, even where source comments use the word
“fallback”. It is not a source-route fallback. This cell neither repairs nor
guards it with a broad textual `fallback = 0` assertion.

## Test migration

Delete the two facade-only tests:

```text
raw_value_return_reuses_binary_and_short_circuit_child_spines
raw_value_return_reuses_actual_method_call_child_spine
```

Do not replace them with a compatibility helper. Existing raw production
ingress and parity suites already cover Binary, ShortCircuit, and MethodCall
value-bearing Returns.

Narrow:

```text
value_return_input_excludes_void_while_legacy_void_return_remains
-> value_return_input_excludes_void
```

Keep its mandatory-value input assertions and delete its direct legacy Void
facade half.

Rename the real ingress fixture:

```text
raw_void_return_stays_on_legacy_facade
-> raw_void_return_selects_void_source_partition
```

Its body continues to enter through:

```text
builder.build_expression(ASTNode::Return { value: None })
```

No new fixture is needed. Preserve the existing raw, parity, and located
coverage, including cleanup failure, child failure, defer, Match, and
same-Builder reuse.

## Private Return helper

Update:

```text
tools/checks/lib/
callable_result_i0_site0_r0_expr0_spine0_stmt0_return.py
```

Remove stale authority for:

```text
src/mir/builder/exprs.rs
build_return_statement Some selector
drive_raw_value_return_statement_v1
generic value driver production caller = 0
legacy Void facade
Match(None) observation
```

Add exact assertions:

```text
statement_surface:
  value driver caller                     = 1
  RawLegacyValueReturnInputV1::new         = 1
  build_void_return_statement caller       = 1

located_legacy_return:
  value driver caller                     = 1

external non-test value-driver sites       = 2
drive_raw_value_return_statement_v1 sites = 0
build_return_statement sites              = 0
try_apply_match_return_optimization(None)  = 0
```

Preserve:

```text
value cleanup -> syntax -> Match -> input -> child -> completion
mandatory value input
Void cleanup -> emit_void -> completion
selected Match bypass
configured defer completion
child/cleanup failure ordering
snapshot parity and Builder reuse
located ReturnValue role and root-inactive proof
no source reconstruction, retry, or route fallback
```

The public EXPR0 parent has unrelated Binary proof drift. Do not repair Binary
or claim that parent green. The focused Return helper is the cell gate.

## Shared replacement guard

Extend the existing:

```text
tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Do not add a per-cell guard.

Guard:

```text
manifest closed Return row                       = 1

statement_surface:
  drive_value_return_statement_v1                = 1
  RawLegacyValueReturnInputV1::new                = 1
  build_void_return_statement                     = 1

located_legacy_return:
  drive_value_return_statement_v1                = 1

external non-test value driver sites             = 2
drive_raw_value_return_statement_v1 sites        = 0
build_return_statement sites                     = 0
try_apply_match_return_optimization(None) sites  = 0
all touched source/check files                   < 800
```

## README and SSOT

Update `src/mir/builder/stmts/README.md` and the module entry documentation to
show:

```text
Return Some -> statement surface -> mandatory raw input -> value driver
Return None -> statement surface -> exact Void owner
old mixed Option facade retired
old raw value facade retired
located value adapter remains root-inactive
historical parity remains cfg(test)-only
```

Selection commit updates only:

```text
CURRENT_STATE.toml
this execution task
mirbuilder-inplace-replacement-current.md
mirbuilder-inplace-replacement0-task-map-2026-07-28.md
mirbuilder-next-edge-design-stop-2026-07-28.md
mirbuilder-inplace-replacement-v1.tsv
```

The immediately following implementation commit closes the same row and
updates code, tests, README, private Return helper, and shared guard.

## LOC budget

The rolling budget is binding:

```text
cells 2..5:
  +153 +44 -52 -77 = +68

required sixth-cell src/**/*.rs delta:
  <= -68
```

The bounded Return-only estimate is approximately `-101` before unnecessary
additions:

```text
raw facade                         -8
two facade-only tests             -52
sole-use test helpers/imports      -35
mixed facade narrowing             -6 or better
```

This is an estimate, not closeout authority. Measure the final
`src/**/*.rs` diff. Hard stop if `<= -68` cannot be achieved without unrelated
cleanup. The target is `<= -90`; the acceptance requirement is `<= -68`.

## Acceptance

```text
raw/default value-bearing Return caller           = 1
raw/default exact Void Return caller               = 1
detached located value caller                      = 1
detached located production root ingress           = 0

drive_raw_value_return_statement_v1 definition     = 0
drive_raw_value_return_statement_v1 call sites     = 0
build_return_statement definition                  = 0
build_return_statement call sites                  = 0
try_apply_match_return_optimization(None)           = 0
Option syntax crossing Return owner boundary       = 0

value cleanup / Match / child / completion order   = preserved
value child evaluation count                       = exactly 1
Void cleanup before Void emission                  = preserved
Void expression child demand                       = 0
Return/defer completion owner                      = unchanged
Return ABI                                         = unchanged

raw, parity, located, failure, reuse suites         = green
focused private Return helper                      = green
shared replacement guard                           = green

source-route fallback / retry / reselection        = 0
detached asset delta                               = 0
new per-cell guard                                 = 0
src/**/*.rs LOC delta                              <= -68
new five-cell rolling LOC                          <= 0
all touched source/check files                     < 800
```

## Gate order

```bash
rg -n -P '\b(?:fn\s+)?build_return_statement\s*\(' src --glob '*.rs'
rg -n -P '\b(?:fn\s+)?drive_raw_value_return_statement_v1\s*\(' src --glob '*.rs'
rg -n -P '\bdrive_value_return_statement_v1\s*\(' src --glob '*.rs'

cargo check -q
cargo test -q return_statement --lib
cargo test -q located_return --lib

PYTHONPATH=tools/checks/lib python3 - <<'PY'
from pathlib import Path
from callable_result_i0_site0_r0_expr0_spine0_stmt0_return import check_ret0_s0

print(check_ret0_s0(Path(".")))
PY

bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Record executed test counts; a zero-test filter is not green evidence.

## Atomic boundaries

Selection:

```text
docs(mir): select explicit return source owners
```

Immediately following implementation:

```text
refactor(mir): split value and void return owners
```

Do not interleave another cell, consultation, Binary proof repair, or
proof-only commit.

## Hard stop

Stop if:

```text
an unknown non-test old-facade caller exists
the raw/default Return selector count is not one
Some/None is not a sufficient exact source partition
the Void owner needs AST, Option, Match input, or child descent
Match(None) has acquired an effect
emit_return_from_value must change
Return ABI, defer, cleanup, Match, or failure policy must change
the located adapter must change or activate
a compatibility wrapper, retry, or second route is needed
the focused Return helper cannot go green with Return-only corrections
src/**/*.rs delta exceeds -68
the five-cell rolling total becomes positive
```

## Explicit non-claims

```text
no Function exit semantic change
no Return ABI, Match/CorePlan, defer, or cleanup policy change
no located production activation
no Binary, ShortCircuit, If, Loop, or non-Program-root work
no Stage-B, Ownership, language, runtime, backend, or selfhost work
no seventh-cell selection
```

## Closeout

Closed on 2026-07-28.

```text
raw/default value-bearing Return caller           = 1
raw/default exact Void Return caller               = 1
detached located value caller                      = 1
detached located production root ingress           = 0

drive_raw_value_return_statement_v1 sites          = 0
build_return_statement sites                       = 0
try_apply_match_return_optimization(None) sites    = 0
source-route fallback / retry / reselection        = 0

focused Return tests                               = 17 / 17
focused located Return tests                       = 6 / 6
private Return helper                              = green
shared replacement guard                           = green
current-state pointer guard                        = green

src/**/*.rs additions                              = 24
src/**/*.rs deletions                              = 165
src/**/*.rs LOC delta                              = -141
new five-cell rolling LOC                          = -73
largest touched source/check file                  = 391 lines
```

The exact Void leaf owns only cleanup preflight, existing Void emission, and
the unchanged completion owner. The value-bearing path and located inactive
adapter retain the same generic driver. Return ABI, Match/CorePlan,
defer/cleanup policy, runtime, backend, and language behavior are unchanged.
