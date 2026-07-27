---
Status: closed
Date: 2026-07-28
Decision: LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0
Pack: DESCENT-SPINE0
Ceremony: T0
Commits: one atomic I0/R0 commit
Parent:
  - docs/development/current/main/investigations/mirbuilder-next-edge-design-stop-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
---

# LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0

## Decision

Credit the already-live raw/default Local statement descent replacement,
delete its two remaining Legacy facades, migrate test setup callers to the
real AST ingress, and transfer the selected production-edge proof to the
shared in-place replacement guard.

This is not a second Local implementation and does not redo the historical
raw dispatcher cutover.

## Exact replacement cell

```text
cell_id:
  LOCAL-STATEMENT-DESCENT-CUTOVER0

responsibility:
  Local declaration associated-input descent

selected_live_production_caller:
  src/mir/builder/raw_expression_dispatch/statement_surface.rs
  ASTNode::Local branch

new_owner:
  src/mir/builder/stmts/local_statement_descent.rs
  drive_local_statement_v1

selected_old_symbols:
  src/mir/builder/stmts/variable_stmt.rs
    build_local_statement

  src/mir/builder/stmts/local_statement_descent.rs
    drive_raw_local_statement_v1

preserved_detached_caller:
  src/mir/builder/located_legacy_lowering.rs
  drive_local_statement_v1
  root activation = 0

fallback / retry / reselection:
  forbidden
```

## Bounded census

The 2026-07-28 four-worker read-only audit found:

```text
raw/default ASTNode::Local selector                    = 1
raw/default drive_local_statement_v1 caller            = 1
raw/default RawLegacyLocalInputV1 constructor           = 1

detached located drive_local_statement_v1 caller        = 1
detached located production root ingress                = 0
post-cutover non-test driver call-shaped sites           = 2

build_local_statement non-test consumers                = 0
build_local_statement cfg(test) call sites               = 9
drive_raw_local_statement_v1 independent consumers       = 0
```

The located adapter is production-shaped source but has no live runtime
ingress. Do not collapse raw/default and located counts into a false global
`driver callers = 1` claim.

Naive word counting is also forbidden. Existing stable debug tags contain
`build_local_statement`; guards must count function definitions and
call-shaped symbols, not arbitrary text.

## Structural boundary

`drive_local_statement_v1` already owns the required order:

```text
borrow syntax once
-> whole-declaration preflight
-> observe preflight success
-> evaluate initializers in declaration order
-> typed-array / record specialized owner
-> publish bindings only after all values succeed
```

Do not modify this semantic interface in the cell.

Keep:

```text
RawLegacyLocalInputV1
LocalStatementSyntaxViewV1
LocalStatementDescentPortV1
blanket RawAstChildLoweringPortV1 implementation
drive_local_statement_v1
observe_preflighted_local_statement
preflight_exact_numeric_local_initializers
build_local_statement_from_values*
```

Delete only the selected entry facades and their stale authority
documentation.

## Atomic implementation

### 1. Reconfirm census before editing

```bash
rg -n -w 'build_local_statement' src --glob '*.rs'
rg -n -w 'drive_raw_local_statement_v1' src --glob '*.rs'
rg -n -w 'drive_local_statement_v1' src --glob '*.rs'
```

Hard stop if a new non-test consumer of either old facade appears.

### 2. Delete the old Local entry facade

Remove from `src/mir/builder/stmts/variable_stmt.rs`:

```text
build_local_statement documentation block
build_local_statement function
```

Do not rename the existing debug tags in the same cell. Debug tag policy is a
separate owner.

Update stale owner lists in:

```text
src/mir/builder/stmts/variable_stmt.rs
src/mir/builder/stmts/mod.rs
```

### 3. Delete the raw compatibility facade

Remove from `src/mir/builder/stmts/local_statement_descent.rs`:

```text
drive_raw_local_statement_v1
now-unused RawLegacyChildLoweringPortV1 import
```

Do not remove `RawLegacyLocalInputV1`; the live statement surface owns it.

### 4. Rewrite nine test setup calls

The old facade has nine cfg(test) call sites:

