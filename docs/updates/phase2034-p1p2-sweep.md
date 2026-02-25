# Phase 20.34 — P1/P2 Sweep (JsonFrag/PatternUtil, loop_form using)

Status: Completed (2025-11-03)

Scope
- P1: Replace ad‑hoc JSON scans with JsonFragBox and unify reverse lookups via PatternUtilBox.
- P2: Remove `include` from loop_form lowers and rely on `using` only.

Changes
- JsonFragBox adoption
  - Use `read_int_after(text, kv+8)`, `read_bool_after(text, kv+8)`, `read_string_after(text, k+5)` consistently.
  - Locate keys with `index_of_from(text, "\"key\":", pos)`.
- PatternUtilBox adoption
  - `find_local_int_before(text, name, before_pos)` and `find_local_bool_before(...)` for reverse lookups.
  - Optional: `map_cmp` for operator mapping (<, >, <=, >=, ==, != → Lt, Gt, ...).
- loop_form lowers
  - `lower_loop_simple/count_param/sum_bc`: remove `include` of loop_form; keep `using selfhost.shared.mir.loopform as LoopFormBox`.

Verification
- Internal lowers canaries: PASS (structure, value paths). Logical Var/Var lower (direct) is green.
- Emit→Core rc verification: use `verify_mir_rc` with `HAKO_VERIFY_PRIMARY=hakovm` while Core rc line is being normalized.
- Test runner fixes: guard unset `prefile`; JSON extraction via `jq` with leading noise stripping.

Policy
- Behavior invariant (fail‑fast). New toggles default OFF. Minimal, localized diffs only.

Next
- Continue registry migration for MirBuilder (toggle‑guarded).
- Migrate remaining non‑internal `include` sites in a separate PR.
