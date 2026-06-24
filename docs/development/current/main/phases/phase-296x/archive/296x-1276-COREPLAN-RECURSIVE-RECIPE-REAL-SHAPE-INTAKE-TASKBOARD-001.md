---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Taskize the next compiler Recipe/CorePlan real-shape intake after json_native app-front hardening.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1274-COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-002.md
  - docs/development/current/main/phases/phase-296x/296x-1275-JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001.md
  - apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
  - apps/rust-subset-to-hako/STATUS.md
---

# COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001

## Decision

Proceed with compiler-side real-shape intake, not app-front syntax expansion.

The immediate candidate is the `read_next_number_literal()` family:

```text
multi-stage scanner loop
conditional break
loop-carried cursor / accumulator
optional sign / decimal / exponent stages
post-loop validity branch
possible EOF/error/value multi-exit
```

The existing minimal staged-loop canary is already planner-required green via
`LoopSimpleWhile` plus flowbox break adoption. The next work must therefore
capture a stronger fixture before opening any Recipe/CorePlan implementation.

## Clarification

The previously suggested `while` and `Vec literal` app-front tasks are already
closed:

```text
RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-003:
  Rust while -> RustSubset While -> .hako loop(cond)

RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-004:
  Rust vec![...] -> RustSubset ArrayLiteral -> .hako array literal
```

Those are RustSubset transport tasks. They are separate from compiler
Recipe/CorePlan acceptance of loop exits.

## Task Ladder

### 1. `COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001`

Capture a stronger minimal fixture based on `JsonParser.parse_integer_span`:

```text
optional leading sign
loop over span
digit_value_at(...)
break on non-digit
loop-carried accumulator
post-loop negative branch
```

Acceptance:

```text
fixture_added=1
planner_required_gate_run=1
first_owner_recorded=1
implementation_allowed_only_if_gate_fails=1
```

If green, close without compiler code changes and move to the next shape.

### 2. `COREPLAN-REAL-SHAPE-FIXTURE-DECIMAL-EXPONENT-001`

Capture a reduced `JsonScanner.read_number()` fixture with staged decimal and
exponent branches:

```text
integer stage
optional dot stage
optional exponent stage
optional plus/minus after exponent
break or return-null style validation
```

Acceptance:

```text
method_name_branch=0
json_native_route_changed=0
minimal_failing_fixture_required=1
```

### 3. `COREPLAN-REAL-SHAPE-FIXTURE-MULTI-EXIT-001`

Capture EOF/error/value multi-exit scanner control flow:

```text
value exit
error/null exit
EOF exit
shared loop-carried scanner state
```

Acceptance:

```text
multi_exit_recipe_gap_recorded=1
single_acceptance_shape_per_row=1
```

### 4. `COREPLAN-CONTINUE-IN-STAGED-LOOP-001`

Only after a break-bearing stronger fixture is classified, capture a minimal
`continue` fixture:

```text
loop(cond)
staged local classification
continue before tail step or tail effect
```

This remains parked until the break/multi-exit ladder identifies the next
actual compiler owner.

### 5. `COREPLAN-NESTED-BREAK-CONTINUE-001`

Capture nested `if`/loop exit interactions only after the non-nested owner is
clear.

## Current Unsupported Shape Queue

```text
active_next:
  read_number_sign_break_fullish

queued_after_active:
  read_number_decimal_exponent
  scanner_multi_exit
  continue_inside_staged_loop
  nested_break_continue
  loop_carried_phi_scanner_shape
  return_break_continue_interaction

rust_subset_transport_already_closed:
  while
  vec_literal
  vec_method_calls
  loop_without_break
  else_if
  returnless_void_body
  explicit_unit_return

rust_subset_semantics_still_unsupported_handoff:
  match_semantics
  for_loop_semantics
  trait_generic_item_support
```

## Stop Lines

```text
do not implement read_next_number_literal by name
do not treat token payload recovery as compiler Recipe evidence
do not broaden recursive Recipe without a minimal failing fixture
do not add while/Vec app-front work here; those transport rows are closed
do not mix continue acceptance into the first break-bearing fixture row
do not change json_native app code to avoid compiler acceptance
```

## Report

```text
output_contract=coreplan-recursive-recipe-real-shape-intake-taskboard-v0
active_next=COREPLAN-REAL-SHAPE-FIXTURE-SIGN-BREAK-001
read_next_number_literal_taskized_as_shape_ladder=1
existing_staged_loop_canary_green=1
while_app_front_transport_closed=1
vec_literal_app_front_transport_closed=1
implementation_allowed=0
minimal_failing_fixture_required=1
summary=ok
```
