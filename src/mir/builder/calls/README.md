# MIR Builder calls

`build.rs` is the member/function-call orchestration entry. Reserved source
`MethodCall` admission is not owned here: the sole policy lives in
`mir::policies::source_method_reserved_route`.

`reserved_method_route.rs` projects the active Builder FastMem session into
the neutral policy context, consumes one typed decision, and delegates only
selected execution. `debug_method_routing.rs` emits verified MIR-debug and
REPL payloads; `fastmem/calls.rs` owns FastMem intrinsic vocabulary and arity.
Those execution modules must not rediscover receiver, method, or argument
admission by name.

Function-form `mem.*` calls are a separate existing route and are outside the
source-MethodCall policy.

`call_argument_descent.rs` owns one behavior-neutral argument boundary:
moved-state preflight happens before effects, then each associated argument
input is checked and lowered exactly once in source order. Its selected raw
facade preserves the existing AST lowering. It owns no receiver or route
decision, result publication, callable-result location/ledger, retry, or
fallback policy. The port is never stored in `MirBuilder`.

`method_call_descent.rs` is the associated-input MethodCall child boundary.
It exposes one borrowed syntax view plus reusable E0 receiver and ARG0 argument
descent primitives. It does not select routes or emit calls, effects, types,
or results, and it is never stored in `MirBuilder`.

`unified_emitter/physical_terminal.rs` is the sole generic physical
`MirInstruction::Call` writer. It issues a non-Clone value receipt only after
the finalized Call succeeds and the existing post-success facts commit.
Compatibility, rewrite, BoxCall, no-destination, and failed-emission routes
never issue that receipt. `unified_emitter/request_boundary.rs` owns the
receipt-required sibling API; it rejects alternate and legacy routes without
retry and owns no source classification or result-type publication.

`preloop_located_argument_port.rs` is the disconnected candidate-only wrapper
for one source-sealed pre-loop argument. It delegates every ordinary trait
capability to the wrapped `MethodCallLoweringPortV1` and leaves the selected
structural argument fail-closed until its later isolated candidate ingress.
Its one typestate retains the exact source association in the current rejected
terminal; payloadless consumed/poisoned states and a separate route-state
field are forbidden. The concrete reached/request product is introduced only
when the later ingress has a real success boundary.
It never stores state in `MirBuilder`, creates no second ordered argument
driver, does not convert the selected input to RawLegacy syntax, and owns no
Call receipt or type publication.
S0 production consumers = 0. Exact route demand remains owned by the later
R0/M0 rows; inactive raw terminals will require the existing ledger's
inactive-prefix proof before located lowering may delegate to them.

ROUTE0 is one behavior-neutral Refactor Series: S0 adds this disconnected port,
GUARD0 restores one exact recursion-depth guard around raw expression descent,
R0 threads exact reserved-route child demand, and M0 threads TypeOp and the
static/env/me/standard routes. Syntax-only TypeOp strings and MIR-debug labels
must never become evaluated arguments; static/env/me/reserved receivers must
never become evaluated receivers.

The GUARD0 owner lives at the raw expression port implementation. Public and
nested raw expression descent both reach that owner exactly once; the public
facade owns no second depth counter. Limit failure and ordinary lowering
failure both restore the entry depth before the same Builder session is reused.

ROUTE0-R0 selects the neutral reserved-route decision once, then requests only
the children that route already evaluates. Reserved receivers are syntax-only.
MIR-debug labels and every `mark` extra argument remain unevaluated; `log`
descends only indices one and later through the indexed E0 primitive. REPL uses
the full ARG0 boundary. FastMem keeps its syntax preflight before indexed E0
descent and shares one intrinsic core with its function-call facade. Ordinary and
reserved-failure decisions descend no children. Terminal emission, result/type
publication, located inputs, and ledger authorization remain outside R0.

ROUTE0-M0 is closed through S0/H0/I0/P0/G0. It reuses the existing non-Clone
`MemberCallRoutePlan`; no second demand or route product is introduced. TypeOp
and Standard descend the receiver exactly once, while Static, Env, Me/This,
and reserved routes never descend source receiver syntax. TypeOp's type string
is syntax-only. Ordinary route arguments use ARG0 only after each route's
existing preflight.

Record-helper scalarization is intentionally not a full-ARG0 consumer:
record-local slots bind their existing values and only non-record slots use the
indexed E0 primitive. Its inline body remains a separate terminal authority.
`property_reads.rs` is also separate because it already owns a materialized
receiver value; the source MethodCall driver must not synthesize an AST for it
or duplicate its standard-handler preflight.

`method_call_terminal.rs` is the disconnected V0 value-only terminal port.
Route selection, syntax preflight, and child descent must finish before it is
called. It is source-neutral: it accepts only already-materialized terminal
operands and does not borrow a `MethodCallInput`. Its raw adapter preserves TypeOp, qualified-static global,
current-owner lowered global, Env extern, and Standard method emission. It
owns no route table, callable key, effect/result inference, located source,
caller ledger, retry, or fallback. V0-S0 production consumers = 0.

V0-I0 threads exactly the five ordinary source completions through the same
associated MethodCall input: TypeOp, qualified static, current-owner lowered
global, Env, and Standard. Early/custom scalar, record-helper, setter, weak,
FastMem, MIR-debug, and REPL terminals remain outside this boundary. A
materialized property keeps the shared Standard preflight and calls only the
raw value-level Standard helper; it creates no MethodCall source carrier.
Located source, caller-ledger, activation, and result authority remain absent.

V0-P0/G0 fixes normalized destination allocation, target/effects, argument
order, Env returning/no-result behavior, and existing type/origin publication
without adding a production snapshot product. Static-scalar, weak-load,
record/helper-setter, FastMem, MIR-debug, and REPL remain explicit custom
terminal owners. Receiver, argument, preflight, and terminal failures enter no
later ordinary terminal, never retry, and leave the Builder reusable. The one
V0 structural guard owns only source-consumer/custom-owner counts and evidence
presence; it is not a route, result, type, or effect authority.

`located_legacy_lowering.rs` is the disconnected EXPR0-L0 session above the
E0/ARG0/ROUTE0/V0 ports. One stack-scoped, non-Clone session owns the source
view and exact caller ledger borrowed from one activation plan. Every selected
or unselected MethodCall row is claimed before route preflight or child
descent. Receiver and argument inputs come only from PATH0 roles, and the
existing expression recursion guard is entered exactly once.

Raw body, statement, or expression delegation requires an exact inactive-prefix
proof. A non-MethodCall prefix containing any activation row therefore fails
closed; L0 does not add a second expression walker. Any location, claim, child,
route, or terminal failure poisons the session, forbids retry, and prevents a
successful finish. The plan, source view, ledger, or current claim is never
stored in `MirBuilder`. EXPR0-L0 has zero production callers and publishes no
callable-result representation; active non-MethodCall spines remain a separate
future boundary if C0 requires them.
