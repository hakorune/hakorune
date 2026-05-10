---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M45 production allocator port entry plan
---

# 293x-097 M45 Production Allocator Port Entry Plan

## Decision

`M45 production allocator port entry plan` is live-narrow.

M45 is the entry boundary for moving from substrate proof apps to production
allocator port work. It adds no app fixture, source syntax, MIR route row,
runtime export, `.inc` emitter, pointer `fetch_add`, native pointer attrs, or
allocator replacement hook.

The purpose of M45 is to prevent the production port from becoming one broad
"allocator wave". The port starts only after the M20-M44 proof ladder is
locked, and proceeds through small seams.

## Production Port Meaning

In this phase, "production allocator port" means:

```text
hako_alloc owns allocator policy/control shape.
runtime/substrate owns raw capability facades.
native metal keep owns final libc/syscall/platform bodies.
```

It does not mean:

```text
.hako replaces the process allocator immediately.
.hako receives unrestricted raw pointer syntax.
.inc starts matching allocator app names.
native pointer attrs become active by implication.
pointer fetch_add becomes active by implication.
```

## First Implementation Order

The first production port work must follow this order:

1. `M46 hako_alloc production facade boundary`
   - create the production-facing allocator facade contract under
     `hako_alloc`;
   - route through existing substrate boxes only;
   - no backend allocator replacement hook.
2. `M47 allocator local page policy proof`
   - prove local allocate/free policy over the facade;
   - use existing size-class/page/free-list vocabulary;
   - no remote-free and no OS VM ownership widening.
3. `M48 allocator remote-free policy proof`
   - compose the M43 retry-loop shape behind the production facade;
   - keep remote-free policy in `.hako`, pointer atomics in substrate.
4. `M49 allocator OSVM page-source proof`
   - compose page reserve/commit/decommit rows as a page-source seam;
   - no unreserve API unless a new row is named.
5. `M50 allocator stress production-facade parity`
   - repoint a stress app to the production facade;
   - keep existing allocator-stress as regression coverage.

This order may be split further, but it must not be collapsed into a single
allocator implementation commit.

## Required Boundaries

- `lang/src/hako_alloc/**`
  - owns allocator policy/state/facade names.
  - must not own direct ABI symbols or platform calls.
- `lang/src/runtime/substrate/**`
  - owns capability facades such as mem/raw_buf/osvm/tls/atomic.
  - must not own allocator policy.
- `lang/c-abi/shims/**`
  - consumes MIR-owned route facts only.
  - must not branch on allocator app names, facade names, or policy class names.
- Rust runtime/native code
  - keeps final metal bodies.
  - must not become a second policy owner.

## Stop Line

M45 explicitly keeps these inactive:

- native pointer `fetch_add`;
- noalias / nonnull / dereferenceable export widening;
- unrestricted pointer arithmetic;
- native layout / `repr(C)` allocator metadata;
- backend allocator replacement;
- OS VM unreserve/release row;
- hidden environment variable toggles.

Any future card that needs one of these must name a new row, fixture, guard, and
fail-fast diagnostic.

## Gate

```bash
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Result on 2026-05-10:
`k2_wide_production_allocator_port_entry_plan_guard.sh` passes.
