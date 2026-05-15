---
Status: SSOT
Date: 2026-05-15
Scope: source-level object construction lifecycle: `new`, `birth`, field initializers, explicit reuse methods, factories, and `fini`.
Related:
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-object-lifecycle-queue-ssot.md
---

# Constructor Birth / New Lifecycle SSOT

Decision: accepted.

Hakorune keeps construction small and explicit:

```text
birth:
  constructor hook
  direct source call forbidden
  fires only through new

new:
  canonical construction surface
  positional args now
  named args later

reuse:
  explicit lifecycle methods such as reset / reactivate / configure / clear / attach

field initializer:
  per-instance
  runs before birth

fini:
  object usable-lifetime exit / cleanup
```

## Canonical construction

Canonical source:

```hako
local page = new HakoAllocPageModel(PageId(0), Bytes(32), 2, 2)
```

The construction order is:

```text
allocate object identity
run declaration-site field initializers
run matching birth(args...)
publish the object as usable
```

`birth` is special because it initializes a fresh identity. It is not hidden
magic: it is a declared hook with normal parameters and body rules. The
special rule is only its call permission.

## Direct `birth` calls are forbidden

Forbidden source:

```hako
page.birth(PageId(0), Bytes(32), 2, 2)
```

Reason:

```text
Direct birth calls would let user code reinitialize an existing object identity.
That makes lifecycle state ambiguous and weakens verifier / allocator reasoning.
```

Parser diagnostics should point users at the canonical surface:

```text
birth is a constructor hook; use new HakoAllocPageModel(...)
```

Existing internal or legacy `birth` routes are compatibility residue unless a
specific row marks them as part of the canonical language. They must not be used
as permission to add source-level `page.birth(...)`.

## Field initializers

Stored field initializers are per-construction values.

```hako
box Counter {
    count: i64 = 0

    birth(start: i64) {
        me.count = start
    }
}
```

For each `new Counter(...)`, field initializers run before `birth(...)`.

Rules:

```text
field initializers:
  create the initial per-instance state
  do not replace birth parameters
  must not be shared mutable state between instances

birth:
  may override initialized fields
  owns fresh-object initialization only
  is not a reuse/reset surface
```

## Reuse is explicit

Object reuse must use ordinary, named lifecycle methods.

```hako
page.reactivate()
page.resetForReuse(Bytes(64), 4)
page.configure(policy)
page.clear()
page.attach(owner)
```

These methods are normal public methods. They should express lifecycle rules
with contracts and transitions when available:

```text
requires:
  pre-state and input validity

ensures:
  post-state and observer facts

transition:
  allowed state movement
```

Do not reuse `birth` for reset/reactivation. This keeps construction,
reconfiguration, and cleanup separate.

### Current allocator reuse inventory

Current `hako_alloc` reuse surface is explicit ordinary method surface:

| Method surface | Owner file | Role |
| --- | --- | --- |
| `HakoAllocPageModel.reactivate()` | `lang/src/hako_alloc/memory/page_box.hako` | Move an empty, committed page back to active reusable state. |
| `HakoAllocPageModel.reuse()` | `lang/src/hako_alloc/memory/page_box.hako` | Guarded wrapper over `canReuse()` and `reactivate()`. |
| `HakoAllocObjectLifecycle*Result.reset()` | `lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako` | Clear result capsule observer state before a new facade operation. |
| `HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage(...)` | `lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako` | Attach a newly sourced page to the object-lifecycle facade. |

These methods are normal public methods. Future `configure`, `clear`, or
`attach*` methods are allowed only as explicit lifecycle methods with their own
contracts / transitions or row guard. They must not be implemented as direct
receiver `birth(...)` calls.

Compatibility exception:

```text
lang/src/hako_alloc/memory/arc_box.hako: arc.birth(ptr)
```

This remains a legacy non-constructor host facade exception. It is not
permission to add source-level receiver `birth(...)` lifecycle reuse.

## Factories

Named construction variants belong in factory methods or factory boxes, not in
extra constructor keywords.

Example shape:

```hako
box HakoAllocPageFactory {
    makeSmall(page_id: PageId): HakoAllocPageModel {
        return new HakoAllocPageModel(page_id, Bytes(32), 2, 2)
    }
}
```

Factories may choose constructor arguments and policies. They do not weaken the
`birth` direct-call ban.

## Named arguments are later

This is readable but not part of the current MVP:

```hako
local page = new HakoAllocPageModel(
    page_id: PageId(0),
    block_size: Bytes(32),
    capacity: 2,
    reserved: 2
)
```

Named constructor arguments require a separate row because they affect parser
surface, diagnostics, argument binding, and metadata transport.

Current MVP:

```text
new Box(positional_args...)
```

Later row:

```text
new Box(named_args...)
```

## `fini` boundary

`fini()` is the usable-lifetime exit / cleanup hook. It is not the inverse of
`birth` in the sense of physical memory deallocation.

Relationship:

```text
new -> field initializers -> birth -> usable methods -> fini
```

`fini()` must remain idempotent and fail-fast after logical finalization, as
defined by `docs/reference/language/lifecycle.md`.

## Stage ownership

```text
Stage0:
  parse birth declarations
  parse new expressions
  reject or diagnose direct source birth calls
  transport constructor metadata
  no lifecycle checker

Stage1:
  constructor resolution
  field initializer ordering facts
  verifier-visible lifecycle facts
  explicit reuse method contracts
  direct-birth negative diagnostics

LLVM/EXE:
  primary acceptance for object-heavy allocator routes

VM:
  semantic reference / scalar smoke only
```

## Task rows

The active task placement is the phase-293x mimalloc taskboard.

Immediate rows:

```text
LIFECYCLE-BIRTH-001:
  document and enforce new-only birth policy

PARSER-BIRTH-001:
  add negative source fixture for page.birth(...)

PARSER-BIRTH-002:
  improve parser diagnostic with new Box(...) hint

NEW-NAMED-ARGS-001:
  parked; design named constructor args later

REUSE-LIFECYCLE-001:
  keep allocator reuse as explicit methods with contracts/transitions
```

Stop line:

```text
Do not accept source-level receiver.birth(...) as a quick fix for constructor
or lifecycle routing failures.
```
