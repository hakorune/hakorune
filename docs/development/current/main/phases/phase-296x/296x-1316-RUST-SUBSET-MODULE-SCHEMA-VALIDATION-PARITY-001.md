# 296x-1316 RUST-SUBSET-MODULE-SCHEMA-VALIDATION-PARITY-001

Status: closed
Date: 2026-06-19

## Purpose

Align the `.hako` RustSubset converter with the Python reference converter for
module-level schema validation before crate manifest implementation.

## Decision

```text
known Unsupported node:
  emit TODO comment

unknown schema item kind:
  fail-fast

unknown schema statement / expression kind:
  Python reference fails fast
  .hako converter emits an error marker in the generated skeleton

schema_version != 0:
  fail-fast

root kind != RustSubsetModule:
  fail-fast
```

The converter core remains input-route agnostic. FileBox, stdin, external
adapter invocation, and crate graph discovery stay outside `converter_core.hako`.

## Implementation

```text
apps/rust-subset-to-hako/schema/rust_subset_schema.hako
  document_status()
  document_status_message()

apps/rust-subset-to-hako/converter_core.hako
  fail()
  schema_version/root-kind validation
  unknown item fail-fast
  unknown statement/expression error marker

apps/rust-subset-to-hako/probes/regression/
  schema_document_validation_probe.hako
  schema_unknown_item_validation_probe.hako
  schema_unsupported_validation_probe.hako
```

Python reference selftest covers unknown statement and expression fail-fast.
The `.hako` regression probes are kept as small standalone probes for
root/item/Unsupported boundaries; the default smoke keeps schema validation in
the Python reference selftest and continues to use existing converter fixtures
for EXE/AOT app-route parity.

## Stop Line

```text
crate_manifest_schema_accepted=0
converter_core_filebox_ownership=0
external_rust_parser_adapter_enabled=0
new_hako_syntax_enabled=0
known_unsupported_is_error=0
unknown_schema_kind_is_todo=0
```

## Evidence

```bash
python3 apps/rust-subset-to-hako/selftest.py
cargo check -q --lib
bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
RUST-SUBSET-PATH-NAME-NORMALIZATION-001
```

Close path/name normalization before any `creat`-style crate pilot:

```text
structured Path / SymbolRef
source_name / emitted_name
reserved-word escaping
tuple field _0 / _1 policy
duplicate emitted_name detection
simple-pattern-only fail-fast
```
