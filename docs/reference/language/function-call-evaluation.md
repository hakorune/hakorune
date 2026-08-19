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

## Compatibility boundary

The current raw Builder may resolve a bare name after argument lowering, consult
mutable Builder state, recover a unique static method, or use a development tail
resolver. Those behaviors are compatibility-only until their caller census and
diagnostic impact are classified. In particular, a shape equivalent to
`f((f = value))` may currently retarget the call; canonical semantics forbid that
retargeting.

Migration must not silently relabel a behavior change as BoxShape. It requires:

1. a complete compatibility caller/fixture census;
2. explicit classification of local callable, current-static, recovery, and tail
   routes;
3. source-level positive and negative witnesses for target and first-error order;
4. atomic removal of each selected post-argument lookup edge when its canonical
   replacement becomes active.

Script activation remains a separate bounded BoxCount after the exact resolver
target and callable header/result relation are available to Script admission.
