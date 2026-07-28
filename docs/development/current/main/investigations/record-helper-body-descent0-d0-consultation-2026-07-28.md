---
Status: accepted execution task
Date: 2026-07-28
Decision: RECORD-HELPER-BODY-DESCENT0-I0-R0
Pack: DESCENT-SPINE0
Ceremony: T1
Selected: Candidate A — existing invocation capability short reborrow
Commits:
  - one docs selection commit
  - one immediately-following atomic I0/R0 implementation commit
Scope: callable-catalog helper declaration body descent after one selected MethodCall route
Parent:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
NorthStar:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# RECORD-HELPER-BODY-DESCENT0-I0-R0

## Decision

Candidate A is accepted. The callable catalog already owns the helper
declaration identity, parameters, declarations, and body AST. The selected
raw/default MethodCall port already owns the recursive invocation capability
and collector-backed header view. The missing piece is only a bounded
projection through the existing argument-capability erasure boundary.

This is a T1 responsibility-interface change:

```text
new source/provenance owner    = 0
new identity issuer            = 0
new publication/failure owner  = 0
helper grammar delta           = 0
located production activation  = 0
fallback / retry / reselection = 0
```

Candidate B remains only a hard-stop escalation. It is not authorized by this
task.

## Why this is the next boundary

The post-Binary `DESCENT-SPINE0-CLOSE-AUDIT` found one first
production-reachable competing descent authority:

```text
RawInvocationChildPortV1
-> raw/default ASTNode::MethodCall
-> one prepared member route
-> InlineRecord / InlineSetter
-> callable-catalog helper declaration body
-> MirBuilder::build_expression / build_statement
```

The MethodCall route is selected once and keeps its associated invocation port
through argument lowering. The helper executor then erases that port to:

```rust
&mut dyn MethodCallArgumentDescentV1
```

That capability can lower call-site arguments only. It cannot lend the
collector/header capability to the catalog-owned helper declaration body.
`lower_record_helper_body_until_return` therefore starts a fresh unassociated
Legacy descent:

```text
Return Some(expr) -> self.build_expression(...)
prefix statement -> self.build_statement(...)
```

This violates the north-star direction because Lower re-enters an unrelated
raw AST ingress after route selection.

## Exact current sites

```text
production invocation port:
  src/mir/builder/module_lifecycle.rs
  RawInvocationChildPortV1::new

raw/default MethodCall selector:
  src/mir/builder/raw_expression_dispatch/mod.rs
  ASTNode::MethodCall

associated route execution:
  src/mir/builder/calls/member_route.rs
  execute_prepared_member_call_route_v1

prepared inline route:
  src/mir/builder/method_call_handlers.rs
  PreparedMeCallExecutionV1::InlineRecord
  PreparedMeCallExecutionV1::InlineSetter
  PreparedStandardMethodExecutionV1::RecordHelper
  PreparedStandardMethodExecutionV1::Setter

catalog declaration snapshot:
  src/mir/builder/record_helper_args.rs
  prepare_same_module_helper_declaration

port-erasure boundary:
  src/mir/builder/record_helper_args.rs
  execute_prepared_record_helper_inline
  execute_prepared_same_module_helper_setter_inline

old direct edges:
  lower_record_helper_body_until_return
    -> self.build_expression
    -> self.build_statement
```

Both direct edges must be covered by one responsibility decision.

## Authority that must remain separate

```text
call-site argument authority:
  MethodCallArgumentDescentV1
  lower_all / lower_index

helper declaration authority:
  callable catalog key, parameters, declarations, and body AST

call-site located authority:
  source path / role / ledger for the MethodCall child

helper completion authority:
  consume the inline helper Return as a value
  do not emit a physical Return for the caller function
```

The call-site location or ledger must not be reused as the provenance of the
helper declaration body.

## Closed decision question

The existing selected MethodCall descent object lends one source-neutral,
short-lived tagged catalog-child operation to the current helper-body
completion owner:

```text
CatalogHelperChildV1
  Statement(ASTNode) | Expression(ASTNode)
  -> exact associated port
  -> corresponding existing descent
```

The call-site MethodCall input, argument index, located role, source ledger,
and selected preloop token are not inputs to either operation.

## Accepted shape — T1 short reborrow

```text
selected MethodCall route
-> associated argument descent
-> short reborrow of the existing raw invocation port
-> exact helper-body driver
   prefix statements in order
   final Return value expression exactly once
-> inline result ValueId
```

