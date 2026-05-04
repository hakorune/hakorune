# Nyash Language Reference – Index

This is the entry point for Nyash language documentation.

- Full Language Reference (2025): reference/language/LANGUAGE_REFERENCE_2025.md
- Syntax Cheat Sheet: quick-reference/syntax-cheatsheet.md
- Phase 12.7 Grammar Specs (ternary, sugar; peek → match に統合):
  - Overview: archive/roadmap/phases/phase-12.7/grammar-specs/README.md
  - Token/Grammar: archive/roadmap/phases/phase-12.7/ancp-specs/ANCP-Token-Specification-v1.md
- Sugar Transformations (?., ??, |> and friends): parser/sugar.rs (source) and tools/nyfmt/NYFMT_POC_ROADMAP.md
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
- See: reference/language/option.md — public `Option<T>` direction and why compiler helper no-match must not use Option on Stage0.

Grammar (EBNF)
- See: reference/language/EBNF.md — Stage‑2 grammar specification used by parser implementations.
- Unified Members (stored/computed/once/birth_once): see reference/language/EBNF.md “Box Members (Phase 15)” and the Language Reference section. Canonical computed syntax is `get name: Type { ... }`; legacy `name: Type { ... }` remains accepted. Default ON (disable with `NYASH_ENABLE_UNIFIED_MEMBERS=0`).

Member exceptions and handlers (Stage‑3)
- Postfix `catch/cleanup` may be attached to computed/once/birth_once/method blocks when Stage‑3 is enabled. Stored members (plain `name: Type [= expr]`) do not support handlers.

Related implementation notes
- Tokenizer: src/tokenizer.rs
- Parser (expressions/statements): src/parser/expressions.rs, src/parser/statements.rs
- MIR Lowering (expressions): src/mir/builder/exprs.rs and friends

Navigation tips
- The “reference/language/LANGUAGE_REFERENCE_2025.md” is the canonical long‑form reference; use the Cheat Sheet for quick syntax lookup.
- Phase 12.7 files capture the finalized sugar and new constructs (match (replaces peek), ternary, null‑safe).
