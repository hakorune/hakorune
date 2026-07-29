# MIRCOMPILER-PUBLIC-PROGRAM-ADMISSION0 — Design Decision

Date: 2026-07-29  
Decision: accepted  
Ceremony: T2  
Next row: `MIRCOMPILER-PUBLIC-PROGRAM-ADMISSION0-I0-R0`

## Outcome

Keep the existing public method names and signatures:

```text
MirCompiler::compile
MirCompiler::compile_with_source
MirCompiler::compile_with_source_and_imports
```

Narrow their source contract to one whole-file `Program` and route that
Program through the existing typed normal lifecycle.

```text
ASTNode
-> exact Program admission once
-> NormalCompileRequestV1
-> compile_normal
-> existing candidate / finish / atomic commit
```

A non-Program root fails before Builder effects with the existing typed
Program-root diagnostic. It is not wrapped in a synthetic Program and is not
retried through Legacy lowering.

`compile_legacy` must not remain as an arbitrary-AST escape hatch. Existing
callers are zero, so it is removed rather than renamed or forwarded.

## Why this decision

Fresh repository census found:

```text
internal runner callers of public arbitrary compile APIs = 0
production compiler -> build_module edges                = 1
active external integration callers                      = Program only
concrete external non-Program MirCompiler callers        = 0
```

Immediate method deletion would cause unnecessary source breakage for Program
embedders. Keeping exact arbitrary-AST behavior would retain the final
compiler-owned production `build_module` edge indefinitely.

Program-only narrowing preserves the public source shape while making the
whole-file compiler contract explicit.

## Authority boundary

This row owns:

```text
public MirCompiler whole-file Program admission
source hint transport
exact imports transport
optimize / REPL / quiet config snapshot
normal finish/result policy
success-only external commit
non-Program fail-fast before Builder
```

This row does not own:

```text
generic MacroBox AST -> AST tooling
AST JSON node transport
NarrowV1 raw published ingress
LegacyModuleLoweringInputV1 used as a source-neutral raw carrier
MirFinishScheduleV1::Legacy used by the normal lifecycle
public MirBuilder::build_module contract
Ownership / View or another language feature
```

## Exact implementation

Add one private adapter in `MirCompiler`:

```rust
fn compile_public_program(
    &mut self,
    ast: ASTNode,
    source_file: Option<&str>,
    imports: HashMap<String, String>,
) -> Result<MirCompileResult, String>;
```

It constructs the existing `NormalCompileRequestV1::for_mir_mode` request and
calls `compile_normal` exactly once.

The public methods delegate only to this adapter:

```text
compile
  -> empty source + empty imports

compile_with_source
  -> exact source + empty imports

compile_with_source_and_imports
  -> exact source + exact imports
```

Do not add a public Program API, deprecated arbitrary-AST facade, feature flag,
or compatibility route.

## Atomic delete set

The same commit deletes:

```text
MirCompiler::compile_legacy
MirCompiler::compile_legacy_request
MirLoweringRequestV1::Legacy
MirLoweringRequestErrorV1::Legacy
compile_request route selector when no longer shared
MirCompiler::compile_legacy_candidate
src/mir/compiler/legacy_candidate_session.rs
compiler -> session.builder_mut().build_module(ast)
MIRCOMPILER-ARBITRARY-AST-COMPAT-SUNSET-001 production edge
```

`compile_resolved` calls its canonical owner directly after the selector is
removed.

Do not delete `LegacyModuleLoweringInputV1` in this row. The typed Raw pipeline
still uses it as a source-neutral carrier. Remove its public re-export and
narrow its visibility to the internal Raw owner chain because no public
constructor consumes it after `compile_legacy` is deleted.

## Test migration

Most current `compile*` tests already pass Program roots and should remain
green without call-site churn.

Four compiler-level non-Program Legacy observations must not keep a second
test facade:

```text
non-Program success inventory
Local root failure
GroupedAssignment root failure
Index root failure
```

Replace their compiler-ingress claim with:

```text
public Program admission rejects before Builder
live Builder remains unchanged
fresh Program compile succeeds
```

Raw node behavior stays owned by existing responsibility-local root/descent
tests.

Active integration tests and the `wasm-backend` benchmark use parser-produced
Programs and must remain green.

## Acceptance

```text
public method names/signatures                 = unchanged
Program execution                              = existing typed normal once
non-Program Builder effects                    = 0
source/import transport                        = exact
fallback / retry / route reselection           = 0

compile_legacy definition/callers              = 0
compile_legacy_candidate                       = 0
compiler legacy request enum arm               = 0
compiler production build_module edge          = 0
legacy_candidate_session.rs                    = absent
public LegacyModuleLoweringInputV1 export       = 0

generic MacroBox / AST JSON behavior            = unchanged
NarrowV1 raw ingress                            = unchanged
global MirBuilder::build_module symbol          = non-claim
new public compatibility API                    = 0
new test Legacy facade                          = 0
new language / Ownership / View behavior        = 0
```

Required gates:

```text
cargo test --lib legacy_candidate_session_tests
cargo test --test method_id_inject_filebox
cargo test --test wasm_demo_min_fixture
cargo check --tests
cargo check --features wasm-backend
shared public-ingress guard
MirBuilder replacement guard
```

Use the exact active integration target names discovered by `cargo test
--test` enumeration if a module path is not a standalone target.

## Hard stops

Stop and return to D0 if implementation requires:

```text
wrapping a non-Program node in Program
Program rejection followed by Legacy retry
a new compile_arbitrary_ast_compat API or feature
different source/import/config/result behavior
keeping compile_legacy as a public escape hatch
deleting LegacyModuleLoweringInputV1 raw-pipeline receipts
changing MacroBox or AST JSON generic-node contracts
deleting public MirBuilder::build_module in the same row
adding Ownership / View or another language feature
```

## Following row

After the compiler edge reaches zero, open:

```text
MIRBUILDER-PUBLIC-ROOT-API0-D0
```

That D0 separately decides the remaining public/test-only
`MirBuilder::build_module` contract and its three direct test authorities.
It must not inherit the MirCompiler decision without its own caller census.