Requirements:

```text
new identity/source/publication authority = 0
catalog ownership change                  = 0
call-site location reuse                  = 0
physical caller Return                    = 0
old direct helper-body edges              = 0
```

The helper-body grammar and completion remain in
`lower_record_helper_body_until_return`. Do not add a whole-body method to a
port. A whole-body method would leak statement ordering, top-level Return
recognition, Return-as-value completion, and missing-return diagnostics out of
their current owner.

## Rejected implementation shapes

Do not genericize every MethodCall handler merely to expose
`AssociatedMethodCallArgumentsV1::terminal_port()`. The current executor
intentionally receives `dyn MethodCallArgumentDescentV1`; widening the handler
graph would touch more route families than the bounded projection.

Do not add a new `CatalogHelperBodyDescentV1` object or a new body-session
product. The tagged operation fits the existing MethodCall descent interfaces and
new files are forbidden by the structural ratchet.

The `MethodCallDescentPortV1` method may have one stable fail-closed default
for custom ports that do not own catalog-helper descent. It must not call
`MirBuilder::build_statement`, `MirBuilder::build_expression`, or any raw
fallback. Every production-capable raw, Located, and Preloop port overrides it
explicitly. `MethodCallArgumentDescentV1` remains required with no default.

Do not make the Located or Preloop implementation consume call-site child
roles, ledger entries, or selected argument tokens. Their catalog-child
implementation must preserve the current unlocated compatibility behavior
explicitly:

```text
LocatedLegacyLoweringSessionV1
  -> raw compatibility statement/expression descent
  -> source / ledger / located child role untouched

PreloopLocatedArgumentPortV1
  -> ordinary port catalog-child operation
  -> selected argument token/state untouched
```

If this explicit delegation cannot preserve current behavior without
activating a located helper-body route, stop rather than substituting a
fallback or silent rejection.

## T2 escalation boundary

A T2 design is required if correct lowering needs any of:

```text
new helper-declaration provenance identity
new source-location owner
new publication or failure owner
helper-body grammar widening
located helper-body production activation
```

This task does not authorize that authority. It must describe the exact product,
issuer, consumer, and fail-fast boundary before implementation.

## Exact interface change

Use one crate-private transport tag:

```rust
enum CatalogHelperChildV1 {
    Statement(ASTNode),
    Expression(ASTNode),
}

fn lower_catalog_helper_child(
    &mut self,
    builder: &mut MirBuilder,
    child: CatalogHelperChildV1,
) -> Result<ValueId, String>;
```

The helper-body driver is the sole tag issuer. A port may only dispatch the
tag to the corresponding existing descent owner; it may not inspect the AST
to reclassify the helper grammar. No `Body` or `Return` variant is allowed.

`MethodCallDescentPortV1` owns the underlying port operation and a stable
fail-closed default. `MethodCallArgumentDescentV1` owns the required
object-safe projection through the erased helper executor. The associated
argument object forwards to its retained port without consulting its
MethodCall input.

The exact current implementer census is:

```text
MethodCallArgumentDescentV1:
  AssociatedMethodCallArgumentsV1<Port>
  LegacyMethodCallArgumentsV1
  PreloopLocatedStaticCompletionV1

MethodCallDescentPortV1:
  blanket Port: RawAstChildLoweringPortV1
  LocatedLegacyLoweringSessionV1
  PreloopLocatedArgumentPortV1

cfg(test) MethodCallDescentPortV1:
  RoutePort
  DistinctMethodCallPort
  FailingOuterStaticTerminalPortV1
```

Every production `MethodCallArgumentDescentV1` implementer and every
production-capable `MethodCallDescentPortV1` implementer must be updated
explicitly. Test-only ports that cannot reach a helper body use the stable
fail-closed default. In particular,
`PreloopLocatedStaticCompletionV1` must forward through its internal port; an
implicit default error would change existing static helper behavior.

The blanket raw implementation short-reborrows the same port into the
existing statement/expression drivers. For `RawInvocationChildPortV1`, this
retains the same `ModuleLoweringPortV1`, collector, and header authority across
nested MethodCalls. `RawLegacyChildLoweringPortV1` remains a separately
selected compatibility implementation, not a retry target.

## Atomic implementation

Production source:

