---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: current allocator provider / replacement hook task breakdown after M75.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - docs/development/current/main/design/allocator-provider-hako-model-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-debug-guarded-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-system-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-mimalloc-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-owner-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-closeout-inventory-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-surface-proposal-ssot.md
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
| M64-M75 | provider ids, reserved provider manifest fixture, diagnostic parser, explicit CLI surface, readiness preflight facts, combined hook/provider dry-run report, registry boundary docs, hako model provider proof fixture, debug guarded provider proof fixture, native system provider proof boundary, and native mimalloc provider proof boundary | complete |
| M76 | activation entry contract naming registry/selection ownership, proof consumption, fail-fast diagnostics, rollback behavior, and the next activation task ladder | complete |
| M77 | registry snapshot diagnostic shape fixing reserved provider entries and registry snapshot facts | complete |
| M78 | selection decision diagnostic shape fixing caller-provided request facts and no-selected-provider diagnostics | complete |
| M79 | provider proof bundle consumption diagnostic shape fixing proof bundle inputs and missing-proof diagnostics | complete |
| M80 | rollback preflight diagnostic shape fixing rollback target facts and activation-blocked diagnostics | complete |
| M81 | activation safety gate diagnostic shape fixing evidence bundle facts and gate-closed diagnostics | complete |
| M82 | activation safety diagnostic owner fixing runtime ownership and future-compatible past guards | complete |
| M83 | activation safety diagnostic report fixing runtime report output while keeping the gate closed | complete |
| M84 | activation safety diagnostic CLI surface fixing explicit CLI output while keeping the gate closed | complete |
| M85 | activation safety closeout inventory fixing coverage across M76-M84 without activation | complete |
| M86 | activation decision surface proposal fixing the future explicit-input decision contract without implementation | complete |

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

activation entry:
  registry snapshot / selection decision / proof consumption / rollback preflight / safety gate
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
| M76 | activation entry contract | reserved activation entry fixture naming ownership/proof/rollback facts | runtime registry code |
| M77 | registry snapshot diagnostic shape | reserved registry snapshot fixture with provider entries | provider selection |
| M78 | selection decision diagnostic shape | reserved selection request/decision fixture with no selected provider | activation |
| M79 | provider proof bundle consumption | reserved provider proof bundle fixture with selected-provider proof inputs | `#[global_allocator]` |
| M80 | rollback preflight contract | reserved rollback preflight fixture with rollback target facts | process allocator replacement |
| M81 | activation safety gate contract | reserved activation evidence bundle fixture with gate-closed facts | hook activation |
| M82 | activation safety diagnostic owner | runtime owner SSOT plus future-compatible provider guards | activation safety implementation |
| M83 | activation safety diagnostic report | runtime report over caller-provided safety TOML text | activation gate opening |
| M84 | activation safety diagnostic CLI surface | explicit CLI over caller-provided safety TOML path | environment discovery |
| M85 | activation safety closeout inventory | coverage guard for M76-M84 artifacts | runtime activation |
| M86 | activation decision surface proposal | docs-first explicit decision input/output contract | runtime/CLI implementation |

## Post-M75 Activation Entry Ladder

| Row | Task | Output | Must Not Add |
| --- | --- | --- | --- |
| M76 | activation entry contract | SSOT + reserved fixture + guard | runtime registry code |
| M77 | registry snapshot diagnostic shape | explicit registry snapshot data shape | provider selection |
| M78 | selection decision diagnostic shape | deterministic selection request/decision facts | activation |
| M79 | provider proof bundle consumption | explicit provider proof validation handoff | `#[global_allocator]` |
| M80 | rollback preflight contract | rollback facts before activation | process allocator replacement |
| M81 | activation safety gate contract | gate-closed activation evidence bundle | hook activation |
| M82 | activation safety diagnostic owner | names the runtime diagnostic owner and cleans guard pins | activation safety implementation |
| M83 | activation safety diagnostic report | diagnostic-only runtime report with gate-closed output | activation gate opening |
| M84 | activation safety diagnostic CLI surface | explicit CLI output for the gate-closed report | environment discovery |
| M85 | activation safety closeout inventory | inventory guard for the complete diagnostic ladder | runtime activation |
| M86 | activation decision surface proposal | proposal-only activation decision surface contract | runtime/CLI implementation |

## Dependency Order

```text
M66 task breakdown
  -> M67 provider manifest parser
  -> M68 provider CLI diagnostic
  -> M69 provider readiness preflight
  -> M70 combined hook/provider dry-run
  -> M71 registry boundary
  -> M72 hako model provider proof
  -> M73 debug guarded provider proof
  -> M74/M75 native provider proof boundaries
  -> M76 activation entry contract
  -> M77 registry snapshot diagnostic shape
  -> M78 selection decision diagnostic shape
  -> M79 provider proof bundle consumption
  -> M80 rollback preflight contract
  -> M81 activation safety gate contract
  -> M82 activation safety diagnostic owner
  -> M83 activation safety diagnostic report
  -> M84 activation safety diagnostic CLI surface
  -> M85 activation safety closeout inventory
  -> M86 activation decision surface proposal
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

Past provider guards must also avoid pinning future diagnostic owner files or
diagnostic type names as absent. They may keep blocking active selection,
environment toggles, hook activation, and process allocator replacement.

## Acceptance Pattern

Every next row should land as:

1. SSOT or implementation doc first.
2. Small runtime/CLI code only when the row explicitly allows it.
3. Dedicated guard.
4. `current_state_pointer_guard`.
5. `git diff --check`.

## Next Step

Provider proof boundary ladder is now closed through M75. M76 opens the
activation entry contract, M77 fixes the reserved registry snapshot shape, and
M78 fixes the reserved selection request/decision shape without runtime
registry code or provider selection. M79 fixes the reserved provider proof
bundle consumption shape without runtime proof consumption or activation. M80
fixes the reserved rollback preflight shape without rollback preparation, hook
activation, or process replacement. M81 fixes the reserved activation safety
gate shape without opening the gate or activating hooks. M82 fixes the
activation safety diagnostic owner and removes stale past-guard pins against
future diagnostic owner files/type names. M83 adds the diagnostic-only runtime
report in that owner while keeping the gate closed. M84 exposes that report
through an explicit-input CLI surface without opening the gate. M85 closes out
the activation safety diagnostic ladder with an inventory guard and no runtime
activation. M86 defines the future explicit-input activation decision surface
as a proposal-only contract without runtime or CLI implementation. The next
safe row is M87 activation decision fixture contract. It must be docs/fixture
only and must not silently enable production activation, `#[global_allocator]`,
process allocator replacement, environment discovery, provider selection,
runtime proof consumption, rollback preparation, hook activation, or `.inc`
name matching.
