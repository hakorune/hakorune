---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: current allocator provider / replacement hook task breakdown after M70.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
---

# Allocator Provider Current Task Breakdown (SSOT)

## Goal

Make the current allocator replacement/provider lane readable at task granularity.

The current lane is not "replace malloc now". It is:

```text
make allocator policy/state/proof visible to Hakorune
then add provider diagnostics
then prove activation safety
then and only then consider process allocator replacement
```

## Current Completed Checkpoint

| Range | Result | Status |
| --- | --- | --- |
| M52-M56 | allocator replacement hook boundary, reserved hook plan/proof vocabulary, runtime owner named | complete |
| M57-M61 | diagnostic-only dry-run runtime validator, manifest text callsite, test surface, proof validator, CLI surface | complete |
| M62-M63 | activation preflight boundary and diagnostic preflight facts/report | complete |
| M64-M70 | provider ids, reserved provider manifest fixture, diagnostic parser, explicit CLI surface, readiness preflight facts, and combined hook/provider dry-run report | complete |

## Layer Model

```text
language lifecycle:
  cleanup / fini / ownership / keepalive / weak / GC trigger

hako_alloc policy/state:
  size class / page policy / free-list / reuse / stats / stress proof

provider diagnostics:
  provider manifest / provider readiness / preflight facts / stable diagnostics

native metal provider:
  system allocator / mimalloc / OS VM glue / platform atomics and TLS
```

## Immediate Task Ladder

| Row | Task | Output | Must Not Add |
| --- | --- | --- | --- |
| M66 | provider task breakdown | this SSOT + guard | runtime code |
| M67 | provider manifest diagnostic parser | caller-provided TOML text parser/report | file discovery, provider selection |
| M68 | provider manifest CLI diagnostic surface | explicit `--allocator-provider-manifest` dry-run output | env toggles, implicit discovery |
| M69 | provider readiness preflight shape | diagnostic facts tying provider readiness to activation preflight | activation |
| M70 | combined hook/provider dry-run report | plan + proof + provider manifest report | process allocator replacement |
| M71 | provider registry boundary docs | registry ownership and future API shape | active registry implementation |
| M72 | hako model provider proof fixture | model-provider validation smoke or fixture | native pointer/metal activation |
| M73 | debug guarded provider proof fixture | guarded-provider diagnostic proof | process allocator replacement |
| M74 | native system provider proof boundary | system provider contract docs/fixture | `#[global_allocator]` |
| M75 | native mimalloc provider proof boundary | mimalloc provider contract docs/fixture | production activation |

## Dependency Order

```text
M66 task breakdown
  -> M67 provider manifest parser
  -> M68 provider CLI diagnostic
  -> M69 provider readiness preflight
  -> M70 combined hook/provider dry-run
  -> M71 registry boundary
  -> M72/M73 debug/model provider proofs
  -> M74/M75 native provider proof boundaries
  -> later activation row only after safety proof
```

## Stop Line

Until a later activation row explicitly changes this, all current tasks keep
these inactive:

- runtime provider registry;
- provider selection;
- provider environment toggles;
- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- implicit runtime file-system manifest discovery;
- `.inc` hook/provider/facade/policy name matching;
- route widening for allocator activation;
- native-pointer attrs inferred from handle ownership.

## Guard Hygiene

Past card guards should not pin `CURRENT_STATE.latest_card`. A past guard should
prove that its own docs/code/fixtures are still present and that forbidden
activation behavior has not leaked in. Only the current card guard may require
the latest-card pointer.

## Acceptance Pattern

Every next row should land as:

1. SSOT or implementation doc first.
2. Small runtime/CLI code only when the row explicitly allows it.
3. Dedicated guard.
4. `current_state_pointer_guard`.
5. `git diff --check`.

## Next Step

M71 may add provider registry boundary docs that name ownership and future API
shape. It must not add active registry implementation, provider selection,
implicit manifest discovery, or allocator replacement.
