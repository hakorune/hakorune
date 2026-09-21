---
Status: closed__2026-09-21__WarningInstanceConstructorManifestTestFacade
Task: MIRBUILDER-WARNING-INSTANCE-CONSTRUCTOR-MANIFEST-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i21-2026-09-21.md
Implementation permission: true for one cfg(test) instance-constructor manifest import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I22
---

# Warning cleanup: instance-constructor manifest test facade

## Six-line brief

```text
Decision: gate the instance-constructor manifest error import consumed only by
  local tests; keep normal_instance_constructor_demand_manifest.rs as owner.
Source authority + canonical issuer: normal_instance_constructor_demand_manifest.rs;
  cfg(test) name resolution is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, admission semantics, visibility
  redesign, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to InstanceConstructorDemandManifestIssueV1's
  import, then run the fixed gates once each.
Non-claims: no constructor ABI redesign, semantic change, suppression, or
  production route change.
```

## Precondition and acceptance

I21 records one lib-only unused-import diagnostic at
`src/mir/builder/normal_instance_constructor_admission.rs:28`. The local test
module consumes the error through `super::*`; production admission does not.
Acceptance requires lib warnings to drop from 1,809 to 1,808, lib-test warnings
to remain 561, both fixed quick-profile commands to exit 0, and fmt/diff/pointer/
lifecycle guards to remain green.

Only the import declaration may change. If the consumer classification or
count differs, stop and return to design stop.

## Execution evidence

The selected error import is now gated with `#[cfg(test)]`; the manifest
builder, role types, and production constructor admission are unchanged.
`cargo check --profile quick --lib -j4` exited 0 with lib warnings at
**1,808**. The sequential `cargo test --profile quick --lib --no-run -j4`
exited 0 with lib-test warnings at **561** and produced the test executable.
No constructor ABI or production route changed.
