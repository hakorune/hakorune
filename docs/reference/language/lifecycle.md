# Box Lifecycle and Finalization (SSOT)

Status: Normative C′ target; production activation 0.

Decision: `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0` accepted on 2026-08-05.
C′ explicitly supersedes B′ eager `obj.fini()` / Dead-with-live-Home
semantics.

Implementation status: transitional. Current `InstanceBox`, Arc/Drop paths,
global finalizer tracking, plugin finalization, `DestroyOwned`, parsers, and
backends do not yet implement the complete C′ contract. They are migration
evidence, not language authority.

Related authorities:

- source Home/handle/transfer/share: [ownership.md](ownership.md)
- scope exit and cleanup: [scope-exit-semantics.md](scope-exit-semantics.md)
- construction order: `docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md`
- C′ cross-layer design:
  `docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md`
- parked execution order:
  `docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md`
- superseded B′ history:
  `docs/development/current/main/design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md`

## Core rule

```text
birth(args) = new-only construction hook
fini { ... } = last-Home-only finalization hook
close()/shutdown()/commit()/abort() = ordinary domain methods
cleanup { ... } = lexical exit action
physical reclaim = runtime storage operation after payload teardown
```

`fini` is not an ordinary callable method. Canonical v1 rejects:

```hako
object.fini()

box Bad {
    fini() { ... }
}
```

The accepted hook spelling is:

```hako
box File {
    fini {
        me.closeBestEffort()
    }
}
```

The hook is optional. A Box without one still receives field/native structural
teardown when its last Home ends.

## Terms

- **Home**: an independently consumable lifetime-supporting owner token stored
  in a verified Home place.
- **ordinary handle**: a non-owning source capability supported by a Home; it
  does not participate in owner counting or finalization.
- **Shared Home**: one of several independent Homes supporting the same
  identity after explicit `share`.
- **weak token**: generation-aware non-owner that does not delay finalization.
- **terminal Home release**: the release that leaves no remaining Home for the
  identity.
- **FinalizerLease**: non-escapable privileged access used only while the C′
  DropPlan runs.
- **structural drop**: release of stored tokens/native payload and eventual
  storage reclaim. It is not a user-callable operation.

## Construction

Successful construction remains:

```text
new
-> declaration-site field initializers
-> birth(args)
-> optional explicit field overrides
-> publish usable identity and first Home
```

`birth` is the constructor hook invoked only by `new`; direct receiver `birth(...)` calls, including `obj.birth(...)`, are forbidden.

Object reuse is an ordinary domain operation. Methods named `reset`, `reactivate`, `configure`, `clear`, or `attach` may prepare an already-alive object according to its Box contract; they never re-run `birth` or bypass the terminal Home DropPlan.

If construction fails before publication:

1. do not run the incomplete outer Box `fini` hook;
2. release only already-initialized field Homes;
3. release them in reverse installation/declaration order;
4. a fully constructed child runs its own hook only if that release is the
   child's terminal Home release;
5. reclaim the unpublished outer storage without publishing a usable handle.

Exact partial-construction receipts remain gated by `OWN-HOME-BIRTH-D0`.

## C′ lifecycle states

The canonical source-visible lifecycle is:

```text
Constructing
-> Alive
-> Finalizing
-> PayloadDropped
```

A weak-only tombstone/control cell may remain after payload drop until the last
weak token disappears. That physical residue is not a source-usable Dead Box.

C′ removes B′'s ordinary state in which `obj.fini()` made a Box Dead while
strong/Home owners remained alive. Domain states such as Open/Closed,
Connected/Disconnected, or Committed/Aborted remain ordinary Box data and
method contracts.

## Sole terminal DropPlan

Every owner-ending edge releases exactly one Home. Only the terminal transition
may dispatch the hook:

```text
release one Home
-> determine/win terminal transition
-> prevent weak upgrade and new ordinary leases
-> drain already-issued leases required by the selected profile
-> create one FinalizerLease
-> run the Box fini hook at most once
-> release stored fields in reverse declaration order
-> structurally drop native payload
-> publish weak-only tombstone or reclaim storage
```

The compiler and runtime do not each call the hook. For StaticUnique, the
compiler may prove and directly materialize the terminal DropPlan. For Shared,
one runtime/ObjectCell zero-owner winner enters the same plan. A refcount value
never defines source ownership by itself.

If the hook Faults, finalization is not rolled back. Preserve the first Fault
in time, continue remaining field/native release best effort, record later
Faults as suppressed diagnostics, and publish the primary terminal Fault.

## Local, transfer, and Shared behavior

| Event | Owner effect | Hook behavior |
| --- | --- | --- |
| owning local scope end | release one Home | run only if terminal |
| ordinary handle scope end | none | never |
| `take` / Home-demand call | atomically forward one Home | never in transit |
| terminal return | forward Home to protected result carrier | never in transit |
| explicit `share` | add one independent Home | never on acquisition |
| Shared non-last release | remove one Home | never |
| Shared terminal release | remove last Home | exactly one winner runs plan |
| weak-token drop | weak bookkeeping only | never for target |

