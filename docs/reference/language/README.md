# Hakorune / Nyash Language Reference – Index

This is the entry point for the current Hakorune / Nyash language reference.

Current canonical surface:

- Grammar and accepted rows: reference/language/EBNF.md
- Type and enum surface: reference/language/types.md
- Option / Result enum prelude: reference/language/option.md
- Low-level language surface: reference/language/low-level-capabilities.md
- Delegation and no-inheritance rule: reference/language/field-visibility-and-delegation.md

Historical long-form reference:

- reference/language/LANGUAGE_REFERENCE_2025.md is a historical snapshot. It is
  useful for old Phase 12/15 context, but it is not the current canonical source
  when it conflicts with the files above.

- Syntax Cheat Sheet: quick-reference/syntax-cheatsheet.md
- Phase 12.7 Grammar Specs (ternary, sugar; peek → match に統合):
  - Overview: archive/roadmap/phases/phase-12.7/grammar-specs/README.md
  - Token/Grammar: archive/roadmap/phases/phase-12.7/ancp-specs/ANCP-Token-Specification-v1.md
- Legacy sugar notes (?., ??, |> and friends): parser/sugar.rs (source) and tools/nyfmt/NYFMT_POC_ROADMAP.md.
  These are not permission to add new canonical surfaces.
- Match Expression (pattern matching): see the Language Reference and EBNF (peek was replaced by match)

Statement separation and semicolons
- See: reference/language/statements.md — newline as primary separator; semicolons optional for multiple statements on one line; minimal ASI rules.

Imports and namespaces
- See: reference/language/using.md — `using` syntax, runner resolution, and style guidance.

Variables and scope
- See: reference/language/variables-and-scope.md — Block-scoped locals, assignment resolution, and strong/weak reference guidance.
- See: reference/language/lifecycle.md — Box lifetime, ownership (strong/weak), and finalization (`fini`) SSOT.
- See: reference/language/scope-exit-semantics.md — SSOT for DropScope (`fini {}` / `local ... fini {}`), `catch`/`cleanup` exit ordering, and ownership-transfer terminology (no `move` keyword).
- See: reference/language/repl.md — REPL mode semantics (file mode vs REPL binding rules).

Type system (SSOT)
- See: reference/language/types.md — runtime truthiness, `+`/compare/equality semantics, and the role/limits of MIR type facts.
- See: reference/language/option.md — current `Option<T>` / `Result<T,E>`
  enum prelude surface and why compiler helper no-match must not use Option on
  Stage0.
- Static const table declarations, reads, and narrow integer initializer const expressions are live for the M11b `u16[]` row. Const fn remains reserved. See reference/language/types.md “Static Const Tables (M11b live)” and `docs/development/current/main/design/static-const-table-syntax-ssot.md`.
- Low-level allocator-grade `.hako` code uses explicit capability modules,
  static tables, and Rune metadata rather than broad `unsafe` blocks. See:
  reference/language/low-level-capabilities.md.

Grammar (EBNF)
- See: reference/language/EBNF.md — living grammar reference used by parser
  implementations.
- Unified Members (stored/computed/once/birth_once): see reference/language/EBNF.md “Box Members (Phase 15)” and the Language Reference section. Stored fields use `name` for simple dynamic slots and `name: Type` when declared-type metadata helps typed-object planning / optimization / verification. Canonical computed syntax is `get name: Type { ... }`; legacy `name: Type { ... }` remains accepted. Default ON (disable with `NYASH_ENABLE_UNIFIED_MEMBERS=0`).

Member exceptions and handlers (Stage‑3)
- Postfix `catch/cleanup` may be attached to computed/once/birth_once/method blocks when Stage‑3 is enabled. Stored members (`name` or `name: Type`) do not support handlers.

Related implementation notes
- Tokenizer: src/tokenizer.rs
- Parser (expressions/statements): src/parser/expressions.rs, src/parser/statements.rs
- MIR Lowering (expressions): src/mir/builder/exprs.rs and friends

Navigation tips
- Use EBNF + topic pages as the canonical reference.
- Use LANGUAGE_REFERENCE_2025 only as a historical snapshot.
- Phase 12.7 files capture old sugar/history; current canonical additions are
  tracked by current design SSOTs and the EBNF page.
