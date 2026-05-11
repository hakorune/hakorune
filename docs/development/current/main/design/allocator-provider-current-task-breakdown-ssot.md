---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: current allocator provider / replacement hook task breakdown after M101.
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
  - docs/development/current/main/design/allocator-provider-activation-decision-v0.toml
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-owner-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-decision-closeout-inventory-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-implementation-entry-contract-v0.toml
  - docs/development/current/main/design/allocator-provider-registry-snapshot-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-diagnostic-inactive-actions-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-diagnostic-closeout-inventory-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-diagnostic-helper-cleanup-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-diagnostic-report-ssot.md
  - docs/development/current/main/design/allocator-provider-runtime-diagnostic-module-boundaries-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-cli-surface-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-consumption-failfast-entry-ssot.md
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
  - docs/development/current/main/design/allocator-provider-selected-provider-precondition-ssot.md
  - docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
---

# Allocator Provider Current Task Breakdown (SSOT)

## Goal

Make the allocator replacement/provider lane readable at task granularity.

This is an optional future host-replacement support lane. It is not the primary
implementation plan for the current mimalloc port.

The current lane is not "replace malloc now". It is:

```text
make allocator policy/state/proof visible to Hakorune
then add provider diagnostics
then prove activation safety
then and only then, if explicitly reopened, consider process allocator replacement
```

The default current mimalloc implementation direction is fixed by
`mimalloc-hako-port-purpose-ssot.md`: implement allocator logic in `.hako` /
`hako_alloc`, backed by narrow capability substrate rows. M104+ remains a
future optional host allocator replacement ladder and does not gate `.hako`
mimalloc progress.

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
| M87 | activation decision fixture contract fixing the reserved caller-provided decision bundle | complete |
| M88 | activation decision diagnostic owner fixing runtime owner naming without implementation activation | complete |
| M89 | activation decision diagnostic report fixing runtime parsing/reporting with all activation booleans false | complete |
| M90 | activation decision diagnostic CLI surface fixing explicit CLI output with no activation | complete |
| M91 | activation decision closeout inventory fixing coverage across M86-M90 without activation | complete |
| M92 | activation implementation entry contract fixing the single future owner/entry while keeping activation inactive | complete |
| M93 | registry snapshot diagnostic report fixing runtime parsing/reporting while active registry construction stays inactive | complete |
| M93B | diagnostic inactive actions cleanup fixing one code-side source for false provider diagnostic outputs | complete |
| M94 | registry snapshot CLI surface fixing explicit TOML-path output while active registry construction stays inactive | complete |
| M95 | activation diagnostic closeout inventory fixing coverage across M92-M94/M93B without activation | complete |
| M96 | selection decision diagnostic report fixing runtime parsing/reporting while provider selection stays inactive | complete |
| M97 | selection decision CLI surface fixing explicit TOML-path output while provider selection stays inactive | complete |
| M97B | diagnostic helper cleanup fixing shared TOML helper and fact-check ownership without behavior change | complete |
| M98 | proof bundle consumption diagnostic report fixing runtime parsing/reporting while proof consumption stays inactive | complete |
| M98B | runtime diagnostic module boundaries fixing report-owner modules and keeping the registry facade under 1000 lines | complete |
| M99 | proof bundle consumption CLI surface fixing explicit TOML-path output while proof consumption stays inactive | complete |
| M100 | proof bundle consumption entry contract fixing the single future behavior owner/entry while proof consumption stays inactive | complete |
| M101 | proof consumption fail-fast entry fixing the reserved runtime entry while proof consumption stays inactive | complete |
| M102 | selected-provider precondition fixing caller-provided selected provider validation while proof consumption stays inactive | complete |
| M103 | selected-provider proof validation fixing operation coverage facts while proof consumption stays inactive | complete |

## Layer Model

