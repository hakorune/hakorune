# Standalone EXE Route Contract SSOT

Status: Active  
Owner: phase-295x comparison/runtime configuration lane  
Last updated: 2026-05-25

## Purpose

Standalone EXE work must be visible as a route/profile contract before it
becomes a packaging backend. The phase-295x comparison lane already proved that
runtime config and plugin loadset selection can dominate the fixed RSS
baseline, so standalone artifacts must say what runtime/loadset they include.

## Decision

Define standalone EXE as a build/run profile family layered on top of:

```text
runtime_config_profile
selected_loadset
plugin_load_policy
link_policy
```

This SSOT does not implement standalone packaging. It defines names and
evidence fields so future implementation rows have one route vocabulary.

## Profiles

```text
standalone-minimal:
  Exact-EXE style application with the minimal selected loadset required by
  the app. For current comparison workloads this maps to runtime_config=empty
  and selected_loadset=empty.

standalone-root:
  Compatibility profile. Preserve repository/root runtime config and root
  plugin discovery semantics.

standalone-diagnostic:
  Diagnostic profile. Emit route/loadset/section/dependency evidence for a
  standalone artifact without declaring memory or performance winners.
```

Future profiles may split static/dynamic plugin linking, but they must remain
explicit and preflight-visible.

## Evidence Contract

Standalone route evidence should include:

```text
standalone_route=<minimal|root|diagnostic>
runtime_config_profile=<root|empty|app>
selected_loadset=<root|empty|app|core|all>
plugin_load_policy=eager_selected
link_policy=<exact-mir-exe|standalone-package|provider-package>
standalone_packaging_generated=<0|1>
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
```

`standalone_packaging_generated=0` is expected until a future implementation row
actually produces a package artifact.

## Linking Rules

- no implicit lazy plugin loading;
- no hidden fallback from missing plugin path to a different loadset;
- no provider/DLL/process replacement route is part of standalone EXE by
  default;
- default compatibility remains `root` until an active row changes it;
- memory/performance winner claims require a separate repeated measurement
  policy row.

## Stop Line

This SSOT does not implement `hakorune build --kind standalone`.

This SSOT does not:

- implement `hakorune build --kind standalone`;
- generate standalone package manifests;
- change exact-MIR EXE runtime behavior;
- change default plugin loading;
- open provider package, DLL, process replacement, hook, backend matcher, or
  `#[global_allocator]` seams;
- compute RSS or speed winners.
