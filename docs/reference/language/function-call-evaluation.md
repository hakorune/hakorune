# Function Call Evaluation

Status: accepted language target; compatibility census and production migration
remain pending.

## Decision

Hakorune has two semantic call shapes:

```text
FunctionCall(name, arguments)
  = a direct FreeStatic call or an explicitly defined special source form

Call(callee, arguments)
  = invocation of a value produced by a callee expression
```

The callee is fixed before argument evaluation. Arguments are then evaluated
exactly once from left to right. An assignment or other effect inside an
argument cannot retarget the enclosing call.

For a direct FreeStatic call, the sole target authority is:

```text
complete source callable inventory
  -> VerifiedCallableIndexV1
  -> exact source FunctionCall site
  -> ResolvedDirectCallTargetV1
```

The resolved function retains only the exact callable identity for that source
site. Lowering consumes it and does not search again by name.

Bare `error` is not a direct FreeStatic provider. It is an unsupported bare
name and rejects before argument evaluation; qualified providers such as
`env.console.error` remain separate Extern contracts and are not inferred from
the bare spelling.

Bare `now` follows the same unsupported-name contract and rejects before
argument evaluation. The qualified `env.now_ms` provider remains a separate
declared Extern owner; the bare spelling does not infer or alias it.

Bare `panic` is an explicitly defined terminal special form carried by the
ordinary `FunctionCall` source shape. Its exact reserved identity, arity `1`,
and verified `String` result contract are fixed before argument evaluation. It
does not enter FreeStatic, SameModule, Extern, provider, or value-call target
resolution. The message expression is then evaluated exactly once. A normal
message creates a pending noncatchable Panic Fault; an argument Fault remains
the earlier primary Fault. The common exit transaction drains cleanup and
Home obligations before the final successorless Fault terminal. Bare `exit`
is a separate undecided application/runtime surface, not a sibling selected by
this rule.

Within the SelectedNormal admission cohort, multiple source occurrences that
project to the same effective top-level `name/arity` physical symbol are an
ambiguous direct target. They reject before body lowering and before argument
evaluation. Declaration order, collector replacement, and a retry through the
RawCompatibility owner do not select a winner. Different arities remain
distinct FreeStatic identities; this is not Box-method overloading.
RawCompatibility keeps its own explicitly owned legacy replacement policy and
never re-enters SelectedNormal.

For a Box method call, the exact nominal owner/receiver relation selects one
Box namespace before argument evaluation. A Box has at most one visible method
per name across direct static/instance declarations and delegate forwarding
names. Arity validates that declaration and never searches another same-name
method. The same name on different Boxes is valid because the nominal owner is
part of the relation. A duplicate name in one Box, static/instance collision,
wrong arity, foreign relation, or ambiguous delegate exposure rejects before
argument effects, with no declaration-order choice or fallback.

`me.method(...)` follows the already selected receiver policy. A
`StaticCurrentOwner` call emits the exact static Global without fabricating a
receiver. A true declared-instance call retains the exact lexical receiver and
emits `Method(Some(receiver))`. Neither form may retry through the other merely
because its method spelling matches.

For a value call, the callee expression is evaluated once before the arguments.
Its resulting callable value is the callee authority. A Builder `ValueId` is a
physical projection of that result, not source-semantic identity.

## Non-authority

The following legacy mechanisms do not issue the language meaning above:

- Builder `variable_map` or `current_static_box` snapshots;
- post-argument `resolve_call_target`;
- bare static recovery by name and arity;
- current-module suffix/tail lookup;
- callable headers without the exact source call site;
- MIR ordering, tests, runtime observations, C, or assembly.

They may remain temporarily as compatibility implementation, but cannot be used
to admit a new canonical call route.

## Failure and effect order

Direct-call target classification occurs before argument effects. A missing,
ambiguous, or unsupported canonical target rejects before argument evaluation
and before MIR effects. After a target is admitted, argument failures are
observed in left-to-right source order.

Explicit special forms own their own source validation but must likewise choose
their form before descending into ordinary arguments. They cannot fall through
to FreeStatic resolution after partially evaluating arguments.

`panic` is not shadowable by a local, parameter, or source callable in v1.
Because it remains an identifier token rather than a lexer keyword, this rule
belongs to semantic declaration/name admission. Qualified member spellings
such as `obj.panic(...)` remain ordinary qualified calls.

## Compatibility boundary

The current raw Builder may resolve a bare name after argument lowering, consult
mutable Builder state, recover a unique static method, or use a development tail
resolver. Those behaviors are compatibility-only until their caller census and
diagnostic impact are classified. In particular, a shape equivalent to
`f((f = value))` may currently retarget the call; canonical semantics forbid that
retargeting.

The internal `ScriptRoot`, `RawScriptRoot`, `RawRootMain`, and `RawLegacy`
provenance names are not public compatibility profiles and do not issue a call
target. No new caller may depend on their post-argument resolver behavior. An
existing origin either migrates to the exact source-site authority above or
retires at an origin-specific typed terminal before argument effects. Preserving
one as a public compatibility contract requires an explicit profile, semantic
owner, and fixtures before the retirement edge is changed.

The internal `RawScriptRoot` ordinary non-special origin is retired at the
typed terminal `[raw-compat/raw-script-root-ordinary-retired]` before argument
effects. The semantic `ScriptRoot` ordinary non-special origin is likewise
retired at `[raw-compat/script-root-ordinary-retired]`; its source-backed path
already stops as `Deferred`/`ObservationDeferred` before physical open. The
`RawRootMain` origin is caller-zero upstream, and RawLegacy ordinary
non-special calls retire at `[raw-compat/raw-legacy-ordinary-retired]` before
argument effects. The shared `Resolved` consumer remains a separate legacy
owner outside this origin-specific slice. These are internal provenance
decisions, not public grammar changes.

The legacy static compatibility edge retires before argument effects when it
has no exact Cataloged issuer. Static receiver, static `this`, receiverless
`me`, and unissued lowered-static routes share one typed terminal; exact
Cataloged, scalar/inline, qualified Math, Env/Extern, receiver-bearing
`Method`, and lowered-global instance owners remain unchanged.

Migration must not silently relabel a behavior change as BoxShape. It requires:

1. a complete compatibility caller/fixture census;
2. explicit classification of local callable, current-static, recovery, and tail
   routes;
3. source-level positive and negative witnesses for target and first-error order;
4. atomic removal of each selected post-argument lookup edge when its canonical
   replacement becomes active.

Script activation remains a separate bounded BoxCount after the exact resolver
target and callable header/result relation are available to Script admission.
