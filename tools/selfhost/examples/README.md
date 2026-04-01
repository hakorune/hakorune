# Selfhost Example Payloads

This directory is now generators-first.

## Cleanup Bands

| Band | State |
| --- | --- |
| Now | `lang/src/vm/hakorune-vm/extern_provider.hako` |
| Next | `tools/selfhost/run_compat_pure_selfhost.sh` + `tools/selfhost/compat/hako_llvm_selfhost_driver.hako` |
| Later | `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)` / `CodegenBridgeBox.emit_object_args(...)` / Rust dispatch residues |

## General Generators

- `gen_v1_*.sh`
- small helper/example payloads used to generate or inspect MIR(JSON)
- not part of the legacy compat selfhost wrapper ownership

## Note

- the historical compat payload has moved to `tools/selfhost/compat/`
- keep this directory focused on generators and small helper/example payloads
