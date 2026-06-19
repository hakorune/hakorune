---
Status: SSOT
Decision: accepted
Date: 2026-06-19
Scope: Structure for `.hako` app fronts that validate through EXE/AOT.
Related:
  - apps/rust-subset-to-hako/README.md
  - apps/rust-subset-to-hako/STATUS.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# Hako App Front Boundary Template

## Decision

`.hako` app fronts that are used for compiler construction / selfhost pressure
must separate startup, input, core logic, and acceptance.

The default shape is:

```text
static box Main:
  startup only
  choose input route
  call App.run/convert
  print result

box App:
  core logic
  no FileBox/stdin/argv ownership
  no host adapter ownership

InputRoute:
  embedded fixture first
  file/stdin/adapter in separate rows

Acceptance:
  EXE/AOT parity or explicit fail-fast
  VM product route remains retired
```

## Rationale

The rust-subset-to-hako app front exposed four independent blockers under one
symptom:

```text
FileBox input
json_native parse/tree traversal
converter helper calls
string / MapBox key materialization
```

Mixing them in `Main` made the active AOT blocker hard to read. Moving
conversion logic into `box RustSubsetConverter` and keeping `Main` as startup
only made the route trace actionable and allowed the embedded fixture slice to
close.

## Standard Layout

New app fronts should prefer:

```text
apps/<app>/
  README.md
  STATUS.md
  smoke.sh

  <app>.hako
    static box Main
    box <App>

  examples/
    input.*
    expected.*

  schema/
    input-v0.md

  probes/
    README.md
    stable/
    regression/
    investigations/
```

Existing app fronts may migrate incrementally. Do not perform a broad physical
move unless the app-front row selects probe layout cleanup as its only purpose.

Current rust-subset-to-hako shape:

```text
apps/rust-subset-to-hako/convert.hako:
  startup wrapper only

apps/rust-subset-to-hako/converter_core.hako:
  RustSubset JSON -> .hako skeleton conversion

apps/rust-subset-to-hako/fixtures/simple_subset_embedded.hako:
  host-generated embedded JSON handoff

apps/rust-subset-to-hako/probes/stable/json_probe.hako:
  stable EXE/AOT JSON probe used by smoke.sh
```

## Input Route Ladder

Input route work must be split from app core work.

Recommended order:

```text
1. embedded fixture parity [landed]
2. temporary bridge closeout when the fixture requires one [landed]
3. host-generated embedded fixture handoff [landed]
4. FileBox minimal EXE/AOT probe, separate from app core [blocked: emit-exe unsupported pure shape]
5. app file input after FileBox probe is green [blocked by 4]
6. external adapter invocation boundary
7. stdin/argv route
```

Rules:

```text
do not mix FileBox input with converter/app core changes
do not mix stdin/argv substrate with app semantics
do not re-enable VM product route for app validation
do not replace `.hako` library pressure with a host DLL unless a separate row
  accepts that boundary
```

## JSON / Schema Boundary

For JSON-backed app fronts:

```text
json_native:
  generic JSON parsing and tree access

app schema layer:
  schema validation
  required field checks
  model-specific errors

app core:
  conversion / analysis / emission
```

Schema-specific lookup logic must not be copied into every app access site.

Temporary bridges are allowed only when they are:

```text
explicitly named as temporary
scoped to an app-front blocker
documented with a removal condition
covered by EXE/AOT smoke
```

Current temporary bridge:

```text
owner=apps/lib/json_native/core/key_materializer.hako
call_site=apps/lib/json_native/parser/parser.hako object-key context
bridge=temporary RustSubset critical-key materialization bridge plus generic fallback
reason=scanner-derived critical keys such as kind are not stable on current EXE/AOT route
schema_key_dictionary_enabled=1
generic_unknown_key_fallback_enabled=1
removal_condition=scanner_derived_critical_keys_stable_on_exe_aot=1
final_json_semantics=0
```

The owning task is:

```text
JSON-NATIVE-SCHEMA-KEY-BRIDGE-CLOSEOUT-001
```

That task does not have to remove the bridge. It must make the bridge
auditable:

```text
bridge_is_temporary=1
schema_specific_json_library_semantics=0
converter_schema_specific_lookup=0
schema_key_dictionary_enabled=1
generic_unknown_key_fallback_enabled=1
removal_condition=scanner_derived_critical_keys_stable_on_exe_aot=1
next_escape_path=lower_StringBox_or_MapBox_key_canonicalization
```

This bridge must not grow into a general JSON feature. The preferred future
fix is either:

```text
generic JsonStringKeyMaterializer / JsonObjectKeyCanonicalizer
```

or a lower-level `StringBox` / `MapBox` key canonicalization fix.

The bridge is intentionally parser-contextual: only JSON object keys are
materialized. JSON string values must remain ordinary JSON string values.

## Probe Layout

Probe directories should distinguish acceptance from investigation.

```text
probes/stable:
  app acceptance probes
  called by smoke.sh

probes/regression:
  small bug-specific probes tied to a card or issue

probes/investigations:
  bring-up trail
  not called by default smoke

probes/retired:
  historical probes that no longer describe an active route
```

Do not delete investigation probes as a drive-by cleanup. Move/classify them in
a dedicated probe-layout row.

## Acceptance

For an app-front slice to close:

```text
main_startup_only=1
core_logic_in_app_box=1
input_route_named=1
vm_product_route=retired
primary_route=EXE_AOT
smoke_script_exists=1
smoke_script_checks_parity_or_failfast=1
temporary_bridges_have_removal_conditions=1
```
