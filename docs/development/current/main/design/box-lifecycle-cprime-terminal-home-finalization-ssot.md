---
Status: Durable lifecycle design SSOT; accepted target, production activation 0
Decision: OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0 accepted; OWN-EXPLICIT-HOME-RELEASE-STMT0-D0 supersedes the earlier ordinary-call spelling on 2026-08-05
Scope: terminal Home finalization, explicit early Home release, Box fini hook, field release, Shared boundary, and C-speed stop lines.
Supersedes: box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
Related:
  - docs/reference/language/lifecycle.md
  - docs/reference/language/ownership.md
  - docs/reference/language/scope-exit-semantics.md
  - docs/development/current/main/design/ownership-home-model-ssot.md
  - docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md
  - docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md
---

# C′ Terminal Home Finalization SSOT

## Decision

C′ replaces B′ eager logical finalization. `fini` is not an ordinary callable
method and not a second object-wide termination authority. It is the optional
user hook inside the one terminal Home DropPlan.

```text
birth(args) = new-only construction hook
release root = release one verified whole-root Home at this source point
fini { ... } = last-Home-only finalization hook
close()/shutdown()/commit()/abort() = ordinary domain methods
cleanup { ... } = lexical exit action
```

Canonical v1 rejects:

```text
obj.fini()
fini() method declaration
delegate/interface/alias exposure of fini
explicit FinalizeObject source authority
Dead-with-live-Home as a normal lifecycle state
last-strong structural drop that bypasses a declared fini hook
drop root / drop(value) as a second source spelling
release by parser/MirBuilder identifier matching
```

This is an explicit constitutional supersession of B′. B′ remains historical
evidence and must point here rather than being silently rewritten.

## Source surface

```hako
box File {
    handle: NativeHandle
    closed: bool = false

    birth(handle) {
        me.handle = handle
    }

    close(): Result<void, IoError> {
        ...
    }

    fini {
        if !me.closed {
            me.closeBestEffort()
        }
    }
}

work(path): Result<void, IoError> {
    local file = open(path)?
    file.close()?
    release file
    return Result::Ok(void)
}
```

`close` is not a keyword or privileged callable role. It is an ordinary method
used when the program needs an exact shutdown time or a `Result` that a caller
can handle. A Box may instead name that method `shutdown`, `disconnect`,
`commit`, or omit it entirely. A later automatic `fini` must safely observe the
already-closed domain state.

## Explicit early Home release

`release root` is the accepted source spelling for ending one available Home
before its lexical scope ends. It does not force the object identity Dead and
does not directly invoke `fini`:

```text
release one verified Home
-> Shared non-terminal: hook 0
-> terminal: enter the same sole C′ TerminalHomeDropPlan
```

The source surface is a statement-only contextual keyword. In v1 it accepts
exactly one identifier root and returns no value. The parser publishes a
source carrier, not semantic authority. Resolution, Home Flow, and
`VerifiedExplicitHomeReleasePlanV1` must prove the exact root and synchronous
consume before any owner-ending effect. Parser/MirBuilder identifier matching,
an ordinary or generic wrapper Call, and backend special casing are forbidden.
Existing `release(value)` functions and `obj.release()` methods remain ordinary
callables and never acquire Home meaning from their spelling.

The first profile admits only a verified whole-root owning local or owning
parameter with exactly one available Home. It rejects ordinary handles, `me`,
fields, indexes/projections, container slots, owner-bearing composites, and
unknown/generic capability before effects. Generic/composite Home-bundle
classification remains provisional, but it does not turn `release root` into a
generic function. `ExactHomeRoot` is a compiler capability, not a source
interface. Trivial/non-Home roots reject instead of becoming a silent no-op.

Releasing a root consumes it and invalidates every handle supported by that
root. Another Shared Home may keep the identity alive, but no handle is
silently re-rooted to it. A registered cleanup that will later read the root or
one of those handles makes an earlier `release root` invalid in the first
profile. `release` has no Result channel; a terminal hook Fault follows the
ordinary terminal-Fault chronology. Recoverable shutdown remains an ordinary
`close()`/`shutdown()` method.

## Sole terminal authority

Every owner-ending edge releases one Home token. Only the terminal transition
may dispatch the user hook:

```text
DestroyHome / DestroyOwned
-> determine or win terminal Home transition
-> Alive -> Finalizing
-> close weak upgrade and new ordinary leases
-> create one non-escapable FinalizerLease
-> run the Box fini hook at most once
-> release stored fields in reverse declaration order
-> structurally drop the native payload
-> publish weak-only tombstone or reclaim storage
```

