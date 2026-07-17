# Same-module callable result representation

This module seals one disconnected exact-`i64` sufficient-condition catalog
from the complete same-module callable declaration catalog.

This S0a substrate owns:

- one result row for every `StaticBoxMethod` declaration key;
- sorted, unique parameter ordinals required to be exact `i64` at a call site;
- pure source-body representation proof;
- explicit `Unavailable` reasons for valid but unsupported source.

It borrows the declaration catalog while sealing and retains only canonical
keys plus result dispositions. It never stores AST bodies, headers, a second
callable index, `ValueId`, `MirType`, Builder state, function metadata, or
runtime facts. Instance methods receive no result rows.

It does not project call targets. A complete declaration catalog proves which
callables exist, but it does not prove which earlier Builder route wins for a
source call. Therefore every `FunctionCall` and `MethodCall` result is sealed
as `StaticCallTargetAuthorityUnavailable`, including bare, qualified,
current-owner, recursive, and builtin-collision spellings. The actual
`StringHelpers.skip_ws/2` local body remains exact; its parser wrapper remains
parked until a canonical source-call target product exists.

All call arguments are still observed once in source order before the call
result closes to unavailable. The first
proof grammar rejects `GroupedAssignmentExpr` anywhere in an untyped body;
this prevents short-circuit and eager-argument binding effects from becoming
an implicit second evaluation-order authority.

Import aliases, physical MIR symbols, declaration-name recovery, dynamic
receivers, properties, and runtime tags are not identities here. Conditions may contain
unsupported expressions when they do not feed a returned exact-`i64` value.

S0a is disconnected: production producers, consumers, call-result publication,
lowering behavior, runtime behavior, and backend behavior remain zero.
