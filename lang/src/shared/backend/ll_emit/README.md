# shared/backend/ll_emit

目的
- backend owner cutover の compare/debug lane に使う `.hako ll emitter` をここに置く。
- canonical seam は MIR のまま維持し、compat `lang/src/llvm_ir/**` を daily owner に戻さない。
- boundary-only first wave では narrow shape の daily owner もここへ寄せる。

Current scope
- `runtime_decl_registry_box.hako`
  - backend-private runtime declare truth の consumer
- `recipe_facts_v0_box.hako`
  - analysis-only facts sidecar
- `ll_text_emit_box.hako`
  - narrow textual LLVM IR emitter
- `driver.hako`
  - compare/daily bridge entry template

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
- narrow `Global print`

Non-goals
- `AST -> LLVM` direct route
- compat `lang/src/llvm_ir/**` revive
- broad fast-leaf widening

Current migration rule
- compare lane は temporary bridge だよ。
- flipped boundary shapes (`ret const`, `bool phi/branch`, `Global print`, `concat3 extern`) のみ narrow daily owner として許可する。
- dead residue は ledger に残してから削除する。
