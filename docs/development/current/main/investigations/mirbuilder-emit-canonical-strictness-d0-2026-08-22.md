Status: design stop; pilot task queued, placement issuer is not yet complete
Task: MIR-EMIT-CANONICAL-STRICTNESS-D0
Date: 2026-08-22
Priority: Medium-High
Owner: `src/mir/builder/builder_emit.rs`
NextCard: MIR-EMIT-CANONICAL-COMPARE-I0 (blocked until this D0 exits design_stop)
---

# MIRBuilder canonical emission strictness D0

## Six-line brief

Decision: keep one final physical writer, but separate canonical verified
placement from legacy repair before either path reaches that writer. The
separation is a private prepared-request type, not a runtime boolean or a
second instruction appender.

Source authority + canonical issuer: for the first pilot,
`PreparedLoopOperationEmissionV1` plus `LoopPhysicalBlockReceiptV1` is the
strongest existing placement candidate; `issue_target_for_pure()` is its sole
adapter into `VerifiedLoopOperationTargetBlockV1`. It is not yet a complete
canonical placement issuer because sealed-state, operand definition/dominance,
and direct canonical-CFG coupling are still missing. The eventual
`MirBuilder::commit_prepared_emission` remains the sole physical commit owner.
The legacy compatibility facade issues only an explicitly repair-permitted request.

Non-authority: ambient `current_block`, `ensure_block_exists`, AST/name lookup,
`variable_map`, LocalSSA scans, `phi_input_materializer`, debug flags, and
`builder_emit.rs` itself cannot turn an unverified instruction into canonical
placement.

Fail-fast boundary: all canonical block, operand, receiver, PHI, and CFG checks
must finish before the first MIR mutation. A canonical reject cannot retry via
the legacy repair request or silently create a target block.

Smallest next slice: pilot the existing Loop physicalizer's explicit-block
`LoopOperationV1::CompareI64` through the typed canonical request, consuming
`VerifiedLoopOperationTargetBlockV1` without re-pairing its block by raw ID.
Call/receiver, PHI, and legacy callers remain on their current route.

Non-claims: no assignment change, A/C, Recipe/Join, backend, performance
optimization, `EmitReceipt`, whole-writer migration, or old-route retirement.

## Why this design stop exists

The current `builder_emit.rs` entrypoint is a single physical writer, which is
the correct ownership direction, but it performs two different jobs without a
type boundary:

```text
canonical caller
  -> ordinary emit_instruction
       -> hidden receiver materialization
       -> hidden PHI input materialization
       -> missing-block creation through ensure_block_exists
       -> instruction append

legacy caller
  -> the same ordinary emit_instruction
       -> the same repair behavior
```

The code comments describe missing-block emission as fail-fast, while the
current `ensure_block_exists` implementation creates a missing block. The
first assignment negative fixture exposed this exact distinction: an invalid
block number did not fail because the helper repaired it by creating the
block. That behavior is not changed in this D0; it is the reason canonical
strictness needs a separate entry contract.

The writer also clones debug/function names and the instruction on the normal
success path. Those are recorded under the separate
`MIR-EMIT-MOVE-COMMIT-R0` performance row and must not be smuggled into this
semantic/authority split.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| canonical source/Facts/Recipe producer | operation meaning, source relation, selected physical demand | ambient repair or MIR append |
| `CanonicalSsaFunctionSessionV2` / `CanonicalCfgSessionV1` | function owner, block allocation/selection, CFG edge and seal evidence | AST lookup, legacy repair, type reinterpretation |
| route-specific canonical physicalizer | verified operation placement, prepared operand/receiver/PHI inputs for its cohort | fallback, second writer, source re-search |
| legacy compatibility lowerer | explicit repair-permitted request and compatibility behavior | canonical claim or silent canonical downgrade |
| `MirBuilder::commit_prepared_emission` | one instruction append plus already-prepared bookkeeping | deciding route, creating semantic facts, repairing inputs |
| outer function session | discard/restore of a failed unpublished draft | retrying another emission mode |

The canonical issuer is not a new generic constructor in `builder_emit.rs`.
Each selected canonical physicalizer must pass the existing route-owned proof
through one private adapter. If a route has no owner that can issue the
placement/operand evidence, it remains `NoSafeSlice`; ambient Builder state is
not promoted to authority.

