---
Status: Draft (design SSOT; implementation phased)
Scope: parser/AST/macro/selfhost alignment for block-expr + map-literal migration
Related:
  - docs/reference/language/block-expressions-and-map-literals.md
  - docs/development/current/main/design/block-expressions-and-condition-blocks-ssot.md
  - docs/development/current/main/design/cond-block-view-prelude-ssot.md
---

# MapLiteral Eviction + BlockExpr Roadmap (SSOT)

Goal: make `{ ... }` usable as a **block expression** (in expression position) by moving map literals off `{ ... }`.

This enables condition prelude patterns without adding a dedicated `cond { ... }` surface syntax.

## Final Target (spec)

Spec SSOT (provisional until promoted):
- `docs/reference/language/block-expressions-and-map-literals.md`

Summary:
- `{ ... }` is a block (statement block and expression block).
- `%{ ... }` is the map literal (entries use `=>`).
- v1 safety: exit statements are rejected inside block **when used as an expression**.

## Migration Constraints

- No silent fallback: ambiguous parses must fail-fast with actionable errors.
- Rust parser and `.hako` selfhost parser must stay aligned (no split-brain syntax).
- Avoid env-var sprawl: use existing staging patterns; do not introduce permanent toggles for syntax.

## Phases (staged tasks)

### Phase B0: Docs + Decision (no code)

Purpose: freeze intent and boundaries before any parser work.

Deliverables:
- Spec-level doc with `Decision: provisional`
  - `docs/reference/language/block-expressions-and-map-literals.md`
- Roadmap SSOT (this file)

### Phase B1: Introduce `%{ ... }` map literal (compat mode)

Purpose: add the new map literal syntax without breaking existing code.

Tasks:
1. Rust tokenizer/parser:
   - Parse `%{ ... }` as `ASTNode::MapLiteral`
   - Accept `=>` as the entry separator (entry key rules: start with string keys only)
2. `.hako` parser (selfhost Stage-B):
   - Accept `%{ ... }` and emit equivalent JSON v0 AST
3. Update fixtures incrementally:
   - Migrate internal `.hako` sources via **text-only** replacement (`{...}` map → `%{...}`)
   - Pin at least one fast-gate fixture that uses `%{...}`

Compatibility:
- Keep legacy `{ "k": v }` map literal accepted for now.

Status: ✅ Completed

Implementation notes (SSOT pointers):
- Rust tokenizer: `TokenType::PercentLBrace` for `%{` (no whitespace), `% {` remains `MODULO` + `LBRACE`.
- Rust parser: `%{ "k" => v }` parses to `ASTNode::MapLiteral` (v1: string keys only). `:` inside `%{}` is fail-fast with an “use `=>`” message.
- Selfhost parser: `%{...}` is accepted and lowered to the same `map.of` call shape as legacy map literals.
- Fixture pinned: `apps/tests/phase29bq_map_literal_percent_min.hako` in `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`.

Drift checks:
- `rg -n "PercentLBrace" src/tokenizer src/parser | cat` → expected hits (token + parser)
- `rg -n "apps/tests/phase29bq_map_literal_percent_min\\.hako" tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv` → 1件

### Phase B2: Enable `{ ... }` as expression block (breaking switch)

Purpose: remove `{}` ambiguity in expression position.

Tasks:
1. Rust AST:
   - Add an AST node for block expressions (e.g., `ASTNode::BlockExpr { prelude_stmts, tail_expr, span }`)
2. Rust parser:
   - In expression primary position, parse `{ ... }` as `BlockExpr` (not map)
3. Verifier + lowering contract:
   - Value-required contexts must require `tail_expr` (else `[freeze:contract]`)
   - Disallow `return/break/continue/throw` in block-expr used as expression (v1)
4. `.hako` parser:
   - Emit the same `BlockExpr` node in JSON v0 AST

Breaking change:
- Legacy `{ ... }` map literal is removed (fail-fast) once the repo and canary sources are migrated.

Status: ✅ Implementation complete (B2-2..B2-6)

Implementation notes (SSOT pointers):
- MIR expr lowering: `ASTNode::BlockExpr` is lowered in `src/mir/builder/exprs.rs`.
- Fixture pinned: `apps/tests/phase29bq_blockexpr_basic_min.hako` in `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv`.
- Manual check (not fast-gate pinned): `apps/tests/phase29bq_blockexpr_legacy_map_error_min.hako` should fail-fast and mention `%{...}`.

#### B2-6: Hardening (post-B2-5)

- Recursive exit check: `contains_non_local_exit()` detects exits nested in if/loop/assignment/etc.
- Spec alignment: `tail_expr` is required (AST: `Box<ASTNode>`, not `Option`)
- Manual fixture: `apps/tests/phase29bq_blockexpr_exit_forbidden_manual_min.hako`

