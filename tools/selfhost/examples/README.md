# Selfhost Example Payloads

This directory is mixed.

## General Generators

- `gen_v1_*.sh`
- small helper/example payloads used to generate or inspect MIR(JSON)
- not part of the legacy compat selfhost wrapper ownership

## Archive-Later Compat Payload

- `hako_llvm_selfhost_driver.hako`
- thin proof/example driver for the historical compat wrapper:
  - `tools/selfhost/run_compat_pure_selfhost.sh`
- exercises the legacy `CodegenBridgeBox.emit_object_args(...)` and
  `CodegenBridgeBox.link_object_args(...)` route
- not a daily backend owner
- keep until the compat wrapper gains a root-first replacement or is retired as
  a whole