```text
language lifecycle:
  cleanup / fini / ownership / keepalive / weak / GC trigger

hako_alloc policy/state:
  size class / page policy / free-list / reuse / stats / stress proof
  primary home for current mimalloc algorithm work

provider diagnostics:
  provider manifest / provider readiness / preflight facts / stable diagnostics

native metal provider:
  system allocator / mimalloc / OS VM glue / platform atomics and TLS

activation entry:
  registry snapshot / selection decision / proof consumption / rollback preflight / safety gate
  optional future host replacement support, inactive by default after M103
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
| M87 | activation decision fixture contract | reserved caller-provided activation decision fixture | runtime/CLI implementation |
| M88 | activation decision diagnostic owner | owner SSOT naming `src/runtime/allocator_provider_activation_decision.rs` | activation implementation |
| M89 | activation decision diagnostic report | diagnostic-only parser/report over caller-provided decision TOML | provider selection |
| M90 | activation decision diagnostic CLI surface | explicit CLI over caller-provided decision TOML path | activation |
| M91 | activation decision closeout inventory | coverage guard for M86-M90 artifacts | runtime activation |
| M92 | activation implementation entry contract | single future activation owner/entry fixture | runtime activation code |
| M93 | registry snapshot diagnostic report | diagnostic-only report over caller-provided registry snapshot TOML | active registry construction |
| M93B | diagnostic inactive actions cleanup | shared code-side false output source for M83/M89/M93 reports | behavior change |
| M94 | registry snapshot CLI surface | explicit CLI output for the inactive registry snapshot report | environment discovery |
| M95 | activation diagnostic closeout inventory | coverage guard for M92-M94/M93B artifacts | runtime activation |
| M96 | selection decision diagnostic report | runtime report over caller-provided selection decision TOML text | provider selection |
| M97 | selection decision CLI surface | explicit CLI output for the inactive selection decision report | activation |
| M98 | proof bundle consumption diagnostic report | runtime report over caller-provided proof-bundle consumption TOML text | proof consumption |
| M98B | runtime diagnostic module boundaries | focused runtime modules with registry facade API compatibility | behavior change |
| M99 | proof bundle consumption CLI surface | explicit CLI output for the inactive proof-bundle consumption report | proof consumption |
| M100 | proof bundle consumption entry contract | single future proof-consumption owner/entry fixture | runtime proof consumption |
| M101 | proof consumption fail-fast entry | runtime attempt report blocking on missing selected provider | actual proof consumption |
| M102 | selected-provider precondition | runtime attempt report validating caller-provided selected provider | provider selection, proof consumption |
| M103 | proof validation for selected provider | selected provider proof operation/capability validation | proof consumption token |
| M104 | proof bundle consumption token | optional future in-memory token after selected provider and proof validation pass | gate opening, hook install, replacement |
| M105 | rollback preparation fail-fast entry | rollback precondition report blocked without token/facts | rollback execution, gate opening, replacement |

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
| M87 | activation decision fixture contract | explicit decision bundle fixture contract | runtime/CLI implementation |
| M88 | activation decision diagnostic owner | runtime owner name for the diagnostic parser/report | activation implementation |
| M89 | activation decision diagnostic report | diagnostic-only report for complete/missing/malformed decision input | provider selection |
| M90 | activation decision diagnostic CLI surface | explicit CLI output for the blocked decision report | activation |
| M91 | activation decision closeout inventory | inventory guard for the complete decision diagnostic ladder | runtime activation |
| M92 | activation implementation entry contract | single future activation owner/entry fixture | runtime activation code |
| M93 | registry snapshot diagnostic report | runtime report for complete/missing/malformed registry snapshot input | active registry construction |
| M93B | diagnostic inactive actions cleanup | shared code-side false output source for M83/M89/M93 reports | behavior change |
| M94 | registry snapshot CLI surface | explicit CLI output for the inactive registry snapshot report | implicit discovery |
| M95 | activation diagnostic closeout inventory | coverage guard for M92-M94/M93B artifacts | runtime activation |
| M96 | selection decision diagnostic report | runtime report over caller-provided selection decision TOML text | provider selection |
| M97 | selection decision CLI surface | explicit CLI output for the inactive selection decision report | activation |
| M98 | proof bundle consumption diagnostic report | runtime report over caller-provided proof-bundle consumption TOML text | proof consumption |
| M98B | runtime diagnostic module boundaries | focused runtime modules with registry facade API compatibility | behavior change |
| M99 | proof bundle consumption CLI surface | explicit CLI output for the inactive proof-bundle consumption report | proof consumption |
| M100 | proof bundle consumption entry contract | single future proof-consumption owner/entry fixture | runtime proof consumption |
| M101 | proof consumption fail-fast entry | runtime attempt report blocking on missing selected provider | actual proof consumption |
| M102 | selected-provider precondition | runtime attempt report validating caller-provided selected provider | provider selection, proof consumption |
| M103 | proof validation for selected provider | selected provider proof operation/capability validation | proof consumption token |
| M104 | proof bundle consumption token | optional future in-memory token after selected provider and proof validation pass | gate opening, hook install, replacement |
| M105 | rollback preparation fail-fast entry | rollback precondition report blocked without token/facts | rollback execution, gate opening, replacement |

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
  -> M87 activation decision fixture contract
  -> M88 activation decision diagnostic owner
  -> M89 activation decision diagnostic report
  -> M90 activation decision diagnostic CLI surface
  -> M91 activation decision closeout inventory
  -> M92 activation implementation entry contract
  -> M93 registry snapshot diagnostic report
  -> M93B diagnostic inactive actions cleanup
  -> M94 registry snapshot CLI surface
  -> M95 activation diagnostic closeout inventory
  -> M96 selection decision diagnostic report
  -> M97 selection decision CLI surface
  -> M97B diagnostic helper cleanup
  -> M98 proof bundle consumption diagnostic report
  -> M98B runtime diagnostic module boundaries
  -> M99 proof bundle consumption CLI surface
  -> M100 proof bundle consumption entry contract
  -> M101 proof consumption fail-fast entry
  -> M102 selected-provider precondition
  -> M103 proof validation for selected provider
  -> M104 proof bundle consumption token (optional future host-replacement lane only)
  -> M105 rollback preparation fail-fast entry
  -> later activation behavior rows only after each owner/entry SSOT
```

