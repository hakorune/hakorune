# 293x-532 RANDOM-CAP-001 Uses Random Capability Decision

Status: landed
Date: 2026-05-17

## Decision

`RANDOM-CAP-001` is a Hakorune core capability row selected by `MIMAP-049B`.

It decides the first narrow compiler-facing contract for `uses random`:

```text
uses random:
  capability metadata / verifier contract surface

random or entropy execution:
  still unsupported unless a later backend route row opens it explicitly
```

The row keeps deterministic proof keys legal only in proof/inventory owners and
keeps cryptographic hardening claims inactive until a real entropy route exists.

## Scope

- Define `uses random` as a recognized low-level capability name.
- Make the MIR capability/fail-fast contract visible enough that future route
  preflight rows can reject unsupported random/entropy execution early.
- Add a focused guard that prevents accidental random externs, backend helper
  name matchers, and secure-list behavior changes.
- Update capability docs and current pointers at closeout.

## Stop Lines

- No random/entropy extern route.
- No OS/provider/hook/TLS/atomic entropy source.
- No secure-list encode/decode behavior change.
- No cryptographic hardening claim.
- No provider activation, host allocator replacement, hook, or
  `#[global_allocator]`.
- No backend `.inc` app/box-name matcher.
- No broad `uses` checker expansion beyond the `random` capability decision.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RANDOM-CAP-001.1` | Add the `uses random` contract SSOT. | docs say `uses random` is recognized but execution is unsupported. | no random route |
| `RANDOM-CAP-001.2` | Add/adjust the narrow MIR capability owner. | capability metadata can name `random` without implying backend support. | no broad checker |
| `RANDOM-CAP-001.3` | Add a focused guard. | guard proves no random extern/source/matcher leak and no secure-list behavior change. | no behavior change |
| `RANDOM-CAP-001.4` | Close out current pointers. | current pointer guard passes and next row is selected. | no multi-row bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_random_capability_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Return Condition

This row closes when `uses random` is a documented capability/fail-fast
contract, while random execution remains explicitly unsupported and guarded.

## Implementation Result

`RANDOM-CAP-001` adds:

```text
SSOT:
  docs/development/current/main/design/random-capability-failfast-ssot.md
MIR owner:
  src/mir/effect_capability_plan.rs
metadata carrier:
  FunctionMetadata.declared_capability_uses
guard:
  tools/checks/k2_wide_random_capability_contract_guard.sh
```

The compiler now keeps source `uses random` as declaration-local capability
metadata and emits:

```text
metadata.capability_plans:
  allow=[hako.random]
  source=source_uses
  verified=false
```

Only `random` is promoted by this row. Existing `uses osvm` / `uses atomic` /
`uses rawbuf` checker expansion remains future work. No random extern route,
entropy source, backend matcher, provider activation, or secure-list behavior
change is added.

## Evidence

```text
cargo test -q --lib mir_transports_source_uses_random_as_metadata_only_capability_plan
bash tools/checks/k2_wide_random_capability_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`RANDOM-CAP-001` selects `RANDOM-CAP-002`.

```text
row:
  RANDOM-CAP-002 random capability unsupported-route preflight
classification:
  Hakorune core diagnostics / fail-fast row
why now:
  `uses random` now has MIR metadata. The next row should make unsupported
  random execution fail before backend emission when a future allocator row
  tries to use it.
stop lines:
  no random extern route
  no entropy source
  no secure-list behavior change
  no provider activation
```
