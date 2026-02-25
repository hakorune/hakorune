# Phase 275: Coercion Implementation (truthiness / `==` / `+`)

Status: ✅ completed (Phase 275 P0)

Goal: implement the accepted coercion SSOT across backends (Rust VM + LLVM harness) and update language docs and fixtures so behavior cannot drift.

Accepted SSOT decisions:
- `docs/development/current/main/phases/phase-274/P3-DECISIONS.md`

Language SSOT (updated after Phase 275):
- `docs/reference/language/types.md`

---

## What changes in this phase

Implement these semantics (A1/B2/C2):

1) truthiness (boolean context)
- `Void` in condition → **TypeError**
- `BoxRef` in condition → **TypeError**, except explicit bridge boxes (BoolBox/IntegerBox/StringBox)
- “object is always truthy” remains prohibited

2) equality (`==`)
- allow Int↔Float numeric comparison only (precise rule; no accidental true via float rounding)
- Bool is not a number: no Bool↔Int coercion
- other mixed kinds: deterministic `false` (not error)
- BoxRef equality: identity only

3) `+`
- numeric add: Int+Int, Float+Float, Int↔Float promotion to Float
- string concat: **String+String only**
- String mixed (`"a"+1`, `1+"a"`) → TypeError (no implicit stringify)

---

## Acceptance criteria (minimum)

- Rust VM behavior matches `P3-DECISIONS.md` for truthiness / `==` / `+`.
- LLVM harness behavior matches Rust VM (no backend divergence).
- `docs/reference/language/types.md` is updated to reflect the new executable SSOT.
- New fixtures + smokes lock behavior (VM + LLVM) without introducing environment-variable sprawl.
- No by-name hardcoding for “special cases”; if something must be special, it must be a documented bridge rule.

---

## Implementation guide

- `docs/development/current/main/phases/phase-275/P0-INSTRUCTIONS.md`
- `docs/development/current/main/phases/phase-275/P1-INSTRUCTIONS.md`