## Owner Map

Diagnostic owner files may exist before the active runtime registry exists.
This prevents stop-line wording from blocking diagnostic-only rows:

- `src/runtime/allocator_provider_registry.rs` may own registry snapshot and
  activation safety diagnostics/reports. It is not an active provider registry
  or selection implementation.
- `src/runtime/allocator_provider_activation_decision.rs` may own activation
  decision diagnostics and reports. It does not select providers, consume
  proofs, prepare rollback, open the gate, install hooks, or replace the
  process allocator.
- `src/runtime/allocator_provider_activation.rs` is named by M92 as the future
  activation attempt owner. M92 does not require the file to exist and does not
  authorize activation behavior in diagnostic owner modules.
- "runtime provider registry implementation" in older cards means active
  registry/selection behavior, not diagnostic owner modules with all activation
  outputs fixed false.

## Stop Line

Until a later activation behavior row explicitly changes this, all current
tasks keep these inactive:

- active runtime provider registry or provider selection implementation;
- runtime activation behavior beyond M92 owner/entry naming;
- proof consumption implementation;
- rollback preparation or execution;
- activation gate opening;
- provider selection;
- provider environment toggles, including `NYASH_ALLOCATOR_PROVIDER`,
  `HAKO_ALLOCATOR_PROVIDER`, and broad `ALLOCATOR_PROVIDER_*` names;
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

M66-M86 were mirrored in the heavy progress tables while the activation safety
ladder was being named.

Historical M75 sentence kept for past guards: Provider proof boundary ladder is now closed through M75.

