---
Status: active design consultation
Date: 2026-07-28
Decision: pending T1/T2 responsibility boundary
Scope: callable-catalog helper declaration body descent after one selected MethodCall route
Parent:
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
NorthStar:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
---

# RECORD-HELPER-BODY-DESCENT0-D0

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

## One decision question

> How does a callable-catalog helper declaration body borrow a short-lived
> nested statement/expression descent capability that retains the current
> invocation collector/header authority, does not reuse call-site location or
> ledger authority, and completes the inline helper Return as a value?

## Candidate A — T1 short reborrow

Preferred if the current catalog snapshot and raw invocation capability are
sufficient:

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

If this is sufficient, ceremony is T1 because the existing capability
interface gains one bounded declaration-body responsibility.

## Candidate B — T2 declaration-body authority

Use only if Candidate A cannot express the contract.

A T2 design is required if correct lowering needs any of:

```text
new helper-declaration provenance identity
new source-location owner
new publication or failure owner
helper-body grammar widening
located helper-body production activation
```

This D0 does not authorize that authority. It must describe the exact product,
issuer, consumer, and fail-fast boundary before implementation.

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

## D0 acceptance

```text
production helper-body terminal census       = exact
declaration-body provenance owner             = one
call-site location / ledger reuse             = 0
nested body/statement/expression capability   = selected
inline Return-as-value completion owner       = one
caller function physical Return emission      = 0
variable-map restore on success/failure       = preserved
fallback / retry / route reselection          = 0
new owner and atomic old-edge delete set       = exact
ceremony                                      = T1 or T2 decided
new proof file                                = 0
source/test file-count delta                  = 0 planned
four structural ratchet ceilings              = preserved
all touched source/check files                < 800 lines
```

## Hard stop

```text
call-site location/ledger must describe helper declaration children
callable catalog identity/body ownership must change
record-local ABI, receiver binding, or argument order must change
helper body acceptance must narrow or widen
located InlineRecord / InlineSetter must activate
fallback, retry, or route re-selection is required
the two old direct edges cannot reach zero in one bounded interface slice
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
no production source edit
no eighth replacement manifest row
no helper body grammar change
no located route activation
no new Recipe/CorePlan claim
no DESCENT-SPINE0 close claim
no proof consolidation or dead-facade cleanup
```
