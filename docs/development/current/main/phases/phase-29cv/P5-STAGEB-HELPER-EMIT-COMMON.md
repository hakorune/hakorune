# P5: Stage-B helper emit common path

Scope: collapse duplicated Stage-B Program(JSON v0) smoke-helper emit logic
behind one helper-local common path.

## Why

`tools/smokes/v2/lib/stageb_helpers.sh` had three copies of the same
Stage-B compile/extract/cleanup shape:

- `stageb_compile_to_json()`
- `stageb_compile_to_json_with_bundles()`
- `stageb_compile_to_json_with_require()`

The only real difference is the extra Stage-B CLI arguments. Keeping three
copies makes Program(JSON v0) fixture ownership harder to audit.

## Decision

Add `stageb_compile_to_json_with_args()` as the SSOT for:

- temp Hako source creation
- Stage-B compiler invocation
- Program(JSON v0) header extraction
- temp cleanup

Keep the public helper names as thin wrappers because existing smokes call
those names directly.

## Files

- `tools/smokes/v2/lib/stageb_helpers.sh`

## Acceptance

```bash
bash -n tools/smokes/v2/lib/stageb_helpers.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_alias_table_ok_vm.sh
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_alias_table_bad_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

