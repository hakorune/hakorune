# Hakorune Language Reference – Index

This is the entry point for the current Hakorune language reference.
Status: Current navigation index. Feature availability is defined in the
[language feature status index](status-index.md); this page does not infer
parser support from examples or historical prose.

Start with the [language feature status index](status-index.md). It separates
grammar status from Stage0/Stage1/concurrency availability and records known
conflicts without making parser support guesses.

Current canonical surface:

- Semantic contract charter: [semantic-contract-charter.md](semantic-contract-charter.md)
- Semantic kernel: [semantic-kernel.md](semantic-kernel.md)
- Function exit, Script result, and entry/process-result boundaries:
  [function-exit-and-entry-result.md](function-exit-and-entry-result.md)
- Grammar contract: [grammar-contract.md](grammar-contract.md)
- Minimal surface policy: [language-minimal-surface-ssot.md](../../development/current/main/design/language-minimal-surface-ssot.md)
- Grammar and accepted rows: [EBNF.md](EBNF.md)
- Bootstrap / phase-1 usable surface profiles: see the profile manual below.
- Feature status: [status-index.md](status-index.md)
- Practical syntax summary: [quick-reference.md](quick-reference.md)
- Ownership / aliasing / explicit Shared boundary:
  [ownership.md](ownership.md)
- Type and enum surface: [types.md](types.md)
- Record vs Box / Object Storage: [types.md](types.md) “Record vs Box”,
  [record-box-two-surface-one-substrate-ssot.md](../../development/current/main/design/record-box-two-surface-one-substrate-ssot.md),
  and [object-storage-plan-boundary-ssot.md](../../development/current/main/design/object-storage-plan-boundary-ssot.md)
- Option / Result enum prelude: [option.md](option.md)
- Failure/Outcome relations: [failure-outcome-relations.md](failure-outcome-relations.md)
- Selector-independent Dynamic invocation contract:
  [dynamic-invocation.md](dynamic-invocation.md)
- Rune declaration metadata: [runes.md](runes.md)
- Declared whole-call query contracts: [callable-contracts.md](callable-contracts.md)
- Build conditional `gate`: [build-conditional-gate.md](build-conditional-gate.md)
- Low-level language surface: [low-level-capabilities.md](low-level-capabilities.md)
- Concurrency / Thread Boundary: [semantics.md](../concurrency/semantics.md),
  [boundary-model.md](../concurrency/boundary-model.md), and
  [threading.md](../runtime/threading.md)
- Delegation and no-inheritance rule: [field-visibility-and-delegation.md](field-visibility-and-delegation.md)
- Language v1 execution order:
  [language-v1-convergence-current.md](../../development/current/main/workstreams/language-v1-convergence-current.md)

Historical references:

- [LANGUAGE_REFERENCE_2025.md](LANGUAGE_REFERENCE_2025.md) is a historical snapshot. It is
  useful for old Phase 12/15 context, but it is not the current canonical source
  when it conflicts with the files above.
- [syntax-cheatsheet.md](../../quick-reference/syntax-cheatsheet.md) is a historical stub that redirects to
  [quick-reference.md](quick-reference.md).
- [language-guide.md](../../guides/language-guide.md) and [language-core-and-sugar.md](../../guides/language-core-and-sugar.md) are historical
  stubs. They are not permission to use legacy sugar or inheritance surfaces.
- Phase 12.7 Grammar Specs (ternary, sugar; peek → match に統合):
  - Overview: [grammar-specs/README.md](../../archive/roadmap/phases/phase-12.7/grammar-specs/README.md)
  - Token/Grammar: [ANCP-Token-Specification-v1.md](../../archive/roadmap/phases/phase-12.7/ancp-specs/ANCP-Token-Specification-v1.md)
- Legacy sugar notes (?., ??, |> and friends): `parser/sugar.rs` (source) and `tools/nyfmt/NYFMT_POC_ROADMAP.md`.
  These are not permission to add new canonical surfaces.
- Match Expression (pattern matching): see the Language Reference and EBNF (peek was replaced by match)

Statement separation and semicolons
- See: [statements.md](statements.md) — newline as primary separator; semicolons optional for multiple statements on one line; minimal ASI rules.

Imports and namespaces
- See: [using.md](using.md) — `using` syntax, runner resolution, and style guidance.

Variables and scope
- See: [variables-and-scope.md](variables-and-scope.md) — Block-scoped locals,
  assignment resolution, and Home/handle/weak binding guidance.
