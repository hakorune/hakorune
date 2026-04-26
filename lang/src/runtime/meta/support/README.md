# lang/src/runtime/meta/support

Scope:
- Active support utilities exported by `selfhost.meta` for compatibility or
  fixture lanes.
- Keep these utilities physically separated from compiler semantic contract
  tables.

Rules:
- Do not add compiler semantic tables here.
- Do not add lowering, probing, or backend emission here.
- Each support export must have an owner-audit card and a retirement or
  quarantine condition.

Current exports:
- `json_shape_parser.hako`
  - exports `JsonShapeToMap`.
  - active JoinIR fixture/support utility audited by `291x-298`.
  - kept for `JsonShapeToMap._read_value_from_pair/1` bridge/frontend tests.
