# 293x-361 MIMAP-015B Facade Release Fail-Fast

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-015B` is the next primary allocator row after `MIMAP-015A`. It adds
double-release and stale-release fail-fast proof over the facade release route
without adding page-map lookup or realloc behavior.

## Scope

- Keep release routed through `HakoAllocObjectLifecycleFacade`.
- Prove double-release rejection for the same known `(page id, block id)`.
- Prove missing/stale page id rejection through scalar reason observers.
- Keep using existing `HakoAllocPageModel.releaseLocal(block_id)` for
  page-local validation.
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No realloc route.
- No alignment route.
- No arbitrary pointer-to-page lookup.
- No page-map ownership lookup.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No selected-object return through the facade.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-release-failfast-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-release-failfast-proof
double=<status>,<reason code>
stale=<status>,<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap015b-mir-json] ok
[k2-wide-mimalloc-facade-release-failfast-exe] ok
```

## Stop Lines

If fail-fast needs arbitrary pointer-to-page lookup, stop and split a page-map
handoff row.

If fail-fast needs release history beyond page-local live-bit validation, stop
and add a named release-history row instead of mixing it into `MIMAP-015B`.

If selected-object return becomes necessary, stop this row and land `MIR-ROW-C`
instead of broadening `MIMAP-015B`.

## Follow-up

After `MIMAP-015B` lands:

```text
MIMAP-016A:
  alignment request metadata and observer result
```