Ordinary aliases remain free of retain/release. Unique code does not become RC
merely because C′ has automatic finalization. Only an admitted Shared/weak
physical profile pays its selected bookkeeping cost.

## Fields and ordering

C′ applies only to fields classified as verified owning Home destinations by
`OWN-FIELD-CONTAINER-DEST-D0`. A current Box-typed field is not silently
reclassified merely from its spelling.

For a fully constructed parent:

```text
Parent.fini hook
-> release verified owning/weak field tokens in reverse declaration order
-> parent native payload structural drop
```

An owning child field release runs `Child.fini` only if it is the child's
terminal Home. A remaining Shared Home delays child finalization.

The parent hook may call ordinary best-effort domain methods in a chosen order
while fields remain usable:

```hako
fini {
    me.database.closeBestEffort()
    me.logger.closeBestEffort()
}
```

It may not call child `fini` directly or reorder the physical field-release
plan. Canonical v1 adds no `release field` spelling; a future reorder facility
requires a separate verified-Home Decision.

Owning field replacement is transactional:

```text
evaluate RHS once
-> verify type/Home and destination commit
-> install new Home once
-> release old Home once
-> run old identity hook only if terminal
```

RHS/preflight failure preserves the old field. Hidden `share`, early old-value
release, and same-identity double finalization are forbidden.

## `close()` versus `fini`

`close` is an ordinary method-name convention, not a keyword or reserved
callable role.

| Property | `close()` / domain method | `fini {}` |
| --- | --- | --- |
| invocation | explicit ordinary call | compiler/runtime terminal DropPlan |
| timing | while a caller exists | last Home release |
| result | may return `Result` | no result channel |
| post-state | Box remains alive in a domain state | payload teardown continues |
| repetition | Box-specific contract | hook exactly once |

Use an ordinary method when exact shutdown timing or a recoverable close error
matters. The later hook is a best-effort safety net and must safely observe an
already-closed state.

## Hook restrictions

A Box `fini {}` hook has no parameters and no return type. It rejects:

```text
return / break / continue
Result ?
await / yield / suspension
share me
return/store/capture/escape of me or FinalizerLease
resurrection
re-entry or direct lifecycle invocation
delegate/interface/alias exposure as an ordinary callable
```

Unknown backend, field/Home classification, Shared representation,
thread-affinity, plugin, or FFI behavior rejects before Builder effects. It
must not retry B′, SharedV1, or a native Drop fallback.

## Weak references, cycles, and GC

Weak upgrade is one generation/state/owner-acquisition transaction:

```text
Alive + admissible profile -> acquire one supported Home/handle relation
Finalizing/PayloadDropped/generation mismatch -> Option::None
invalid receiver/type -> Fault
```

A strong Shared cycle can prevent terminal Home release and therefore prevent
`fini`. Canonical v1 uses `weak` to break ownership back-edges. It does not
promise a tracing collector or user-hook execution by a future cycle
collector.

Base C′ therefore needs no heap tracing, root scanning, GC safepoint,
stop-the-world finalizer pass, or global finalizer registry. A future optional
collector may reclaim otherwise unreachable storage only under a separate
Decision and must not invent terminal user-hook semantics.

Cross-thread finalization is not accepted merely because Shared exists.
Thread-affine resources stay Unique/same-thread or reject until an exact
atomic-winner and finalizer-affinity contract lands.

## Performance contract

The target physical tiers are selected from verified facts:

```text
StaticUnique -> no RC/control-cell/global-finalizer work
same-thread Shared -> selected non-atomic owner bookkeeping
cross-thread Shared -> future atomic/affinity profile
weak-capable identity -> generation/tombstone-capable control cell
```

No C-speed claim is valid before `OWN-HOME-C-SPEED0-G0` measures the exact
front and assembly. In particular, the implementation must not make every Box
an `Arc`, retain/release ordinary handles, allocate runtime cleanup lists, or
dispatch QMark through dynamic methods.

## Implementation and reference closeout

Current production authority does not satisfy this page. The parked sequence
is owned by the Home taskboard:

```text
OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0
-> OWN-GRAM-FINI-HOOK0
-> OWN-FINI-HOOK-PLAN0-S0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/U
-> I0/U Unique local
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST
-> OWN-HOME-STORAGE0-I0/F
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/F
-> I0/F owning field and birth rollback
-> OWN-HOME-SHARE0-I0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/S
-> I0/S Shared/weak terminal winner
-> OWN-HOME-C-SPEED0-G0
-> R0 competing-authority retirement
-> PRODUCT-READINESS -> CUTOVER
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FINAL
```

After the first production C′ slice and again after final cutover,
`LIFECYCLE-LAST-HOME-FINI-REFERENCE-CLOSEOUT0-DOC0` is a mandatory receipt of
the Home reference closeout. It must synchronize EBNF/registry, both parsers,
lifecycle descriptor, VM/EXE/AOT or exact unsupported-backend rejection,
ownership/scope/birth/memory/plugin/FFI references, examples, and migration
guides. Direct `obj.fini()` callers and all live B′ claims must be zero before
final completion.