```text
stmts/variable_stmt.rs local_contract_tests                 3
control_flow/plan/parts/wiring_tests.rs                     2
control_flow/plan/parts/associated_source/raw_parity_tests.rs 2
control_flow/plan/parts/if_general.rs tests                 1
control_flow/plan/parts/stmt/tests.rs                       1
```

Rewrite each through:

```rust
builder.build_expression(ASTNode::Local { ... })
```

Remove the four explicit `build_local_statement` imports. Do not introduce a
test-only compatibility facade or second raw selector.

Existing owner-level tests already call `drive_local_statement_v1` directly;
do not rewrite them.

### 5. Preserve existing production-ingress fixtures

`local_statement_raw_tests.rs` already contains seven
`builder.build_expression(ASTNode::Local)` fixtures. The exact ingress witness
is:

```text
raw_local_selector_preserves_initializer_order_and_binding_completion
```

Do not add a redundant fixture unless the implementation changes a previously
uncovered failure boundary.

### 6. Repair the old EXPR0 Local proof without widening scope

Update:

```text
tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0.py
```

Retire assertions requiring:

```text
drive_raw_local_statement_v1 definition = 1
variable_stmt facade selector = 1
raw facade caller = 1
summary lcl_raw_selector = 1
```

Repair the already-stale raw implementation assertion. The current owner is
the blanket implementation:

```text
impl<Port> LocalStatementDescentPortV1 for Port
where Port: RawAstChildLoweringPortV1
```

Replace the already-stale global `driver callers = 1` assertion with exact
separate evidence:

```text
statement_surface raw/default caller = 1
located adapter caller               = 1
non-test driver call-shaped sites    = 2
```

Preserve all existing assertions for:

```text
syntax / preflight / initializer / completion order
typed-array and record specialized ownership
binding publication timing
failure and same-Builder reuse
parity reference
located exact-role and inactive-subtree proof
retry / fallback / AST-reconstruction absence
800-line and stack-scoped boundaries
```

The public EXPR0 entry is currently red before reaching Local:

```text
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0.py
-> BIN0-I0 raw implementation: expected=1 actual=0
```

This is pre-existing unrelated Binary guard drift. Do not repair Binary,
ShortCircuit, Assignment, Return, If, or Loop in this cell, and do not claim
the whole public EXPR0 guard green.

Run the existing Local helper directly as focused diagnostic evidence after
repair:

```bash
PYTHONPATH=tools/checks/lib python3 -c \
  'from pathlib import Path; from callable_result_i0_site0_r0_expr0_spine0_stmt0 import check_lcl0_s0; root=Path("."); print(check_lcl0_s0(root, (root/"src/mir/builder/located_legacy_lowering.rs").read_text()))'
```

Do not create a new guard or public wrapper for this cell.

### 7. Transfer production-edge authority to the shared guard

Extend:

```text
tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Require:

```text
manifest closed Local row                            = 1
statement_surface drive_local_statement_v1           = 1
statement_surface RawLegacyLocalInputV1::new          = 1
located_legacy_lowering drive_local_statement_v1      = 1
call-shaped build_local_statement definitions/calls   = 0
call-shaped drive_raw_local_statement_v1 definitions/calls = 0
Local owner retry / fallback                          = 0
all touched source/check files                        < 800
```

Count exact files and call shapes. Do not use repository-wide word counts that
include debug strings or owner-level tests.

### 8. Update statement boundary documentation

Update `src/mir/builder/stmts/README.md`:

```text
raw/default statement_surface directly selects the generic Local owner
old variable_stmt facade is retired
located adapter remains detached with root activation zero
```

Do not rewrite the surrounding Assignment/Return/If/Loop history.

## Manifest closeout row

```tsv
cell	LOCAL-STATEMENT-DESCENT-CUTOVER0	DESCENT-SPINE0	raw_expression_dispatch/statement_surface.rs:ASTNode::Local	stmts/local_statement_descent.rs:drive_local_statement_v1	variable_stmt::build_local_statement+local_statement_descent::drive_raw_local_statement_v1	cargo-test:local-statement	-	closed
```

## Acceptance

```text
raw/default production Local selector                 = 1
raw/default selected generic owner caller              = 1
detached located owner caller                          = 1
detached located production root ingress               = 0
post-cutover non-test generic driver sites             = 2

