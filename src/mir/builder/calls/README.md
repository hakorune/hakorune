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