For the selected pilot, the existing chain is concrete:

```text
PreparedLoopOperationEmissionV1
  + ReadyLoopEntryV1
  + LoopPhysicalBlockReceiptV1
  -> issue_target_for_pure()
  -> VerifiedLoopOperationTargetBlockV1
  -> validate_function(builder)
  -> strict canonical emission request
```

`VerifiedLoopOperationTargetBlockV1` is not a new source authority: it is the
already-issued placement product consumed by the Loop physicalizer. The pilot
must preserve its owner, loop/item, logical-block, role, and physical-block
relation through the new request; it may not reconstruct a target from the
`BasicBlockId` alone. D0 remains open until the target witness is extended or
co-sealed with the missing strict facts without making `builder_emit.rs` their
issuer.

## Worker read-only audit checkpoint — 2026-08-22

The requested read-only audit confirmed the following:

- generic `emission::compare::emit_to_at` is not a canonical placement issuer;
- the Loop target chain is the best bounded pilot candidate, but its current
  target is `Clone + Copy`, does not reject sealed blocks, does not prove
  operand definition/dominance, and is not directly coupled to
  `CanonicalCfgSessionV1`;
- the one-writer prepared-request boundary can preserve failure atomicity only
  when preparation completes all fallible checks, commit bookkeeping is
  infallible (or covered by the outer unpublished-session discard), and no
  canonical reject enters legacy repair;
- `emit_prepared_pure_operation_at_target_v1` still has fallible result-type
  and value-ledger checks after the physical leaf emission, so those checks
  must move into preparation or be covered explicitly before I0.

The worker made no file changes, commits, pushes, or heavy-gate claims.

## Proposed request boundary

This is a contract sketch, not an implementation instruction:

```rust
enum PreparedEmissionRequestV1 {
    Canonical(PreparedCanonicalEmissionV1),
    Legacy(PreparedLegacyEmissionV1),
}

impl PreparedEmissionRequestV1 {
    fn commit(self, builder: &mut MirBuilder) -> Result<CommittedEmissionV1, EmissionErrorV1>;
}
```

The constructor and fields remain private to the selected adapters. The
canonical variant has no repair options and no `Option`-shaped missing-block,
receiver, or PHI state. The legacy variant carries an explicit repair plan;
repair is visible in the type and cannot be reached by a canonical error arm.

The public-in-module facades may be named:

```text
emit_canonical_prepared(request)
emit_legacy(instruction)
```

Both delegate to the one private `commit` implementation. There must be one
append site for `BasicBlock::add_instruction_with_span`, one predecessor
update owner, and one post-success origin/metadata owner. A pair of wrappers
is acceptable; a pair of physical writers is not.

## Canonical preparation contract

Before `PreparedCanonicalEmissionV1` can be issued, its route owner must
co-seal the following facts for the same function/session:

```text
function identity and current canonical session owner
explicit target block exists in the function
target block is not terminated or sealed
instruction operands are defined and satisfy the route's dominance contract
MethodCall receiver is already prepared, if the instruction is a MethodCall
PHI inputs are complete/normalized by the route-owned PHI authority
branch/jump targets exist and pass the canonical CFG preflight
metadata/origin/type post-success facts are already decidable
```

The canonical adapter must reject before mutation when any row is absent,
foreign, stale, or contradictory. It must not call:

```text
ensure_block_exists
ssa::local::recv / materialize_local_v1
phi_input_materializer::for_pred
function-repair or missing-PHI completion
AST/name/digest lookup
legacy emit retry
```

The existing `CanonicalCfgSessionV1` is the model for this boundary: its
`preflight_edge` checks source/target existence, cached edge truth, terminal
state, and seal state before mutation, then the commit performs only the
checked write and predecessor publication.

## Legacy preparation contract

The current compatibility behavior may remain available behind the legacy
facade while production callers are being retired. It may explicitly own:

```text
missing-block creation where the legacy route requires it
receiver LocalSSA materialization
legacy PHI input rematerialization
legacy operand normalization and diagnostics
```

Those operations must be named as a `LegacyRepairPlanV1` (or remain inside a
clearly named legacy adapter) and must never be inferred from a canonical
preparation failure. A canonical error is terminal for that attempt; it is not
a request to run legacy compatibility.