```text
src/mir/builder/calls/method_call_descent.rs
  add one tagged helper-child carrier
  add one port operation and one erased argument operation
  forward Associated through its retained port
  implement explicit Legacy compatibility descent
  implement the raw blanket short reborrow

src/mir/builder/record_helper_args.rs
  pass descent through inline_record_helper_body
  pass descent through lower_record_helper_body_until_return
  consume top-level Return before statement descent
  lower only Return Some(expr)'s expression through the catalog hook
  lower prefix statements through the catalog hook

src/mir/builder/located_legacy_lowering.rs
  implement explicit unlocated compatibility projection
  do not read source, ledger, LegacyExprInputV1, or child roles

src/mir/builder/calls/preloop_located_argument_port.rs
  delegate catalog children to ordinary
  do not arm or consume the selected argument token

src/mir/builder/calls/preloop_located_outer_completion.rs
  forward the erased tagged operation through its internal port
```

`method_call_handlers.rs` already passes the descent object to the prepared
helper executors. It is not an expected production edit.

Atomic old-edge deletion:

```text
lower_record_helper_body_until_return
  -> self.build_expression(*expr.clone()) = 0
  -> self.build_statement(stmt.clone())   = 0

dead same-family facades:
  try_inline_same_module_helper_setter_call
  try_inline_same_module_helper_setter_call_with_descent
  try_inline_same_module_helper_setter_call_from_receiver_with_descent
```

The function itself remains the sole owner of:

```text
statement ordering
top-level Return recognition
Return-as-value completion
missing-return diagnostic
```

`Return Some(expr)` lowers the expression exactly once and returns its
`ValueId` as the inline helper result. `Return None` emits the existing Void
value. Neither shape is sent to the ordinary Return statement owner, and no
physical Return terminator is emitted for the caller function.

## Focused evidence

Do not create a new parity, test, or guard file.

Update existing tests only:

```text
src/mir/builder/record_helper_args_tests.rs
  port continuity:
    prefix statement(s) -> final Return expression
    one retained port instance
    call-site argument input access = 0

  failure / reuse:
    prefix statement failure
    final expression failure
    exact variable_map restore after both
    same Builder and port succeed afterward

  Return-as-value:
    final expression demand = 1
    inline result ValueId exists
    physical Return terminator = 0
    generic Call / BoxCall = 0

src/mir/builder/calls/method_call_descent_tests.rs
  explicit custom-port projection contract
```

Strengthen the existing real production fixture rather than adding another:

```text
tools/checks/impl/k2_wide_allocator_record_construction_read_guard.sh
  normal CLI -> MIR JSON
  helper prefix statement is exercised
  inline result reaches the caller's one real Return
  helper physical Return / generic Call / NewBox / FieldGet = 0
```

The private M0 route helper currently has a pre-existing first red:

```text
static me source/property standard ARG0 demand: expected=5 actual=4
```

Before using it as green evidence, perform an exact current caller census.
Update the stale count only if the four current callers are mechanically
proven. If the helper cannot become green through an evidence-backed current
graph correction plus this helper-only contract update, hard stop. Do not hide
the red or weaken unrelated assertions.

The shared replacement guard must gain only macro-level evidence:

```text
closed manifest row                         = 1
selected raw/default helper-body terminal   >= 1
old direct build_expression edge            = 0
old direct build_statement edge             = 0
located helper-body production activation   = 0
fallback / retry / reselection              = 0
```

No per-cell guard is allowed.

## Structural budget

Current measured footprint and accepted ceilings:

```text
                         current   ceiling   headroom
source files                 952       952          0
source LOC                182384    182452         68
test files                   139       139          0
test LOC                   40826     40826          0
```

Therefore:

```text
new source files             = 0
new test/check files         = 0
production Rust LOC delta   <= 68
test LOC after           <= 40826
all four ceilings            = green
all touched source/check     < 800 lines
five-cell rolling Rust LOC   <= 0
```

Focused evidence added to existing tests must replace or consolidate at least
the same number of test lines in the atomic implementation commit. A positive
test LOC delta is not accepted. Source growth is accepted only inside the
already-ratcheted 68-line ceiling and only while the five-cell rolling total
remains non-positive. The tagged carrier plus dead same-family facade
retirement is the bounded implementation; unrelated deletion cannot be used
to buy headroom.

## Commit boundary

Selection commit:

```text
docs(mir): select record helper body descent
```

Bounded interface correction:

```text
docs(mir): compact record helper descent capability
```

Immediately following atomic implementation:

```text
refactor(mir): thread record helper body descent
```

The implementation commit contains:

```text
one tagged bounded interface operation
all production-capable implementer updates
helper-body terminal switch
two old direct-edge deletions
three dead same-family facade deletions
focused existing-test consolidation
private M0 proof correction
shared guard extension
closed eighth manifest row
this card and current SSOT closeout
measured four-metric and production LOC closeout
```

Do not interleave proof consolidation, raw-body facade cleanup, the non-Program
root, another consultation, or an unrelated source edit.

## Gate order

```bash
# exact implementer / old-edge census
rg -n 'MethodCallArgumentDescentV1|MethodCallDescentPortV1' \
  src/mir/builder --glob '*.rs'
rg -n 'self\.build_(?:expression|statement)' \
  src/mir/builder/record_helper_args.rs

cargo check -q
cargo test -q record_helper_args --lib
cargo test -q method_call --lib

python3 \
  tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py
bash tools/checks/impl/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh

git diff --check
```

Zero-test filters are not green evidence. Record executed test counts in the
closeout.

## Rejected shortcut

Do not pass a located call-site port directly over the helper body. Its
`CallArgument` roles and ledger refer to the invocation source, not the
catalog declaration.

Do not lower the whole helper body through the ordinary Return statement
owner. An inline helper Return produces the inline expression value; it must
not emit a caller-function Return terminator.

## Preserved behavior

```text
record-helper eligibility                = unchanged
setter allowlist                         = unchanged
helper body accepted shapes              = unchanged
receiver binding                         = unchanged
argument evaluation order/count          = unchanged
record-local ABI                         = unchanged
variable-map restore on success/failure  = preserved
missing-return diagnostic                = preserved
fallback / retry / route reselection     = 0
```

`prepare_record_helper_inline` currently does not apply the setter-only
`is_inlineable_same_module_helper_body` check. Adding that check during this
descent cutover would change accepted behavior and is forbidden.

## Existing evidence to reuse

Do not create a new parity family.

```text
tools/checks/impl/k2_wide_allocator_record_construction_read_guard.sh
  real Main.make(meta) helper
  normal CLI -> MIR JSON
  no record NewBox / FieldGet
  inline result reaches Return

src/mir/builder/record_helper_args_tests.rs
  existing prepare/execute fixtures
  add focused port continuity plus failure/reuse evidence here if selected
```

## Acceptance

```text
production helper-body terminal census       = exact
declaration-body provenance owner             = one
call-site location / ledger reuse             = 0
nested body/statement/expression capability   = selected
MethodCallArgumentDescentV1 implementers       = exact, all explicit
production MethodCallDescentPortV1 implementers= exact, all explicit
unsupported custom-port default               = fail-closed
inline Return-as-value completion owner       = one
caller function physical Return emission      = 0
helper generic Call / BoxCall emission         = 0
variable-map restore on success/failure       = preserved
fallback / retry / route reselection          = 0
new owner and atomic old-edge delete set       = exact
ceremony                                      = T1
new proof file                                = 0
source/test file-count delta                  = 0 planned
four structural ratchet ceilings              = preserved
production Rust LOC delta                     <= 68
test LOC delta                                <= 0
five-cell rolling production Rust LOC         <= 0
all touched source/check files                < 800 lines
```

## Hard stop

```text
call-site location/ledger must describe helper declaration children
callable catalog identity/body ownership must change
record-local ABI, receiver binding, or argument order must change
helper body acceptance must narrow or widen
located InlineRecord / InlineSetter must activate
an implementer requires a raw fallback or behavior-changing rejection
fallback, retry, or route re-selection is required
the two old direct edges cannot reach zero in one bounded interface slice
private M0 proof cannot become green by an exact current census plus this contract update
production Rust LOC exceeds the ratchet or makes the rolling window positive
test LOC or any structural ceiling grows
```

## Other audit findings

These remain candidates, not current execution authority:

```text
BINARY-SOURCE-PARTITION-PROOF-CONSOLIDATION0
  executable proof cleanup
  production authority delta = 0

RAW-BODY-FACADE-RETIRE0
  safe delete-only cleanup candidate
  gross source deletion = 45 LOC
  blocked on stale E0 proof-authority disposition

non-Program root fallback
  separate COMPILER-RESIDUE0 responsibility
```

None outranks the live record-helper descent red.

## Non-claims

```text
no production source edit in the selection commit
no eighth replacement manifest row before the atomic implementation is green
no helper body grammar change
no located route activation
no new Recipe/CorePlan claim
no DESCENT-SPINE0 close claim
no proof consolidation or dead-facade cleanup
```
