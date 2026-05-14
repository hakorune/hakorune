# AST Cleanup Before Local Type SSOT

Status: active

## Decision

Decision: accepted.

Before continuing `LOCALTYPE-001`, remove low-risk AST legacy residue that would
make new local type metadata harder to reason about.

## Rows

| Row | Scope | Type |
| --- | --- | --- |
| `ASTCLEAN-001 legacy AST enum removal` | Remove unused `StructureNode` / `ExpressionNode` / `StatementNode` and collapse `ASTNodeType` classification to direct `is_expression()` matching. | BoxShape cleanup |
| `ASTCLEAN-002 normalize logical ops helper` | De-duplicate `normalize_logical_ops` in `src/parser/mod.rs`. | BoxShape cleanup |
| `ASTCLEAN-003 parser depth no-op hook removal` | Remove legacy parser depth no-op hooks. | BoxShape cleanup |
| `ASTCLEAN-004 dead-code allowance inventory` | Inventory `#[allow(dead_code)]` and install no-growth / reason policy. | docs/guard |

## Stop lines

- Do not change language surface.
- Do not remove `Outbox`, `this`, or Stage-3 compatibility tokens in this phase.
- Do not delete runtime/backend stubs without an owner-specific card.
- Do not mix this cleanup with `LOCALTYPE-001` implementation.

## Follow-up prune rows

- ASTCLEAN-005 MIR TypeRegistry dead_code allow prune | Status: complete
  - Scope: only `src/mir/builder/type_registry.rs`.
  - Contract: used entries do not carry `#[allow(dead_code)]`; retained allowances must explain their staged/debug role.

- ASTCLEAN-006 MIR numeric substrate dead_code rationale guard | Status: complete
  - Scope: only `src/mir/numeric_substrate.rs`.
  - Contract: staged exact-numeric allowances must carry row/rationale comments.

- ASTCLEAN-007 MIR loops duplicate dead_code allow prune | Status: complete
  - Scope: only duplicate adjacent allowances in `src/mir/builder/loops.rs`.
  - Contract: staged helpers may keep one allowance, but duplicate attributes are forbidden.

- ASTCLEAN-008 test/dev helper dead_code allowance prune | Status: complete
  - Scope: selected test/dev helper files from the ASTCLEAN-004 inventory.
  - Contract: retained helper allowances must carry `ASTCLEAN-008` rationale comments; removed VM legacy benchmark stub must not return.

- ASTCLEAN-009 backend/optimizer utility dead_code allowance prune | Status: complete
  - Scope: selected backend utility files plus stale `src/mir/optimizer/diagnostics.rs`.
  - Contract: used helpers do not carry `dead_code`; retained backend utility allowances carry `ASTCLEAN-009`; optimizer diagnostics are owned by `optimizer_passes`.

- ASTCLEAN-010 runner backend JSON bridge helper prune | Status: complete
  - Scope: runner MIR JSON emit helpers and JSON v0/v1 bridge wrappers from the ASTCLEAN inventory.
  - Contract: stale wrapper helpers stay deleted; active scoped/with-vars owners remain.

- ASTCLEAN-011 runner exec stale dead_code allowance removal | Status: complete
  - Scope: only `src/runner/modes/common_util/exec.rs` live backend runner APIs.
  - Contract: runner exec product APIs stay present without stale `dead_code` allowances.

- ASTCLEAN-012 host provider compare bridge dead_code rationale guard | Status: complete
  - Scope: only `src/host_providers/llvm_codegen.rs` staged hako-ll module allowances.
  - Contract: keep/archive decision is explicit; no bare host-provider `dead_code` allowance may return.

- ASTCLEAN-013 MIR builder utility dead_code allowance prune | Status: complete
  - Scope: stale `src/mir/builder/loops.rs` helper shelf only.
  - Contract: active loop APIs remain elsewhere; the stale helper module must not return.

- ASTCLEAN-014 MIR builder scope/local utility dead_code prune | Status: complete
  - Scope: `scope_context.rs` loop helper methods and `utils/local_ssa.rs` wrapper allowances.
  - Contract: live LocalSSA wrappers stay; stale loop helper methods and unused compare wrapper stay deleted.

- ASTCLEAN-015 MIR builder utility shelf prune | Status: complete
  - Scope: weak-ref/barrier helpers, pinning wrappers, schedule copy helpers, call-emission wrapper delegates, and stale `utils/type_ops`.
  - Contract: live helpers stay without stale allowances; unused wrapper shelves stay deleted.

- ASTCLEAN-016 MIR builder call resolution duplicate helper prune | Status: complete
  - Scope: duplicate warning helper functions in `src/mir/builder/call_resolution.rs` only.
  - Contract: call-resolution live helpers remain; duplicate warning helpers stay owned by `calls/method_resolution.rs`.

- ASTCLEAN-017 runner/provider/runtime dead_code rationale pass | Status: complete
  - Scope: runner child/process helpers, provider registry surfaces, plugin loader v2 diagnostic shelves, and resolver metadata fields from the ASTCLEAN inventory.
  - Contract: live runner/provider APIs do not carry stale `dead_code`; retained optional/diagnostic shelves must carry `ASTCLEAN-017` rationale comments.
