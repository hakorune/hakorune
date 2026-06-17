# Array Text Observer Region Contract

This module owns the nested executor contract for array/text observer routes.
It does not introduce a backend proof family.

Responsibilities:

- `array_text_observer_region_contract.rs`: facade and stable re-exports.
- `model.rs`: passive region/executor contract data.
- `matcher.rs`: MIR shape recognition for the currently accepted region form.
- `types.rs`: stable string vocabulary consumed by metadata/report emitters.

Boundaries:

- MIR owns legality and proof construction.
- Backends consume emitted metadata; they must not inspect these MIR matchers.
- Fallback or failed shape evidence is not a contract.
- New accepted shapes belong in `matcher.rs` only after a card fixes the shape,
  report fields, and guard expectations.
