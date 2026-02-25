# `lang/src/compiler/mirbuilder/recipe/` (R0-R3)

Responsibility:
- Hold `.hako` mirbuilder Recipe-first core vocabulary only.
- Provide data containers and structure validation primitives for later lanes.

Current scope (R0-R3):
- `recipe_item_box.hako`: `Seq` / `If` / `Loop` / `Exit` item constructors.
- `recipe_port_sig_box.hako`: minimal def/update tracking (`PortSig`).
- `recipe_verifier_box.hako`: verifier skeleton (structure + PortSig aggregation).
- `recipe_facts_box.hako`: stmt-4 facts extractor (`Print` / `Local` / `Assignment` / `Return`).

Non-goals:
- No direct lowering to MIR JSON.
- No parser acceptance expansion.
- No fallback behavior.

Fail-fast tag (SSOT):
- `[freeze:contract][hako_mirbuilder]`
