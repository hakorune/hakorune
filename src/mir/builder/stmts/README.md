# MIR Builder statement boundary

This directory owns source-statement orchestration after the expression
dispatcher has selected a statement family.

## Existing semantic owners

- `variable_stmt.rs`: Local declaration preflight, specialized typed-array and
  record initializer lowering, binding publication, LocalSlot contracts, and
  result metadata.
- `return_stmt.rs`: Return preflight, cleanup, contracts, and emission.
- `block_stmt.rs` / `block_driver.rs`: statement order, scope, termination, and
  the existing suffix-router boundary.

## Local associated-input descent

`local_statement_descent.rs` is a child-demand boundary. It may:

1. borrow the Local declaration syntax once;
2. run the existing whole-declaration exact-numeric/typed-array preflight
   before initializer effects;
3. request ordinary initializer expressions in declaration order through the
   shared recursive child-lowering port;
4. request the existing typed-array or record initializer owner through an
   explicit specialized hook;
5. publish bindings once through the existing from-values completion owner
   after every initializer succeeds.

The specialized hooks are not permission to bypass located coverage. A future
located port must first prove the exact `LocalInitializer(index)` subtree
inactive before invoking either hook. Active array elements or record
arguments require their own associated-input row and fail closed in LCL0.

This boundary must not reconstruct source sites, own a caller ledger, infer
types, reimplement typed-array or record semantics, publish a binding before
all initializer values exist, or store its port/input in `MirBuilder`.

LCL0-I0 selects the owned raw Local input through the existing
`variable_stmt::build_local_statement` facade. That facade has one consumer of
the raw adapter; the old initializer loop is retired. The shared driver keeps
the existing preflight-success debug observation in its original position,
before initializer effects.

LCL0-P0 keeps one `cfg(test)` pre-I0 orchestration reference. Selected and
reference paths compare exact results plus normalized MIR, transient type and
origin facts, bindings/scopes, specialized array/record contracts, slot state,
allocator counters, partial failure state, and same-Builder reuse. The
reference must not call the selected driver or either descent port.

LCL0-L0 adds one disconnected located Local selector. Ordinary initializers
are requested only through the existing `LocalInitializer(index)` source role,
so expression descent retains the exact site and ledger order. Typed-array and
record hooks require exact inactive-subtree proof before specialized effects;
an active element or constructor argument fails closed. The adapter does not
catch `RowsUnderPrefix` to probe a route, reconstruct a site, or publish a
production located root caller.

The canonical source grammar currently admits at most one initialized binding
per `Local`, so its reachable located role is `LocalInitializer(0)`. The shared
driver deliberately keeps an indexed associated-input contract; its synthetic
port fixture fixes `LocalInitializer(0)` then `LocalInitializer(1)` ordering.
Located LCL0 binds every requested index to the exact source role, but does not
invent a malformed multi-initializer source fixture to widen the parser law.

## Variable-target Assignment associated-input descent

`variable_assignment_descent.rs` owns one disconnected exact Variable-target
Assignment boundary. Its input carries only the already-selected variable name
and RHS; field/index target syntax is structurally absent. It observes the name
once and runs the existing declared-binding preflight before requesting the
RHS. The RHS is lowered once through the shared recursive expression port,
then the existing `build_assignment_from_value` owner repeats the declaration
check and performs contracts, ownership effects, and binding publication.

ASN0-S0 must not inspect or reconstruct target AST, admit field, index,
compound, or grouped assignment targets, inspect callable-result rows, or
publish a binding itself. An undeclared target rejects before RHS input or
effects, and an RHS failure leaves the previous assignment binding unchanged.
The second completion-time declaration check is retained.

ASN0-I0 selects this raw driver exactly once from the exact Variable branch of
the existing `exprs.rs` target selector. Field/index targets and compound
assignment retain their existing owners. `GroupedAssignmentExpr` remains
parked on a dedicated legacy facade because sharing its old facade would widen
ASN0 indirectly. The raw adapter does not inspect the target AST or add a
second selector. The parity reference and located `AssignmentValue` navigation
remain disconnected at I0.

ASN0-P0 retains the pre-I0 exact Variable orchestration only in a cfg(test)
reference: declared-binding preflight, raw RHS lowering, then the existing
from-value completion. Selected and reference paths compare result/error,
ordered MIR, transient facts, bindings/scopes, local and typed-array contracts,
slot/cached state, allocators, failure effects, and same-Builder reuse. The
reference rejects Grouped, field, index, and compound surfaces and has no
production caller. Located `AssignmentValue` navigation remains disconnected
until ASN0-L0.

ASN0-L0 adds one disconnected located adapter in
`located_legacy_assignment.rs`. It selects an exact Variable-target statement
once, preserves the outer statement recursion guard, and derives the RHS only
through the existing `AssignmentValue` role. The shared driver still owns the
declared-binding preflight and completion; the located expression session and
caller ledger still own MethodCall claims and order. Field/index/compound and
If/Loop statement surfaces fail closed instead of probing a raw route.

The current Loop-body source-view carrier includes a parked `LoopBodyRoot`
path seam that does not match the actual callable-result ledger row. ASN0-L0
records that mismatch as a no-effects negative fixture; it neither rebuilds
the path nor claims Loop traversal. Exact Loop body carriage remains LOOP0
authority.

## Value-bearing Return associated-input descent