The compiler and runtime do not independently call the hook. StaticUnique may
prove the terminal transition and materialize it directly. Shared consumes one
token per release and lets one ObjectCell/runtime winner perform the same
transition. Runtime refcount observations never invent source ownership.

## Home release matrix

| Edge | Home effect | User hook |
| --- | --- | --- |
| owning local scope end | release one Home | only if terminal |
| ordinary handle scope end | no owner effect | never |
| `take` / Home-demand call | forward one Home atomically | never during transfer |
| terminal `return` | forward one Home to result carrier | never during transfer |
| explicit `share` | add one independent Home | never on acquisition |
| explicit `release root` | release one verified root Home now | only if terminal |
| owning field replacement | commit new Home, then release old | old identity only if terminal |
| parent teardown | release fields in reverse declaration order | child only if child becomes terminal |
| weak-token drop | drop weak token | never for target |
| failed outer `birth` | roll back initialized fields | outer hook never; complete child may finalize if terminal |

Ordinary aliases do not participate in owner counts. A parent disappearing
does not guarantee a child hook: another Shared Home may keep the child alive.
The deterministic parent action is the field-Home release order, not a global
promise about when every child identity becomes terminal.

## Parent and field ordering

For a fully constructed parent:

```text
parent fini hook
-> fields released in reverse declaration order
-> parent native payload structural drop
```

The hook runs while fields are still usable. It may invoke ordinary
best-effort domain methods in an explicit order, but it may not directly call
a child `fini`. Canonical v1 does not add a source `release field` spelling;
physical field-Home release order remains fixed. A future reorder facility
requires a separate verified-Home Decision.

Field replacement is one transaction:

```text
evaluate RHS exactly once
-> verify type/Home relation and destination commit
-> install the new Home once
-> release the old Home once
-> run the old identity's hook only if that release is terminal
```

RHS or preflight failure leaves the old field unchanged. Hidden `share`,
release-before-commit, and same-identity double finalization are forbidden.

## Hook restrictions and Faults

`fini { ... }` has no parameters or result channel. Its body rejects:

```text
return / break / continue
?
await / yield / suspension
share me
return/store/capture/escape of me or FinalizerLease
resurrection and re-entry
direct lifecycle invocation
Home-demand/consume calls from the hook in the first profile
```

A hook Fault does not roll finalization back. The first Fault in time remains
primary; remaining field/native release continues best effort and later
teardown Faults become suppressed diagnostics. A recoverable external-resource
error belongs in an ordinary `close(): Result<...>` method while a caller still
exists.

## Unique, Shared, weak, and cycles

The physical strategy is selected from verified semantic facts:

```text
proved StaticUnique = no RC/control-cell/global-finalizer work
same-thread Shared = local owner count when selected
cross-thread Shared = separate capability/atomic/affinity Decision
weak-capable identity = generation/tombstone-capable control cell
```

`share` is the only ordinary source operation that adds an owner. The language
does not become RC for ordinary handles or Unique transfers.

A strong Shared cycle may prevent terminal Home finalization. Canonical v1
requires `weak` to break ownership back-edges; it does not promise a tracing
collector or user-hook execution from a future cycle collector. Thread-affine
resources must remain Unique/same-thread or fail before effects until a typed
finalizer-affinity contract exists.

## Current implementation boundary

```text
Home production activation = 0
VerifiedTerminalHomeDropPlan production consumer = 0
Box fini hook grammar = 0
Shared last-Home C′ winner = 0
```

Current Arc drops, `InstanceBox::fini`, global finalizer tracking, plugin Drop
routes, `DestroyOwned`, and B′ documents are migration evidence. None is a C′
terminal-Home authority. This Decision changes no production behavior and does
not move the current MirBuilder row.

## Task family

The parked Home taskboard owns the exact dependencies and implementation
cells:

```text
OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0   # this accepted Decision
-> OWN-EXPLICIT-HOME-RELEASE-STMT0-D0     # contextual-statement amendment
-> OWN-GRAM-RELEASE0                      # production-zero contextual statement carrier
-> OWN-GRAM-FINI-HOOK0                    # production-zero source carrier
-> OWN-FINI-HOOK-PLAN0-S0                 # passive non-callable verifier plan
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0        # sole whole-object DropPlan
-> OWN-EXPLICIT-HOME-RELEASE0-S0          # passive root/Flow/terminal plan
-> OWN-HOME-CLOSED-CALL0-I0 + OWN-HOME-STORAGE0-I0/L
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/U
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/U  # Unique local terminal release
-> OWN-EXPLICIT-HOME-RELEASE0-I0/U        # one exact Unique whole-root release
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST   # first-slice reference receipt
-> OWN-HOME-STORAGE0-I0/F
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/F
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/F  # field replacement/order/birth
-> OWN-HOME-SHARE0-I0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/S
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S  # Shared/weak terminal winner
-> OWN-EXPLICIT-HOME-RELEASE0-I0/S        # Shared non-last/terminal release parity
-> OWN-HOME-C-SPEED0-G0
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-R0
-> OWNERSHIP-HOME-PRODUCT-READINESS-D0
-> OWNERSHIP-HOME-CUTOVER0-I0-R0
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FINAL
```

No physical implementation cell starts before the relevant Home taxonomy,
composite, representation, destination, transfer-failure, birth, ABI, Home
Flow, grammar carrier, and passive hook-plan products are sealed. Scope-fini
retirement precedes the Box-member carrier. Plugin/FFI lifecycle is a separate
bounded series.

`R0` removes direct `obj.fini()` callability, callable-catalog exposure, the
B′ Dead-with-live-Home state, manual parent-to-child fini cascades, global
finalizer authority, and any route where terminal structural drop bypasses a
declared hook. Compatibility migration may not enter canonical AST/MIR/runtime
as a fallback.

## Implementation-coupled reference updates and closeout

Every grammar, passive-plan, production, and retirement cell in this family
updates its exact live reference/support status and examples in the same
commit. A production-zero row must stay labelled production-zero. The
`/FIRST` and `/FINAL` receipts below audit synchronized truth; they are not
permission to defer documentation.

`OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST` runs after `I0/U` and records only
the exact first production profile. It proves grammar/parser/lifecycle-
descriptor parity, direct-`obj.fini()` rejection, exactly one Unique-local
terminal hook dispatch, ordinary-method status for `close`/`shutdown`, and no
B′ claim inside that first slice. When the Unique `release` cell lands, the
same implementation commit must also synchronize the exact reference surface;
the FIRST receipt proves contextual-statement parser parity, one resolved root,
one sealed release plan, whole-root/alias diagnostics, ordinary/generic release
wrapper Call count zero, `drop` alias count zero, and no generic/composite/
Shared claim. It must not claim field, Shared, or default-profile support.

`.../FINAL` runs after final cutover, repeats every `/FIRST` proof, and adds
parent-hook-before-reverse-field-release, zero Shared non-last hook dispatch,
exactly one Shared terminal-winner dispatch, final cutover/reference parity,
exact Shared `release` parity, and zero B′/`drop`-alias claims across all live
reference pages. Only `/FINAL` closes the parent DOC0 row. Both cells must
update at least:

```text
required receipt: LIFECYCLE-LAST-HOME-FINI-REFERENCE-CLOSEOUT0-DOC0
required receipt: OWN-EXPLICIT-HOME-RELEASE-REFERENCE-CLOSEOUT0-DOC0
```

```text
docs/reference/language/lifecycle.md
docs/reference/language/ownership.md
docs/reference/language/scope-exit-semantics.md
docs/reference/language/EBNF.md
grammar/language-v1-registry.toml
constructor/birth reference
docs/reference/boxes-system/memory-finalization.md
docs/reference/architecture/rust-to-hako-lifecycle-projection.md
delegate/interface callable reference
docs/reference/plugin-system/plugin_lifecycle.md
docs/reference/boxes-system/plugin_lifecycle.md
docs/reference/plugin-system/vm-plugin-integration.md
plugin/FFI lifecycle ABI reference
examples and migration guide
```

Evidence is evaluated against only the production slice available at that
cell. A future field or Shared witness cannot be credited to `/FIRST`, and a
docs-only target cannot satisfy either cell.

## Non-claims

- no parser, runtime, ownership, backend, plugin, or FFI implementation;
- no tracing GC or cycle-finalization guarantee;
- no cross-thread finalizer policy;
- no user-selectable field-release order in v1;
- no automatic finalization claim for ordinary handles;
- no field/projection/container/composite or unknown-generic `release` in v1;
- no ordinary/generic wrapper Call as Home-release authority;
- no `drop root` / `drop(value)` compatibility alias or source grammar production;
- no current-lane change.
