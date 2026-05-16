# 293x-532 RANDOM-CAP-001 Uses Random Capability Decision

Status: selected current
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
