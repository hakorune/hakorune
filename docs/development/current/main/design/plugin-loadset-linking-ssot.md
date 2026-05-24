# Plugin Loadset Linking SSOT

Status: Active  
Owner: phase-295x comparison/runtime configuration lane  
Last updated: 2026-05-25

## Purpose

Hakorune plugin loading must be explicit enough that memory footprint and
startup behavior can be diagnosed from a small plan, not inferred after a
surprising `dlopen`.

The phase-295x comparison work found that root plugin loading can dominate an
otherwise tiny exact-EXE RSS baseline. The fix is not to delete root plugin
loading or silently lazy-load plugins. The fix is to make the selected loadset a
first-class runtime contract.

## Decision

Use manifest-selected loadsets with eager loading of the selected set.

```text
hako.toml package intent
  -> selected loadset/profile
  -> runtime plugin plan
  -> generated runtime nyash.toml or direct loader plan
  -> eager load selected libraries
```

The default root behavior remains compatibility-first. Comparison and
benchmarking profiles must select their loadset explicitly and record it in
their evidence.

## Loadset Vocabulary

```text
root:
  Compatibility profile. Preserve current repository/root plugin discovery.

empty:
  No plugin libraries. Used by exact-EXE comparison workloads that do not need
  plugin boxes.

all:
  Explicit spelling for every configured library in the selected config.

app:
  Future package profile. Load only libraries required by the selected package
  or app manifest.

core:
  Future named core profile. Load a small stable set of runtime/core plugins.
```

`root` and `all` may currently resolve to the same library set. They are still
separate names because `root` means compatibility, while `all` means an explicit
heavy loadset choice.

## Loading Policy

```text
plugin_load_policy=eager_selected
```

Rules:

- no implicit lazy loading;
- no free-miss or fallback path may load a provider/plugin unexpectedly;
- comparison profiles must emit `selected_loadset`,
  `selected_library_count`, and `plugin_load_policy`;
- loadset preflight must be available without loading libraries;
- provider/DLL/replacement/hook/global allocator seams remain parked unless an
  active row explicitly opens them.

Lazy loading is not a default policy. If it is added later, it must be explicit
in the manifest, preflight-visible, and forbidden for comparison/winner-claim
profiles until measured separately.

## Preflight Plan Contract

A preflight plan is a diagnostic artifact. It does not call `dlopen`, execute a
provider, install hooks, or alter runtime defaults.

Required fields:

```text
output_contract=hako-plugin-loadset-plan-v0
selected_loadset=<root|empty|all|app|core>
plugin_load_policy=eager_selected
library_count=<n>
missing_library_count=<n>
preflight_ok=<0|1>
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
```

The plan may include per-library names, configured paths, resolved paths, box
names, and path-existence diagnostics. These are diagnostics only; they are not
allocator performance claims.

## Stop Line

This SSOT does not:

- change default NyRT plugin loading;
- teach NyRT to read `hako.toml` directly;
- remove `nyash.toml` compatibility;
- make `empty` the default runtime profile;
- compute RSS winners or require RSS parity;
- open provider package, DLL generation, process replacement, hook, backend
  matcher, or `#[global_allocator]` seams.
