# Standalone EXE Routes

Status: provisional reference  
Scope: standalone route vocabulary and evidence fields

## Purpose

Standalone EXE support must say what runtime and plugin loadset the executable
contains or uses. Otherwise a small app can appear large because it silently
started with a root plugin set, or a compatibility build can be confused with a
minimal build.

Standalone routes are therefore defined as a profile family layered over:

```text
runtime_config_profile
selected_loadset
plugin_load_policy
link_policy
```

This reference defines the vocabulary. It does not mean the standalone package
backend is implemented.

## Route Names

```text
standalone-minimal:
  Minimal app route. Use the smallest explicit runtime/loadset that the app
  requires. Current comparison workloads map this to runtime_config=empty and
  selected_loadset=empty.

standalone-root:
  Compatibility route. Preserve repository/root runtime configuration and root
  plugin discovery.

standalone-diagnostic:
  Diagnostic route. Emit loadset, dependency, section, and footprint evidence
  without memory or speed winner claims.
```

## Link Policy

Current vocabulary:

```text
link_policy=exact-mir-exe
link_policy=standalone-package
link_policy=provider-package
```

Meanings:

- `exact-mir-exe`: current exact-MIR EXE route. It can run with an explicit
  runtime config/loadset profile.
- `standalone-package`: future app package route. It may generate an executable
  plus manifest/config artifacts.
- `provider-package`: future provider/DLL/shared-library package route. It is
  not part of the current standalone EXE MVP.

## Evidence Fields

Standalone-related evidence should include:

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

`standalone_packaging_generated=0` is expected while the implementation is only
using the exact-MIR EXE route and has not generated a standalone package.

## Static vs Dynamic Plugins

MVP reading:

```text
dynamic eager selected:
  load only the selected dynamic plugin libraries.
```

Future readings:

```text
static selected:
  embed a selected core/plugin set into the artifact.

hybrid selected:
  statically include core runtime pieces and dynamically load selected external
  plugins.
```

All modes must remain preflight-visible. Hidden static or dynamic plugin cost is
not allowed.

## Stop Line

This reference does not implement `hakorune build --kind standalone`, generate
standalone manifests, change exact-MIR EXE behavior, make `empty` the default,
compute RSS/speed winners, or open provider/DLL/replacement/hook/global
allocator seams.
