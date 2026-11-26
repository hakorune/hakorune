# Loop Recipe V2 wire example

`emit_m8c_scans_wire.hako` prints one fixed JSON artifact for the typed
`find_ok` scan in `apps/tests/scan_with_init_typed_ok_min.hako`. It uses schema
version 2 and producer ID `scan_with_init_v2`. Its example includes
`call_slot`, `text_eq`, a Text value, and a Return exit.

The corpus exercises the example's compilation and output. The compiler
produces and verifies its own recipes through the V2 contract.
[Loop Recipe V2 contract](../../../../../docs/reference/mir/loop-recipe-contract.md)