## Commit and failure order

The intended order is:

```text
route-owned read-only prepare
  -> PreparedCanonicalEmissionV1 or PreparedLegacyEmissionV1
  -> sole commit writer moves the instruction once
  -> infallible/prepared predecessor, origin, metadata, and PHI bookkeeping
  -> outer function session closes or discards the draft
```

No fallible validation may be deferred until after `add_instruction`. If a
bookkeeping operation cannot be made infallible, it must be included in the
outer session's typed discard proof before the canonical pilot is connected.
Local type publication follows the existing prepared-receipt rule: publish
only after the physical instruction commit succeeds.

## Finite state and forbidden transitions

The preferred Rust representation is move-only types rather than one mutable
runtime enum, but the state table is fixed here:

| State | Owner | Meaning | Allowed next state |
| --- | --- | --- | --- |
| `Unprepared` | route owner | instruction and route facts are not yet co-sealed | `CanonicalPrepared`, `LegacyPrepared`, or typed reject |
| `CanonicalPrepared` | canonical adapter | strict placement/operand/PHI evidence is complete | `Committed` or typed commit failure to session discard |
| `LegacyPrepared` | compatibility adapter | explicit repair permission is complete | `Committed` or typed legacy failure |
| `Rejected` | preparation owner | missing/foreign/contradictory input | terminal; no fallback |
| `Committed` | sole writer | one physical instruction and prepared bookkeeping are installed | outer session close |

Forbidden:

```text
CanonicalPrepared -> LegacyPrepared
canonical missing block -> ensure_block_exists
canonical receiver/PHI missing -> hidden materialization
canonical commit -> second append site
commit error -> AST/name re-search or compatibility retry
legacy repair result -> canonical placement authority
```

## Pilot boundary: Loop physicalizer CompareI64

The first I0 should use the existing
`loop_recipe_physicalizer::operation_emitter` CompareI64 leaf. It already has
an explicit target receipt, a prepared operation, a value ledger, and a
prepared Bool result contract. It does not require MethodCall receiver
materialization or PHI input repair. The generic `emission::compare` helper
remains a lower-level compatibility-compatible helper until a later caller
has the same placement proof.

Positive pilot:

```text
canonical placement issuer
  -> existing Loop target receipt + Compare operands
  -> PreparedCanonicalEmissionV1
  -> one writer commit
  -> prepared Bool type publication
```

Negative pilot cases:

```text
missing target block       -> typed reject from target issuer, block count unchanged
terminated/sealed target   -> typed reject, instruction count unchanged
undefined operand          -> typed reject, no type publication
canonical preparation fail -> no legacy retry
```

Call/receiver and PHI paths remain separate follow-up rows. They must not be
made to look canonical merely because the Compare pilot is green.

## Acceptance evidence for the D0 and I0 handoff

Design acceptance:

- one final physical writer and one append site are named;
- canonical and legacy issuers, non-authorities, and failure boundary are
  explicit;
- `ensure_block_exists`, LocalSSA receiver repair, and PHI rematerialization
  are phase-separated rather than hidden in canonical commit;
- the move/clone optimization row is separate;
- `NoSafeSlice` is retained if the selected canonical route has no placement
  issuer.

Pilot implementation acceptance:

- valid Compare emits exactly one instruction and one Bool fact;
- missing/terminated/sealed/undefined inputs fail before any physical or type
  publication;
- canonical failure has zero legacy fallback;
- legacy focused parity remains green;
- the final writer remains one and touched Rust owners stay below 760 lines;
- a reusable structural guard proves canonical code does not call
  `ensure_block_exists` or legacy repair helpers in the pilot adapter.

## Non-claims and parked work

This card does not authorize:

```text
assignment failure atomicity (already closed by Gate 1)
workspace Clippy rollout or EmitReceipt
full LocalSSA definition-index design
PHI repair/finalization retirement
builder.rs or builder_init.rs physical reorganization
per-instruction clone removal / compile-time benchmark gate
Script A/C, Recipe/Join, publication, backend, or main integration
```

The next implementation card must be created only after the source authority
and pilot placement issuer are accepted. The current `work_mode` therefore
stays `design_stop` while this D0 is the active pointer.