M87 and later follow the lightweight docs sync policy. M87-M103 are landed.
M95 is the latest closeout checkpoint, M98 is the latest runtime diagnostic
checkpoint, M99 is the latest CLI diagnostic checkpoint, M98B is the latest
BoxShape cleanup checkpoint, M100 is the latest behavior owner/entry
checkpoint, M101 is the first small runtime fail-fast implementation row, M102
is the first caller-provided selected-provider precondition row, and M103 is
the selected-provider proof validation row:

1. SSOT or implementation doc first.
2. Small runtime/CLI code only when the row explicitly allows it.
3. Prefer an existing post-M101 guard or a tiny focused guard; do not grow
   `tools/checks/k2_wide_allocator_gate.sh` per row by default.
4. `current_state_pointer_guard`.
5. `git diff --check`.

Do not require phase README, phase taskboard, global mimalloc taskboard, or the
full progress tables in this file to change for every row. Update those mirrors
at closeout rows or when their own stable contract changes.

## Next Step

Historical M86 sentence kept for past guards: The next safe row is M87 activation decision fixture contract.

The activation decision diagnostic ladder is now closed through M91. M92 names
the single future activation implementation owner/entry without creating
runtime activation behavior. M93 adds diagnostic-only parsing/reporting over
caller-provided registry snapshot TOML text while keeping active registry
construction inactive. M93B centralizes the diagnostic false outputs before the
CLI row. M94 exposes only an explicit caller-provided TOML path and keeps active
registry construction, provider selection, proof consumption, rollback
preparation, activation gate opening, hook activation, process allocator
replacement, and hidden environment discovery split into later guarded rows.
M95 closes out the M92-M94/M93B activation diagnostic inventory without changing
runtime behavior. M96 adds diagnostic-only parsing/reporting over
caller-provided selection decision TOML text while keeping provider selection
and activation inactive. M97 exposes that report through an explicit
caller-provided TOML path while keeping provider selection and activation
inactive. M97B centralizes allocator provider diagnostic TOML helpers and
fact-check shape without behavior changes. M98 adds diagnostic-only
parsing/reporting over caller-provided proof-bundle consumption TOML text while
keeping proof consumption inactive. M98B splits the runtime diagnostic
implementation into focused report-owner modules behind the historical registry
facade without behavior changes. M99 exposes the inactive proof-bundle report
through an explicit TOML-path CLI while keeping proof consumption inactive. M100
reserves the single future proof-bundle consumption behavior owner/entry under
the activation owner while keeping proof consumption inactive. M101 creates the
reserved runtime entry as a fail-fast attempt report that blocks when a real
selected provider is absent. M102 validates only a caller-provided selected
provider precondition; it does not select a provider and does not consume
proofs. M103 validates selected-provider proof facts while still keeping
`proof_bundle_consumed=false`. The default next implementation work now returns
to `.hako` mimalloc / `hako_alloc` completeness on top of the capability
substrate. M104 is the next safe row only if the optional host allocator
replacement ladder is explicitly reopened; even then, it may create only proof
custody/readiness and still must not prepare rollback, open the gate, install a
hook, activate native allocator behavior, or replace the process allocator. Any
later activation behavior row must keep using explicit owner/entry contracts
and must not piggyback on diagnostic CLI surfaces.

Post-M101 growth stop-line: if `allocator_provider_activation.rs` starts owning
detailed proof/rollback/gate internals, keep only the public orchestration entry
there and split internals into focused runtime modules. If provider/mimalloc/hook
names appear in `.inc` route/matcher logic, stop immediately.

M103 must start with `src/runtime/allocator_provider_proof_validation.rs` for
proof-validation internals and must not add another per-row step to
`tools/checks/k2_wide_allocator_gate.sh`. Use a consolidated post-M101 guard or
a focused row proof that verifies the M103 guard is not individually registered
in the wide allocator gate.