`return_statement_descent.rs` owns one disconnected orchestration boundary for
`return value` only. The driver runs the existing cleanup prohibition first,
observes one required value syntax, delegates the existing match-return probe
through the port, requests `ReturnValue` once only when that probe declines,
lowers the child through the shared recursive expression port, and completes
through the existing `emit_return_from_value` owner.

The match hook is not a second Match/CorePlan authority. Raw lowering delegates
to the existing `try_apply_match_return_optimization`; a future located port
may invoke that owner only after proving a Match-valued subtree inactive. A
selected match result bypasses ordinary child demand and completion exactly as
before. Cleanup/defer policy, CorePlan composition, Return ABI/contracts,
instruction emission, source location, and callable-result ledger ownership
remain outside this driver.

RET0-S0 does not admit `return;`: the input and syntax view both carry one
non-optional value. Void emission remains on the existing legacy facade. A
cleanup rejection happens before port observation, while match-hook, input,
or child failure emits no Return completion. The driver must not reconstruct
sites, inspect Match facts, emit Return directly, retry another route, or
store its input/port in `MirBuilder`.

RET0-I0 selects that driver exactly once inside the existing Return facade
when the source value is present. The facade performs no cleanup or Match work
before selection; those remain ordered once inside the driver. It keeps the
`None` branch on the legacy Void path, including cleanup, the existing
no-value Match observation, `emit_void`, and the existing completion owner.
Expression dispatch remains a thin caller of the Return facade, and failure
never retries the retired inline value-lowering route.

RET0-P0 keeps one `cfg(test)` pre-I0 Return orchestration reference with no
production caller. It preserves the former span, cleanup, Match probe,
child-or-Void lowering, and completion order without calling either selected
driver. Selected and reference paths require exact normalized parity across
result/error, MIR, transient types/kinds/origins/literals, exact facts,
bindings/scopes, caches, allocator counters, span, cleanup state, defer state,
partial failure effects, and same-Builder reuse. The reference owns no located
source, callable-result ledger, Match/CorePlan policy, retry, or fallback.

RET0-L0 adds one disconnected exact `Return { value: Some(_) }` adapter. It
publishes the existing statement span and recursion shell, then gives the
shared Return driver one mandatory syntax value and derives its child only
through the existing `ReturnValue` source role. A Match-valued Return must
first prove that exact located value subtree inactive before invoking the
existing raw Match/CorePlan owner; an active row below Match fails closed
before Match, child, or Return effects. Ordinary values continue through the
located expression spine and its ledger claims. Void Return stays outside the
adapter on the existing inactive legacy path. The adapter owns no site/path
construction, call-row claim, cleanup/defer policy, completion, retry, or
production located root.

## Statement If associated-input descent

`if_statement_descent.rs` owns one disconnected statement-If child-demand
boundary. It observes exact If syntax, lowers one condition through the shared
expression port, preserves the existing FastMem post-condition verification
and fact publication, and delegates control to the existing IfForm owner. Each
branch carrier is requested lazily at its existing execution point; else is
never requested before then succeeds. The new driver has no production caller
in IF0-S0.

IfForm remains the sole block, EdgeCFG, variable-snapshot, scope/debug,
termination, result/variable PHI, JoinIR selection, and diagnostic authority.
Its branch execution seam is one callback invoked at the existing then and
optional else points; the legacy wrapper still lowers its raw Program inputs,
while the disconnected driver lowers only associated `BodyInput`s. IfForm is
not copied. Condition and FastMem failure happen before IfForm CFG effects;
branch carrier failure preserves the existing partial IfForm state.

FastMem policy does not enter the port. One thin success-only completion seam
sequences the existing Void emitter after control lowering for both the raw
`block_stmt` facade and the disconnected located adapter; the constant emitter
remains the sole Void-representation owner. Expression-position If remains on
`cf_if`, and the canonical resolved located-If route remains separate. IF0-S0 owns no
source paths or caller ledger. It owns no Match, Loop, suffix routing, PHI
emission, retry, fallback, or Builder-stored port/input. A later located port
may expose a raw branch only after the complete associated typed body domain
is proven inactive.

IF0-I0 selects the raw driver exactly once from `build_if_statement`. The old
inline FastMem split and direct `cf_if` selection are retired without a probe
or retry. One production raw If port preserves the retired branch Program shell:
each demanded branch is wrapped in `ASTNode::Program { span: Span::unknown() }`
and lowered through the existing raw expression recursion guard. This keeps
the former branch recursion boundary and empty-branch span behavior while the
generic associated-input driver remains free of source-span policy.

`block_stmt::build_statement` remains the sole raw statement-source selector;
the disconnected located session owns one exact statement-If selector but has
no production root caller. Both selectors use the shared success-only
completion seam, while the raw driver and its production port publish no
statement Void. Expression-position If continues through `exprs.rs` and
`cf_if`; the canonical resolved located-If route remains separate.

IF0-P0 keeps one cfg(test)-only pre-I0 reference. It directly replays the
retired ordinary `cf_if` and FastMem orchestration, unknown-span Program branch
shells, outer statement span, and success-only facade Void; it never calls the
selected driver or production raw port. Exact snapshots compare CFG edges and
spans, PHIs, transient facts, scopes, allocators, FastMem facts, failure state,
recursion boundaries, and reuse. Production behavior and caller counts remain
unchanged. Nested statement If is not a reference fixture because it would
re-enter the selected statement route from inside the old Program shell.
