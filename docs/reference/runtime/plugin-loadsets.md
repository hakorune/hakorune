# Runtime Plugin Loadsets

Status: accepted reference  
Scope: runtime plugin selection, preflight, and evidence vocabulary

## Purpose

Hakorune can run with different plugin sets. A small exact-EXE app should not
silently pay for every plugin in the repository, and a compatibility run should
not hide that it used the root plugin set.

The runtime contract is therefore:

```text
config intent -> selected loadset -> preflight plan -> eager load selected set
```

The current implementation still uses `nyash.toml` as the runtime plugin input.
`hako.toml` is the package-facing intent file. Tools may lower a selected
profile into a generated runtime `nyash.toml`.

## Loadset Names

```text
root:
  Compatibility profile. Preserve repository/root plugin discovery.

empty:
  No plugin libraries. Used by exact-EXE workloads that do not need plugin
  Boxes.

all:
  Explicit heavy profile. Select every configured library in the chosen config.

app:
  Reserved future package profile. Select only libraries required by the app or
  package manifest.

core:
  Reserved future core profile. Select a small stable runtime/core plugin set.
```

`root` and `all` may currently resolve to the same libraries. The names are
separate because `root` means compatibility while `all` means an explicit heavy
loadset choice.

## Loading Policy

Current policy:

```text
plugin_load_policy=eager_selected
```

Rules:

- no implicit lazy loading;
- load only the selected loadset;
- expose the selected loadset in evidence;
- make preflight available without loading libraries;
- do not implicitly lazy-load plugins;
- do not load a provider/plugin because a fallback or free-miss path failed.

Lazy loading is a future explicit policy only. It must be visible in the
manifest/preflight plan and is not allowed for comparison or benchmark profiles
until measured separately.

## Preflight Plan

The preflight plan is diagnostic-only. It parses the selected config and emits
the selected libraries without calling `dlopen`.

Current tool:

```bash
python3 tools/allocator/hako_plugin_loadset_plan.py \
  --config hako.toml \
  --loadset empty
```

Output contract:

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

The plan may include per-library configured paths, resolved paths, candidate
paths, and Box names. These are diagnostics only.

## Comparison Evidence

Phase-295x comparison runners emit the selected `.hako` loadset:

```text
hako_runtime_config_profile=<root|empty>
hako_selected_loadset=<root|empty>
hako_plugin_load_policy=eager_selected
hako_selected_library_count=<n>
hako_missing_library_count=<n>
hako_loadset_preflight_ok=<0|1>
```

This is not a winner claim. It records fixed runtime/plugin footprint context
for RSS and startup measurements.

## Config Files

Current roles:

```text
hako.toml:
  package-facing intent and root repository configuration.

nyash.toml:
  current runtime plugin loader input.

generated nyash.toml:
  allowed for tools that need a selected profile, such as empty comparison
  runs.
```

The runtime does not currently read `hako.toml` directly as its plugin-loader
input. Changing that is a separate compatibility row.

## Stop Line

This reference does not make `empty` the default, change root compatibility,
enable provider packages, generate DLLs, install hooks, replace the process
allocator, or enable `#[global_allocator]`.
