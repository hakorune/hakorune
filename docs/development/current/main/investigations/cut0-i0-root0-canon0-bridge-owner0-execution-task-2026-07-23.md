# CUT0-I0 ROOT0-CANON0 CANON-BRIDGE0 OWNER0 実行タスク

Status: **Active — physical compiler-owned bridge only**

Related:

- `cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md`
- `cut0-i0-root0-canon0-source-binding-execution-task-2026-07-22.md`
- `cut0-i0-root0-canon0-lower0-execution-task-2026-07-22.md`
- `CURRENT_STATE.toml`

## Objective

Consume one `SourceBoundCanonicalPackageV1` by value at a private
`MirCompiler` terminal and open the real physical invocation before lowering:

```text
source-bound package
-> one ModuleBuilderInvocationSessionV1
-> one branded shell + canonical collector
-> same-session draft-only lowering
-> unpublished lowered owner
```

This row proves the compiler package and Builder physical owner are one chain.
It does not activate canonical production ingress or any publication path.

## Decision lock

Candidate CB-prime OWNER0 is selected:

- the bridge is compiler-owned and one-shot;
- the shared non-Clone token moves unchanged through the phase boundary;
- the physical session, shell, and collector are opened before plan
  consumption;
- `CanonicalModuleLoweringSessionV1` is not used by the new bridge;
- failure returns the unpublished package/owner, leaves the live Builder
  unchanged, and permits no retry or fallback.

## Required implementation

1. Add one private `MirCompiler::begin_canonical_invocation` terminal consuming
   `SourceBoundCanonicalPackageV1` by value.
2. Construct the actual `ModuleBuilderInvocationSessionV1` with the explicit
   canonical Builder config and the shared token brand.
3. Construct one branded shell and one typed canonical collector from that
   same physical owner. No loose token, family, header, catalog, or receipt
   argument is accepted.
4. Move the exact route plan into the existing draft-only LOWER0 seam inside
   that same candidate Builder session.
5. Return a typed rejected owner on physical-open or lowering failure; no
   live-Builder mutation, sibling continuation, retry, fallback, drain,
   finalizer, or external commit is allowed.

## Acceptance

```text
SourceBoundCanonicalPackageV1 consumer = 1
physical bridge constructor = 1 (MirCompiler-owned)
ModuleBuilderInvocationSessionV1 new owner = 1
CanonicalModuleLoweringSessionV1 new-bridge callers = 0
shared token/brand preserved without conversion = 1
shell and collector opened before lowering = 1
draft-only plan consumer = 1 per canonical route
Option<Plan>/take().expect on new path = 0
live Builder pre-write on failure = 0
retry/fallback/publication/drain/finalizer consumers = 0
all touched source/check files < 800 lines
```

## Required evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q canonical_source_binding_owner0 --lib
python3 tools/checks/lib/cut0_i0_root0_canon0_bridge_guard.py
```

## Stop line

Do not add typed canonical collection, receipt retention, recursive capability
changes, aggregate fixtures, DRAIN0, finalization, external commit, or atomic
CUT0 wiring in this row. Those remain separate rows after OWNER0 closes.
