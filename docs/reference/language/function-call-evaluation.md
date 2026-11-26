# Function Call Evaluation

Status: accepted language target. File-mode static calls and missing bare
targets are resolved before lowering. Production DeclaredInstance Method(Some)
migration remains pending.

## Decision

File functions live in static boxes under [ADR-005](../../development/adr/adr-005-file-functions-live-in-static-boxes.md).
Inside a static box, including in `Main.main`, `me.f(x)` calls the method
declared in that box. Outside the box, use `Box.f(x)` or a `using` alias
for that box. Resolve selects the declaration at the original call site.
Both static receiver spellings use the qualified static lowering and its
result and cleanup checks.

A bare `f(x)` in a file must name an admitted special form or a lexically
bound callable value. For an unbound ordinary name, Resolve reports
`call/target-missing` at the call site. It suggests `me.f(...)` only when
the current static box declares `f`. See [Reserved bare call forms](#reserved-bare-call-forms)
and the separate rules for [future REPL session names](repl.md).

The callee is fixed before argument evaluation. Arguments are then evaluated
exactly once from left to right. An assignment or other effect inside an
argument cannot retarget the enclosing call. For a known static declaration,
a wrong argument count is reported as `call/arity-mismatch`.

### Reserved bare call forms

`print(expr)` is a statement with exactly one expression. The parser produces
a Print statement directly. Expression calls and method lookup do not provide
the bare Print form.

`mem.name(args)` is reserved within a
[FastMemory contract region](low-level-capabilities.md#fastmemory-contract-regions)
written `fastmem ContractName { ... }`. The region selects the FastMemory
intrinsic. Outside the region, member calls use ordinary receiver resolution.

A reserved call never selects a same-named method by bare-name lookup.
Use `me.name(...)` or `Box.name(...)` to call a declared static method.

Bare `error` is reserved and rejects before argument evaluation; qualified providers such as
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
Home obligations before the final successorless Fault terminal.

Current file-mode implementation rejects bare `panic(...)` as
`call/target-missing`, including when a box declares a method named `panic`.
The terminal Fault implementation is still pending. `me.panic(...)` and
`Box.panic(...)` use ordinary method resolution.

Bare `exit(...)`, `str(...)`, and `Range(...)` have no special target. An
unbound call follows the ordinary `call/target-missing` rule. The standalone
`a .. b` expression has been retired at every sugar level. The
`loop i in a..b { ... }` range header remains part of loop syntax.

A [declared Brand](brands.md) owns its constructor spelling through its
declaration. Math operations use [MathBox methods](../boxes-system/box-reference.md#mathbox),
such as `math.sqrt(16)`. An unbound bare math name follows the ordinary
`call/target-missing` rule, including when a static box declares a method of
that name.

### Declared call target selection

Current implementation until ADR-005 step 3: within the SelectedNormal
admission cohort, multiple source occurrences that project to the same
effective top-level `name/arity` physical symbol are an ambiguous direct
target. They reject before body lowering and before argument evaluation.
Declaration order, collector replacement, and a retry through the
RawCompatibility owner do not select a winner. Different arities are still
distinct FreeStatic identities there; this is not Box-method overloading.
RawCompatibility keeps its own explicitly owned legacy replacement policy and
never re-enters SelectedNormal. Under ADR-005 a top-level function is itself
the compile error, and a moved function follows the one-name-per-Box rule
below.

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

This is one source syntax, not an overload or fallback syntax. Semantic
admission first fixes the receiver policy and the nominal Box declaration:

```text
StaticCurrentOwner
  -> exact same-session static declaration
  -> Global(StaticBoxMethod)
  -> N explicit source arguments

DeclaredInstance
  -> exact lexical receiver binding
  -> one visible method in that nominal Box namespace
  -> Method(Some(receiver))
  -> N explicit source arguments
```

The same source spelling is classified by its enclosing receiver context; it
is never reinterpreted after argument evaluation:

| Source form | Semantic family | Canonical target | Receiver rule |
| --- | --- | --- | --- |
| `me.method(args)` in a `static box` with an exact static declaration | `StaticCurrentOwner` | `Global(StaticBoxMethod)` | no fabricated receiver; `args` are the explicit arguments |
| `me.method(args)` in an instance method, or `obj.method(args)` | `DeclaredInstance` | `Method(Some(receiver))` | the exact lexical/object receiver is carried once |
| `host.exposed(args)` for a declared delegate exposure | `DelegateForward` | exact host-field/delegate relation | the exposure's declared field relation supplies the forwarded receiver; no name fallback |
| `obj.field.method(args)` | explicit delegate-object call | the nominal target of `field` | the field path is part of the owner relation |
| `Box.method(args)` naming a `static box` (or a `using` alias of it) | qualified static call | the exact static declaration of that Box | no receiver; the named Box is the owner |
| `f(args)` | `FreeStatic` (current implementation until ADR-005 step 3) | exact `name/arity` identity | no Box receiver |

Under ADR-005 the `f(args)` row leaves the file language: a file function is
called through the `me.method(args)` or `Box.method(args)` row. `f(args)` stays
the spelling of explicitly defined special forms such as `panic(message)`, of
value calls, and of future REPL session functions
([repl.md](repl.md) section 2.4).

If the enclosing context has no exact declaration in the applicable family, the
call rejects before receiver materialization and before argument effects. A
static `me.method` is never retried as an instance call, an exposed delegate is
never retried as a direct host method, and an explicit delegate field path is
never collapsed into a host-name lookup. These are language distinctions, not
overload or fallback syntax.

The method name selects within that one nominal Box namespace; arity validates
the selected declaration and never searches a second same-name declaration.
Static/instance declarations and delegate exposures therefore collide on the
same visible name even when arity or receiver policy differs. A distinct
explicit delegate alias is valid, and the same spelling on another nominal Box
is valid because the owner relation differs. The file language follows the
box method namespace of ADR-005. The StaticCurrentOwner handoff and resolver-owned
source-site/declaration relation are landed implementation slices; true
DeclaredInstance production Method(Some) remains a design stop until
effect/result contracts, the full receiver lane, and selected physical
capability are co-sealed.

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

Resolve rejects a missing call declaration before argument evaluation. The
call site's diagnostic is `call/target-missing` for a bare ordinary name or
a missing method in the current static box. A file-level `function` or
`static function` declaration is reported separately as
`decl/function-outside-box` at the declaration under ADR-005.

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

## Selected package diagnostic order

Within selected callable package issuance, parameter/declaration validation
precedes Dynamic owner selection. Dynamic source/Recipe/exit selection then
precedes ordinary Completion/S6C publication. If both latter stages are invalid,
the Dynamic error is reported first and the package is not installed. A failed
Dynamic candidate never retries as an ordinary callable. This compiler ordering
does not change runtime argument evaluation or grant an owning-slot capability.

After Dynamic selection, ordinary source preflight completes each declaration
in retained batch order before proceeding to the next. Within a declaration,
New candidate validation precedes Completion/result validation in the same
source loan. Thus a missing Birth candidate is diagnosed before an unsupported
result annotation in that declaration. S6C consumes its Completion only after
the ordinary source cohort succeeds. This replaces the former whole-cohort
Completion-before-New diagnostic ordering; it does not widen accepted programs.

Successful Dynamic selection retains its canonical Completion as the sole
completion input for its installed callable view. Parameter/result validation
and downstream physical-header checks still apply; ordinary result retention
does not verify or own that successful slot a second time. A declined Dynamic
candidate's later ordinary path is a separate case, with no global one-pass claim.

For the bounded AppMain-to-ordinary-Map compiler connection, exact Integer
literal arguments are paired with declaration-issued I64 formals. The source
terminal Call retains a pending return only on Normal; Fault uses caller
Completion cleanup without publishing the result. Early target/arity errors
remain first. Unsupported Map-call correspondence fails before argument effects
instead of using the scalar Call path. This source connection does not yet
activate the ordinary callable Fault ABI or object/executable emission.

Compiler retention note: successful callable lowering preserves the original
ordinary Completion and its same-package formal/selected correspondence through
finalization. This is ownership transport only; it does not admit ordinary
lifecycle Call execution or infer an absent result/cleanup contract.