- See: [ownership.md](ownership.md) — SSOT for Home slots/tokens, ordinary
  handles, declaration `take`, non-group postfix `share`, result relations,
  canonical whole-root statement `release root`, and callable Home ABI.
  Generic/composite release remains provisional; accepted target spellings
  become parser-live only when their Rust/Hako implementation rows land.
- See: [lifecycle.md](lifecycle.md) — terminal Home finalization, weak-only
  tombstones, non-callable Box `fini {}` hooks, and structural drop SSOT.
- See: [constructor-birth-new-lifecycle-ssot.md](../../development/current/main/design/constructor-birth-new-lifecycle-ssot.md) — `new` / field initializer / `birth` construction order, direct `birth` call rejection, and explicit reuse method policy.
- See: [scope-exit-semantics.md](scope-exit-semantics.md) — SSOT for one
  standalone `cleanup {}`, Result-only `?` exit ordering, terminal Home
  release, and accepted Home-transfer/`share` terminology. Scope-`fini`,
  catch, and postfix/local cleanup are retirement input. Parser-live status
  remains owned by EBNF/grammar rows.
- See: [repl.md](repl.md) — REPL mode semantics (file mode vs REPL binding rules).
  Current interactive implementation work is parked by
  [vm-active-lane-retirement-ssot.md](../../development/current/main/design/vm-active-lane-retirement-ssot.md) and
  [repl-mir-interpreter-interactive-session-ssot.md](../../development/current/main/design/repl-mir-interpreter-interactive-session-ssot.md).

Type system (SSOT)
- See: [types.md](types.md) — runtime truthiness, `+`/compare/equality semantics, and the role/limits of MIR type facts.
- See: [option.md](option.md) — current `Option<T>` / `Result<T,E>`
  enum prelude surface and why bootstrap compiler helper no-match must not use
  Option.
- Static const table declarations, reads, and narrow integer initializer const expressions are live for the M11b `u16[]` row. Const fn remains reserved. See [types.md](types.md) “Static Const Tables (M11b live)” and [static-const-table-syntax-ssot.md](../../development/current/main/design/static-const-table-syntax-ssot.md).
- Low-level allocator-grade `.hako` code uses explicit capability modules,
  static tables, and Rune metadata rather than broad `unsafe` blocks. See:
  [low-level-capabilities.md](low-level-capabilities.md) and [runes.md](runes.md).
- Build-time conditional code uses `gate Build... { ... }`, not C-style
  token preprocessing. Member-level `gate` is accepted inside box bodies for
  paired declaration branches with the same public signature, and
  statement-level `gate` is accepted inside method bodies as a build-time
  branch selector. `@rune Gate(...)` is accepted as top-level single-
  declaration sugar only. See [build-conditional-gate.md](build-conditional-gate.md).

Grammar (EBNF)
- See: [EBNF.md](EBNF.md) — living grammar reference used by parser
  implementations.
- See: [stage-profiles.md](stage-profiles.md) — practical support manual for
  what bootstrap readers may carry and what phase-1/selfhost code may rely on.
  It is not a second grammar.
- Box members: the accepted target is field-or-method only. Stored fields use
  `name`, `name: Type`, and optional `= expr`; weak fields remain a separate
  non-owning storage relation. Computation is an ordinary `name()` method.
  Production still accepts legacy computed/once/birth_once Property forms
  until the parked retirement pack lands; do not add new uses. See
  [EBNF.md](EBNF.md) “Box Members” and the
  [retirement SSOT](../../development/current/main/design/box-member-field-method-surface-ssot.md).

Failure, cleanup, and finalization
- Recoverable failure is `Result<T,E>`; typed Result-only postfix `?` is the
  accepted unchanged-propagation target. Option `?`, catch, and
  `RecoverableFailure` are rejected in v1.
- standalone `cleanup {}` is the sole lexical cleanup target. Box-member
  `fini {}` is a non-callable terminal Home hook; direct `obj.fini()` retires.
  `release root` is the sole early whole-root Home-end target; ordinary
  `release(value)` has no Home authority and `drop` is not an alias.
  Current handler-tail/TryCatch/legacy QMark paths are migration evidence, not
  a profile owner, and production activation remains 0.

Related implementation notes
- Frontend AST schema: crates/hakorune_frontend_ast/src/
- Tokenizer: src/tokenizer/
- Parser (expressions/statements): src/parser/expressions/, src/parser/statements/
- MIR Lowering (expressions): src/mir/builder/exprs.rs and related modules

Navigation tips
- Use EBNF + topic pages as the canonical reference.
- Use LANGUAGE_REFERENCE_2025 only as a historical snapshot.
- Phase 12.7 files capture old sugar/history; current canonical additions are
  tracked by current design SSOTs and the EBNF page.
