# field_receiver_provenance

This module owns one bounded, read-only proof:

```text
field-base value
  -> finite acyclic ordinary Copy/Phi graph
  -> exact current implicit receiver parameter 0
```

Authority:

- current method parameter publication identifies the receiver;
- the in-progress `MirFunction` owns definitions and CFG edges;
- existing verification utilities provide ephemeral reachability/dominance;
- `user_box_field_decls` confirms the receiver owner exists.

Non-authority:

- `variable_map["me"]`;
- `current_static_box` or function/symbol parsing;
- method/field names;
- runtime tags and downstream route plans;
- `value_origin_newbox` backfill;
- persistent ValueId-to-owner/type/equivalence maps.

The proof accepts ordinary `Copy` and finite acyclic non-loop `Phi` only.
Every terminal root must be receiver parameter zero. Value-definition cycles,
CFG self-loops/backedges, foreign roots, unsupported definitions, incomplete
PHIs, and unavailable edge inputs reject.

`R0-DECLFIELD-PHI0-S0/P0` keeps production consumers at zero. The later I0
may connect exactly one consumer in `declared_field_type_for_value`; no other
field/property/method owner may consume this proof.
