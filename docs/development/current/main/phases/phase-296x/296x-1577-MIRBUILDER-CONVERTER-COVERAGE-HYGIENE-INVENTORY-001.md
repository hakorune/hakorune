# 296x-1577 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-INVENTORY-001

Status: landed
Date: 2026-06-22

## Purpose

Inventory the remaining MirBuilder converter coverage hygiene debt without
opening any raw-string rewrite or route-selection work yet.

## Scope

```text
BoxCount: one inventory slice
owner: MirBuilder converter coverage hygiene
input: current family artifact specs, current task-order SSOT, current converter path scan
output: durable remaining-slice inventory
```

## Observed State

```text
raw_hako_harness_sites=5
return_source_sites=1
typed_converter_core=present
shared_renderer_layer=present
shared_generator_layer=present
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Required Checks

```text
python3 - <<'PY'
from pathlib import Path
paths = [
    Path('tools/rust_lifecycle/mirbuilder_family_artifacts.py'),
    Path('tools/rust_lifecycle/mirbuilder_carrier_snapshot_artifacts.py'),
]
for path in paths:
    text = path.read_text()
    print(path, text.count('main_lines=_lines'), text.count('ReturnSource'))
PY
```

## Acceptance

```text
the remaining raw-string carrier sites are enumerated
the five raw-harness slices are separated from the one ReturnSource contract slice
the typed converter core is left alone
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_raw_string_rewrite=1
```