#### B2-7: Planner-required normalizer value lowering

Contract: treat `ASTNode::BlockExpr { prelude_stmts, tail_expr }` as a value expression by lowering:
- `prelude_stmts` → effects + bindings (same vocabulary as condition prelude)
- `tail_expr` → value (under the updated bindings)
- result → `(tail_value_id, prelude_effects + tail_effects)`

Fail-fast (v1 safety):
- Any non-local exit inside `prelude_stmts` is forbidden (`contains_non_local_exit()`), report `[freeze:contract][blockexpr]`.

Ops note:
- `HAKO_JOINIR_DEBUG=1` may print trace output that breaks stdout-based comparisons; use `HAKO_JOINIR_DEBUG=0` for gate runs.

Manual check (B2-6):
```bash
./target/release/hakorune apps/tests/phase29bq_blockexpr_exit_forbidden_manual_min.hako
# Expected: exit != 0, stderr contains "[freeze:contract][blockexpr]"
```

#### Drift check: legacy map literal absence

Run periodically to ensure no legacy map literals creep back:

```bash
rg -n --glob '*.hako' '\{\s*"[^"\n]+"\s*:' .
```

Expected: all matches should be:
- Comments describing JSON format (e.g., `// 期待値: {"op":"loopform"...}`)
- String literals (JSON being parsed, e.g., `JsonParserBox.parse('{"key": "value"}')`)
- Test files (e.g., `phase29bq_blockexpr_legacy_map_error_min.hako` which tests fail-fast)

No **code-level** `{ "k": v }` map literals should exist.

Token note: `%{` is `TokenType::PercentLBrace` (no whitespace allowed between `%` and `{`).

### Phase B3: Optional sugar (post-stability)

Purpose: improve ergonomics without expanding core semantics.

**SSOT**: `docs/development/current/main/design/block-expr-b3-sugar-decision.md`

Examples:
- `if local x = f(); x > 0 { ... }` desugars to `if ({ local x = f(); x > 0 }) { ... }`

Rules (from SSOT):
1. Sugar is **parser-level only** - parser emits `BlockExpr` directly
2. No lowering changes - MIR builder sees only `BlockExpr`
3. "Parser desugar" is distinct from "AST rewrite prohibition" (runtime transformations)

Status: **Deferred** (design anchor only, no implementation planned).

### Phase B4: Enable condition prelude under planner-required (JoinIR/plan)

Purpose: make BlockExpr-in-condition work in CorePlan/Parts paths (planner-required), not only in the direct MIR expression builder.

Status: ✅ Completed

Scope (v1):
- Only condition entry (`CondBlockView`) gains prelude lowering support.
- No accept-shape expansion: this is a BoxShape change (contract wiring), not BoxCount.
- No rewrite: prelude statements are evaluated in source order, `tail_expr` evaluated once.

SSOT entry points:
- `src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs`
  - `lower_cond_branch(...)`
  - `lower_cond_value(...)`
  - `lower_loop_header_cond(...)`

Contract (v1):
- Prelude statement vocabulary is restricted to “stmt-only effects” already supported by plan normalizer helpers.
- Exit (`return/break/continue/throw`) is forbidden anywhere in prelude (same as BlockExpr v1 contract).
- Tail expression is evaluated exactly once and then used for branch/value conversion as usual.

Acceptance:
- Add a pinned fixture that exercises the planner-required path with a condition block expression:
  - `apps/tests/phase29bq_cond_prelude_planner_required_min.hako`
- `phase29bq_fast_gate_vm.sh` remains green.

Drift checks (after):
- `rg -n \"CondBlockView prelude is not supported yet\" src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs` → 0件
- `rg -n \"cond_prelude_unsupported\" src/mir/builder/control_flow/plan/parts/verify.rs` → 0件

## Gates / Acceptance Criteria (per phase)

- Phase B1:
  - Fast gate green
  - At least one pinned `%{...}` fixture passes
- Phase B2:
  - Fast gate green after repo migration
  - `lang/src/compiler/**/*.hako` に brace-colon literal が残っていない（repo migration 完了条件）
    - `rg --pcre2 -n --glob '*.hako' '^(?!\\s*//)[^\\\"]*\\{\\s*[A-Za-z_][A-Za-z0-9_]*\\s*:' lang/src/compiler`
    - `rg --pcre2 -n --glob '*.hako' '^(?!\\s*//)[^\\\"]*\\{\\s*\"[^\"\\n]+\"\\s*:' lang/src/compiler`
  - Explicit fail-fast message for legacy `{...}` map literal
