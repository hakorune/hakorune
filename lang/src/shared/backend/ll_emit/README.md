# shared/backend/ll_emit

目的
- backend owner cutover の compare/debug lane に使う `.hako ll emitter` をここに置く。
- canonical seam は MIR のまま維持し、compat `lang/src/llvm_ir/**` を daily owner に戻さない。

Current scope
- `runtime_decl_registry_box.hako`
  - backend-private runtime declare truth の consumer
- `recipe_facts_v0_box.hako`
  - analysis-only facts sidecar
- `ll_text_emit_box.hako`
  - narrow textual LLVM IR emitter
- `driver.hako`
  - explicit compare lane entry

Current subset
- `const`
- `copy`
- `binop`
- `compare`
- `select`
- `branch`
- `phi`
- `ret`
- direct `Extern` call

Non-goals
- daily owner flip
- `AST -> LLVM` direct route
- compat `lang/src/llvm_ir/**` revive
- broad fast-leaf widening
