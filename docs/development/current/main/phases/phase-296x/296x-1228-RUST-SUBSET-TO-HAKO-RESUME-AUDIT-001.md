---
Status: Done
Decision: accepted
Date: 2026-06-18
Scope: Resume the rust-subset-to-hako app front and pin the current EXE/AOT blocker.
Related:
  - apps/rust-subset-to-hako/README.md
  - apps/rust-subset-to-hako/STATUS.md
  - apps/rust-subset-to-hako/HAKO_JSON_PLAN.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md
---

# RUST-SUBSET-TO-HAKO-RESUME-AUDIT-001

## Result

The rust-subset-to-hako app front is reopened as the active selfhost/app front.

Verified:

```text
python_reference_selftest=ok
json_native_reserved_word_type_blocker=fixed
json_native_probe_mir_json_emit=ok
hako_converter_mir_json_emit=ok
vm_product_route=retired
primary_app_validation_route=EXE/AOT
```

The `.hako` converter exists, but EXE/AOT cannot judge converter parity yet:

```text
converter=apps/rust-subset-to-hako/convert.hako
json_owner=apps/lib/json_native
exe_status=blocked
error=unsupported pure shape for current backend recipe
target_shape_blocker_symbol=JsonParserUtils.parse_json/1
blocker_inner_call=JsonParser.parse/1
owner=AOT backend user-box method lowering
```

## Reproduction

```bash
bash apps/rust-subset-to-hako/smoke.sh
```

Current expected result:

```text
summary=blocked_by_aot_json_parser_user_box_method
```

## Next

```text
selected_next_task=RUST-SUBSET-TO-HAKO-AOT-JSON-METHOD-CALL-001
implementation_allowed=1
```

Goal:

```text
Allow the EXE/AOT route to accept the minimal user-box method call shape needed
by JsonParserUtils.parse_json(text):

new JsonParser()
parser.birth()
parser.parse(text)
parser.has_errors()
```

Stop lines:

```text
do not re-enable VM as the product app route
do not replace json_native with a JSON DLL in this row
do not special-case rust-subset-to-hako names
do not bypass JsonParser.parse with hardcoded fixture JSON
```

## Contract

```text
output_contract=rust-subset-to-hako-resume-audit-v0

python_reference_selftest=ok
hako_converter_mir_json_emit=ok
json_native_probe_mir_json_emit=ok
exe_blocker_pinned=1
blocker_symbol=JsonParser.parse/1
implementation_allowed=1
selected_next_task=RUST-SUBSET-TO-HAKO-AOT-JSON-METHOD-CALL-001

summary=ok
```
