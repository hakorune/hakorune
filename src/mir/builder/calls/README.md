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
