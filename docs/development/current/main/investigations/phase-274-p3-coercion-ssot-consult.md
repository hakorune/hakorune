# Phase 274 P3 — coercion SSOT consultation memo (truthiness / `==` / `+`)

Status: for external consultation (self-contained)

Goal: decide and freeze **language-level coercion semantics** as SSOT, then make VM/LLVM/docs consistent.

Repo context (Nyash/Hakorune):
- Philosophy: **Fail-Fast**, **no guessing**, **SSOT-first** (runtime semantics must not “accidentally” emerge from resolver/type-facts).
- Current SSOT doc draft: `docs/reference/language/types.md`
- Phase 274 overview: `docs/development/current/main/phases/phase-274/README.md`

This memo asks you to recommend a clean, modern coercion policy and (if needed) a migration plan.

---

## 0) Why we need P3

We have a dynamic runtime, but coercions are currently a mix of “historical convenience” and “implementation drift”.
If we don’t freeze these rules, type facts / resolver heuristics can start acting like semantics.

P3 is the decision phase for these three coercion points:
1) truthiness (conditions)
2) equality (`==`)
3) `+` (add/concat)

---

## 1) Current observed behavior (summary)

Treat this as “current reality”, not necessarily desired design.

### 1.1 truthiness (boolean context)

Current doc says (Rust VM behavior):
- Bool → itself
- Integer/Float → 0 is false, non-zero is true
- String → empty false, otherwise true
- Void → false
- BoxRef:
  - some “bridge boxes” (BoolBox/IntegerBox/StringBox/VoidBox) behave like their primitives
  - other BoxRef types currently **TypeError** (fail-fast)

### 1.2 `==` (equality)

Current doc says:
- same-kind primitives compare normally
- some cross-kind coercions exist (best-effort):
  - Integer ↔ Bool (non-zero == true)
  - Integer ↔ Float (numeric comparison)
- BoxRef == BoxRef is identity
- mixed kinds often return false (not an error)

### 1.3 `+` (add / concat)

Current doc says:
- Integer+Integer, Float+Float are numeric add
- if either side is String, it concatenates (stringifies the other operand)
- other combos are TypeError

---

## 2) Target design constraints

We want:
- Minimal, composable semantics
- Predictable failure (fail-fast where it prevents silent bugs)
- Avoid “JS-style surprise coercions”
- Keep language dynamic (no full static typing required)

---

## 3) Decision questions (please answer A/B/C)

### A) truthiness: should `Void` be allowed in conditions?

Options:
- A1 (Fail-Fast): `if void` → TypeError
- A2 (Compatibility): `if void` → false (but maybe add lint/warn later)

Question:
- Which should be SSOT, and why?
- If you choose A1, suggest the recommended explicit pattern (`x != Void`? `bool(x)`?).

### B) `==`: should we keep any cross-type coercions?

We want to avoid half-coercions that become “spec by accident”.

Options:
- B1 (Strict): cross-type equality is always false (no coercion)
- B2 (Number-only): allow Int↔Float numeric compare, but **do not** allow Bool↔Int
- B3 (Legacy): keep both Int↔Float and Bool↔Int

Question:
- Which option is the best “modern + safe” SSOT, and why?
- If you choose B2 or B3, define the exact rule (edge cases).

### C) `+`: should `"String" + 1` be allowed?

Options:
- C1 (Strict): only same-kind `+` is allowed:
  - Int+Int, Float+Float, String+String
  - anything else is TypeError
- C2 (Number-only): allow Int↔Float numeric add (promotion), but String mixed is TypeError
- C3 (Legacy): if either side is String, concatenate (stringify the other side)

Question:
- Which option is the best SSOT, and why?
- If you choose C2, define the promotion rule precisely (Int→Float only?).

---

## 4) Implementation impact / migration plan (if SSOT changes)

If your recommended SSOT differs from current behavior, please propose:
- Whether to do a “compatibility freeze” phase (document current behavior first), then a separate “breaking change” phase.
- What minimum tests/fixtures should exist to lock the SSOT:
  - truthiness cases (Bool/Int/Float/String/Void/BoxRef)
  - equality matrix (same-type + selected cross-type)
  - `+` matrix (including failure cases)

---

## 5) Preferred final output format

Please respond with:
1) Final SSOT decision table (truthiness / `==` / `+`)
2) Rationale (short)
3) Migration plan (if needed)
4) Suggested test matrix (minimum)

