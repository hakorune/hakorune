# JOINIR-IF-RECIPE-SSA-ADOPTION0-D0

Status: D0 queued design task; located-legacy S0/S1 retirement green — do not
wire production If yet.
Date: 2026-08-04

This card records the next cleanup target after the Loop cutover lane. It is
not a claim that If already has two, or only two, equivalent production
implementations. The first step is an authority census.

## Verified audit facts

The repository currently contains several If-producing surfaces:

```text
raw/descent path:
  src/mir/builder/if_form.rs
  src/mir/builder/stmts/if_statement_descent.rs
  block_stmt -> drive_raw_if_statement_with_port_v1

CorePlan/JoinIR path:
  control_flow/plan/lowerer/plan_lowering.rs
  control_flow/plan/features/if_join.rs
  control_flow/plan/parts/dispatch/if_join.rs
  if_branch_lowering.rs / if_general.rs / if_exit.rs

resolved source-bound path:
  resolved_lowering/located_if.rs
  resolved_lowering/if_materialization.rs (IfCfgSessionV1)
```

`if_statement_parity_tests.rs` exercises the raw IfForm boundary and is useful
parity evidence, but its existence alone does not prove that the two paths
have identical authority or that one is test-only. The resolved path adds a
third surface that must be classified separately.

The current canonical lifecycle is:

```text
CanonicalSsaFunctionSessionV2
  = Binding SSA + CanonicalCfgSessionV1 + one PhiTxn
```

It is the SSOT for the canonical resolved lane. It is not yet the sole writer
for every If/Loop/JoinIR production edge. `IfCfgSessionV1`, the plan If join
materializers, raw `IfForm`, legacy PHI repair, and any JoinIR inline writer
remain execution surfaces until their callers are retired.

## Non-claims and boundary

Do not claim any of the following before the census is closed:

- that `if_form.rs` and CorePlan are the only If authorities;
- that `located_legacy_*` is wholly dead or safe to delete;
- that all PHI/CFG writers already use the canonical session;
- that a portable `IfRecipeV1` is already consumed by production;
- that the feedback's line-count reduction is a safe deletion estimate.

`LocatedLegacyLoweringSessionV1::verify` currently appears test-only, while
related located/raw carriers still have production-facing references. The
cleanup task is therefore scoped to a caller census and test migration proof,
not a blanket `located_legacy_*` deletion.

## Located legacy S0 census (2026-08-04)

The dedicated guard
`tools/checks/joinir_located_legacy_retire_guard.sh` is green and fixes the
retirement boundary:

```text
LocatedLegacyLoweringSessionV1::verify calls = 48, all test-only
production-root constructors                        = 0
non-test module/re-export roots                     = builder.rs only
internal adapter selector/lower pairs               = 6, session-local only
source/claim carriers                               = retained and still separate
test oracles                                        = 2 callable-result modules present
all located components                              < 800 lines
```

This census has now been followed by the bounded S1 retirement. The carrier
`callable_result_representation::located_legacy` and its caller-ledger/
loop-claim consumers remain in scope. No normal/raw ingress, IfCfgSession, or
resolved A+ owner was changed.

## Located legacy S1 closeout (2026-08-04)

The disconnected session and its test-only adapters were removed after the
caller-zero proof; no production caller was introduced to preserve the old
oracles. Deleted scope is limited to:

```text
builder/located_legacy_lowering.rs
builder/located_legacy_{assignment,if,return}.rs
builder/located_legacy_*_tests.rs
callable_result_representation/tests/{located_legacy_lowering,located_short_circuit_lowering}.rs
builder.rs module/re-export and callable-result test-module registrations
```

The live located carrier/ledger/claim files remain. The post-retirement guard
and focused compile/test checks are green:

```text
bash tools/checks/joinir_located_legacy_retire_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q
RUSTFLAGS='-Awarnings' cargo test -q callable_result_representation --lib
RUSTFLAGS='-Awarnings' cargo test -q nested_predicate_profile --lib
```

Mixed raw/descent guards are intentionally retained for the later raw-parity
retirement lane; they are not evidence that the deleted located session still
exists.