build_local_statement function definition              = 0
build_local_statement call-shaped sites                = 0
drive_raw_local_statement_v1 function definition       = 0
drive_raw_local_statement_v1 call-shaped sites         = 0

fallback / retry / route reselection                    = 0
whole preflight before child effects                    = preserved
initializer order                                      = preserved
later initializer after child failure                  = 0
binding publication before all values ready            = 0
typed-array specialized owner                          = preserved
record-constructor specialized owner                   = preserved
untyped missing initializer Null sugar                 = preserved
same-Builder reuse                                     = green

detached_asset_delta                                   = 0
production Rust LOC delta                              < 0
four-cell cumulative production Rust LOC               < 0
five-cell rolling budget                               <= 0
new per-cell guard                                     = 0
all modified/new source/check files                    < 800
```

## Gates

```bash
cargo check -q
cargo test -q local_statement --lib
cargo test -q mir::builder::located_legacy_local_tests --lib
cargo test -q mir::builder::stmts::variable_stmt::local_contract_tests --lib
cargo test -q mir::builder::control_flow::plan::parts::wiring_tests --lib
cargo test -q mir::builder::control_flow::plan::parts::associated_source::raw_parity_tests --lib
cargo test -q mir::builder::control_flow::plan::parts::if_general::tests --lib
cargo test -q mir::builder::control_flow::plan::parts::stmt::tests --lib

# focused existing Local semantic helper; public parent has unrelated baseline red
PYTHONPATH=tools/checks/lib python3 -c \
  'from pathlib import Path; from callable_result_i0_site0_r0_expr0_spine0_stmt0 import check_lcl0_s0; root=Path("."); print(check_lcl0_s0(root, (root/"src/mir/builder/located_legacy_lowering.rs").read_text()))'

bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

In the same atomic commit:

```text
manifest Local row                    active -> closed
live_replacement_cells_closed         3 -> 4
production Rust LOC                   record measured negative value
four-cell cumulative                  record measured value
current blocker                       return to MIRBUILDER-NEXT-EDGE-DESIGN-STOP
```

Update:

```text
CURRENT_STATE.toml
mirbuilder-inplace-replacement0-task-map-2026-07-28.md
mirbuilder-inplace-replacement-current.md
mirbuilder-next-edge-design-stop-2026-07-28.md
```

No fifth production row is selected by this card.

## Hard stop

Return to design consultation if:

```text
new non-test build_local_statement consumer appears
new non-test drive_raw_local_statement_v1 consumer appears
raw/default ASTNode::Local selector count is not exactly one
located root activation becomes non-zero
semantic Local driver interface must change
test migration requires a compatibility facade
Local deletion requires Assignment/Return/If/Stage-B changes
focused Local helper remains red after only Local assertions are updated
```

Never add fallback, retry, or a second selector to escape a stop.

## Non-claims

```text
DESCENT-SPINE0 pack close
located production activation
Assignment / Return / If / Binary / ShortCircuit cutover
Function-state writer replacement
non-Program root fallback repair
Stage-B special activation
Ownership / language / runtime / backend change
selfhost migration
```

Recommended commit message:

```text
refactor(mir): retire legacy local descent facades
```

## Landed closeout

Closed on 2026-07-28.

```text
raw/default production Local selector       = 1
raw/default generic owner caller            = 1
detached located owner caller               = 1
detached located production root ingress    = 0
post-cutover non-test generic driver sites  = 2

build_local_statement call-shaped sites     = 0
drive_raw_local_statement_v1 call sites     = 0
fallback / retry                            = 0
cfg(test) production-ingress migrations     = 9

focused Local tests                         = green
focused Local semantic helper               = green
production Rust LOC delta                   = -52
four-cell cumulative production Rust LOC    = -57
detached asset delta                        = 0
```

The unrelated Binary assertion still prevents the public EXPR0 parent from
being a whole-row gate. This cell repaired only the Local helper and made the
shared in-place replacement guard the stable production-edge authority.
