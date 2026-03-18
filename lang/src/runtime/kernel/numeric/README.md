# runtime/kernel/numeric

`.hako` numeric kernel policy owner for narrow math kernels.

Current pilot:
- `matrix_i64.hako`
  - owns the `MatI64.mul_naive` core loop / policy
  - ring1 wrapper stays in `lang/src/runtime/numeric/mat_i64_box.hako`
  - keep this narrow; do not move `IntArrayCore` allocation ownership here
  - ring1 wrapper constructor style is `new MatI64(rows, cols)`

Non-goals:
- numeric ABI / handle registry ownership
- raw memory substrate
- extra runtime layer