## Ordered task sequence

### D0-A — authority and caller census (design only)

Inventory every production and test caller for:

```text
IfForm / if_statement_descent
CorePlan::If / plan If join helpers
IfCfgSessionV1 / resolved located_if
PhiMergeHelper / emission::phi / phi_input_materializer
route-local If/Loop PHI materializers
JoinIR inline PHI/CFG writers
json_v0 bridge writers
located legacy sessions and raw child carriers
```

For every surface record owner, input contract, mutation boundary, and
whether it can be reached from an unpublished compile candidate. This is a
BoxShape task: no new accepted source shape and no production wiring.

### D0-B — portable IfRecipeV1 contract

Design one recursive semantic product, analogous to the Loop recipe, with:

```text
condition
then block
optional else block
branch-transfer obligations
join/exit shape
carrier/merge obligations
source provenance without AST, Builder, or physical IDs
```

The recipe is the semantic boundary; existing raw/plan/resolved structures are
parity oracles until a named producer and consumer are proven. Control flow
must remain in the recursive block algebra; leaf operations do not contain a
nested If or Loop.

### D0-C — one canonical production consumer

After D0-A/B, select one exact If shape and connect it through the existing
resolved source-bound candidate chain:

```text
one selection/preflight
  -> sealed IfRecipeV1
  -> CanonicalSsaFunctionSessionV2
  -> CanonicalCfgSessionV1 + PhiTxn
  -> one If merge physicalizer
  -> unpublished compile candidate
```

Do not create a second SSA/PHI transaction, do not connect directly to a raw
route registry, and do not preserve post-effect route retry at this seam.
Unsupported branch-transfer shapes must be typed rejects until their JoinSig
obligations are closed; the physicalizer must not repair missing predecessors
or invent PHI inputs.

### D0-D — canonical PHI/CFG adoption

For the selected If shape, make the canonical session the only production
writer for its branch blocks, merge block, predecessor seals, and PHI commit.
Classify and retire only the selected old writer in the same cutover. Then
repeat for remaining loop-variant and JoinIR writers. This is adoption of the
existing owner, not a new SSA design.

### D0-E — cheap cleanup, independently gated

S0 and the bounded S1 deletion are complete. The disconnected session,
adapters, test-only module registrations, and dedicated test modules are
gone; the carrier and its semantic evidence remain. S1 did not modify
normal/raw ingress, `located_if.rs`, `IfCfgSessionV1`, or the portable IfRecipe
design. Any future located lowering requires a new candidate-scoped caller
and a new design row.

This cleanup must not be mixed with If BoxCount or PHI owner adoption. The
raw/descent/parity trio for local/return/assignment/binary/short-circuit is a
later retirement lane after If parity is green.

## Acceptance gates

```text
D0-A: every If/PHI/CFG writer has one classified owner and caller set
D0-B: IfRecipeV1 has no AST/Builder/ValueId/BasicBlockId ownership
D0-C: selected recipe producer = exactly 1; physicalizer = exactly 1
D0-C: selected caller is inside an unpublished compile candidate
D0-D: selected old If/PHI writer caller = 0 after cutover
D0-D: selected shape has no post-effect Option/Retry/reselection
D0-D: legacy/new semantic digest, MIR/CFG, PHI, diagnostics, and reuse parity green
D0-D: injected late failure leaves live Builder/candidate owner unchanged
D0-E/S0: 48 test-only verifies, zero production roots, and retained carrier
D0-E/S1: only the proven disconnected session/adapters/tests are deleted; the
         post-retirement guard and focused checks are green
all touched Rust/test files < 800 lines
```

## Relationship to the Loop lane

This is queued after the active Nested D5-I0 consultation and the remembered
Loop convergence task:

```text
JOINIR-LOOP-RECURSIVE-FRAME-CONVERGENCE0-M12
```

If adoption may share the canonical `JoinSig` branch-transfer/merge owner,
but it must not reopen Generic post-effect debt or silently broaden Nested I0.
The Loop lane's bounded Nested work and the Located S0/S1 retirement are now
closed. This card is the next design target, not an implementation
authorization.
