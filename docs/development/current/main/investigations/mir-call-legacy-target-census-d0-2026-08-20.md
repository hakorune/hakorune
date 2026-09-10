---
Status: Closed design; observation manifest resynchronized 2026-09-11
Date: 2026-09-11
Decision: MIR-CALL-LEGACY-TARGET-CENSUS-D0
Active row: MIR-R7-LEGACY-CENSUS-RECONCILE-D0
Parent: docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
ProductionCaller: none (observation only)
ReplacementCell: none until the census closes
---

# MIR-CALL-LEGACY-TARGET-CENSUS-D0

## Six-line brief

Decision: Freeze and classify every remaining `MirInstruction::Call {
callee: None }` producer/consumer before any canonical-call guard or field
retirement. This is a read-only design census, not an implementation row.

Source authority + canonical issuer: The `MirInstruction::Call` definition
and each exact producer/consumer at the pinned commit are the source facts;
one deterministic census manifest is the sole observation issuer and adds no
semantic receipt.

Non-authority: text hit counts, `ValueId::INVALID`, `func` spelling, test names,
compatibility labels, diagnostics, inferred backend behavior, or a selected
green fixture cannot classify a row or authorize a migration.

Fail-fast boundary: before a GUARD-I0 is opened, every row must have one exact
file/line, role (`canonicalizer_input`, `explicit_compatibility`, `test`,
`diagnostic`, or `unreachable`), reachability disposition, owner, and named
replacement/retirement condition. Missing, duplicate, or commit-drifted rows
stop the lane with `NoSafeSlice`.

Smallest next slice: produce the pinned 25-mention census, split selected
native/canonical candidates from JSON-v0, interpreter, compatibility, tests,
and diagnostics, and write the GUARD-I0 acceptance without changing MIR,
loader, optimizer, backend, or printer code.

Non-claims: no `Option<Callee>`/`func` deletion, no `LegacyCall` variant, no
default/sentinel callee, no backend fallback change, no JSON-v0 retirement, no
Script transport, no production switch, and no performance claim.

## Scope and fixed order

This row is the first successor named by the canonical-call SSOT:

```text
CENSUS-D0 -> CANONICAL-CORRIDOR-GUARD-I0 -> LEGACY-TARGET-RETIREMENT-R0
```

It is explicitly selected only after the Script cleanup row closed. It does
not bypass the parked canonical Script transport or reopen the selected-normal
physical bridge. The later GUARD-I0 must first prove, for one named native /
canonical corridor, `callee.is_some() == 100%`, zero legacy `func` authority,
and zero string fallback. Only after every non-corridor family has an explicit
compatibility or retirement owner may R0 choose an end state.

## Initial source census (read-only)

At the current branch HEAD, the literal `callee: None` occurrences are spread
across 20 Rust files and 25 source mentions. The worker-audited inventory is:

| Family | Current locations | Initial role |
| --- | --- | --- |
| JSON-v0 compatibility producer | `src/runner/json_v0_bridge/lowering/expr/call_ops.rs:81,368,412` | 3 explicit compatibility producers |
| Canonicalizer input | `src/mir/passes/callsite_canonicalize/pass.rs:112` | 1 canonicalizer input |
| Native boundary reject/analysis | `src/mir/inline_leaf.rs:272`, `src/mir/contracts/backend_core_ops/allowlists.rs:14` | 2 production reject/diagnostic consumers |
| Contract/comments only | `src/mir/instruction.rs:430`, `src/mir/join_ir_to_mir/call_generator.rs:16`, `src/mir/contracts/backend_core_ops/allowlists.rs:6` | 3 non-executable references |
| Tests/fixtures/assertions | ownership, verification, instruction/backend, callsite, JSON program, LLVM guard, and VM tests | 16 test-only mentions |

The audit found zero remaining selected-native producers outside the
canonicalizer input and zero `unreachable` production rows. The selected
native/canonical corridor is the final canonicalized MIR after
`MirOptimizerLateCallAndInline`, at the boundary immediately before LLVM or
other native backend consumption; JSON-v0 and VM compatibility are excluded.
This is a design census, not permission to change that boundary.

## Required GUARD-I0 design output

The census must define one machine-readable manifest (or an equivalent stable
guard fixture) with:

- pinned commit and source digest;
- unique row ID and exact file/line/symbol;
- producer vs consumer direction;
- family role and production/test/compat/diagnostic reachability;
- whether `func` is an authority or merely a legacy carrier;
- selected-corridor eligibility;
- replacement owner and retirement condition;
- explicit `NoSafeSlice` for rows whose caller/owner is not proven.

The guard design must reject row count drift, an unclassified new occurrence,
duplicate row IDs, a selected corridor with `callee=None`, and any claim that
`ValueId::INVALID` alone proves canonicalization. It must not mutate code or
promote the manifest into semantic compiler input.

The candidate GUARD-I0 acceptance is:

```text
final canonicalized native MIR: every Call has callee.is_some() == true
selected corridor callee=None count = 0
call-missing-callee reject code = stable
legacy func re-resolution/fallback = 0
callee=Some does not require func == ValueId::INVALID
```

## Stop conditions

Return to `NoSafeSlice` and do not open GUARD-I0 implementation if:

1. a producer/consumer cannot be assigned one source-backed owner;
2. JSON-v0, interpreter, or compatibility behavior is mixed with native
   canonical behavior in one row;
3. the selected corridor still depends on a string/name fallback;
4. `func` and `callee` both remain authorities in the same route;
5. a proposed guard requires changing MIR, loader, optimizer, backend, or
   printer behavior to make the census green;
6. the source commit changes while the manifest is being issued.

The smallest successful outcome is a complete, pinned census and a named
GUARD-I0 design. It is not a production cutover.

## 2026-09-10 R7 census reconcile (selected design)

The previous 25-mention snapshot is historical evidence only. At HEAD
`e7e5c1455a8b4c487c0311f8064c8311d784e172`, the bounded R7 inventory is:

| Inventory | Current count | Boundary and disposition |
| --- | ---: | --- |
| direct `LegacyCallV0` production constructors | 3 | `compat_entrypoints.rs` (2) and JSON-v0 `module.rs` `boxcall` (1); explicit compatibility ingress |
| mechanical Legacy reissuer | 1 | `joinir_id_remapper.rs`; ID remap only, no semantic issuance |
| production lexical legacy surface | 239 occurrences / 127 files | `src` + `crates` Rust, excluding tests and `#[cfg(test)]`; census only, not a reader count |
| compile env save/set/restore | 5 functions / 4 route families | Rust transport/static/ny-llvmc plus C route helper; no runtime hook registry |
| JSON-v0 `boxcall` ingress | 1 | `module.rs`; `call`/`mir_call` remain pre-publication freeze paths |
| test-only Dynamic Loop-PHI residue | 1,009 LOC / 4 files | no production caller; canary inventory only |

The finite boundary is:

```text
canonical/compatibility MIR ingress + compile profile
  -> LegacyCallV0 constructors/reissuer/readers/egress
  -> selected artifact reject or compatibility terminal

test-only Dynamic Loop-PHI files
  -> production-caller census terminal
```

Included are the Rust production scope under `src` and `crates`, plus the
fixed `lang/c-abi` environment anchors. Excluded are
the canonical `MirInstruction::Call` definition, ordinary test fixtures,
unrelated startup/runtime environment state, runtime hook registries, future
VM/WASM parity, and production Dynamic Loop canary code.

This reconcile corrects the old labels: direct production constructors are
three, not four; the llvmlite-era projection is a reader/projection; and the
old 202/208-reader and 990-LOC figures are stale for this boundary. The row
does not authorize LegacyCallV0 deletion, a reader migration, or a caller-zero
claim.

The aggregate table is not yet the machine-readable manifest required by the
parent GUARD-I0 design. The next execution slice must add one observation-only
manifest with a pinned full commit, scope digest, and stable rows. Each row
must carry a deterministic id derived from `path|line|symbol|token`, the exact
source anchor, role, owner, reachability, selected-corridor disposition, and
retirement/reopen condition. The manifest must cover 239 Legacy lexical rows,
5 compile-environment route rows, and 4 test-only Dynamic Loop-PHI file rows
(248 rows total); `boxcall` and the mechanical reissuer are classifications
within the Legacy rows, not duplicate rows. A dedicated guard is deferred to
GUARD-I0, and no production code or semantic receipt is permitted in this
design stop.

### Reopen triggers

Reopen the census if a selected canonical backend consumes `LegacyCallV0`, a
new non-test constructor or reissuer appears, JSON-v0 reaccepts `call` or
`mir_call`, a compile-profile save/set/restore helper is added, the Dynamic
Loop-PHI fixtures gain a production caller, or any inventory path/count drifts.

### R7 design acceptance

Close this row only after the pinned inventory, includes/excludes, authority
and non-authority, and the named reopen triggers above are recorded together.
No R7 deletion, compatibility retirement, or performance claim is evidence for
this design row.

## 2026-09-11 observation manifest closeout (superseded counts)

The bounded manifest is now materialized at
`tools/checks/manifests/mir_r7_legacy_census_manifest_v1.json`. Its generator
is `tools/checks/mir_r7_legacy_census_manifest.py`; it uses only tracked source
text and fixed anchors, and it never supplies compiler input or semantic
authority. Stable row IDs are derived from the row kind, path, line, symbol,
and token. Per-file SHA-256 anchors and the scope digest detect source drift.

Validation evidence:

```text
python3 tools/checks/mir_r7_legacy_census_manifest.py --write
  -> deterministic 248-row manifest
python3 tools/checks/mir_r7_legacy_census_manifest.py
  -> ok rows=248 legacy=239 env=5 loop_phi=4
python3 -m py_compile tools/checks/mir_r7_legacy_census_manifest.py
git diff --check
bash tools/checks/current_state_pointer_guard.sh
  -> ok
```

The manifest records 239 Legacy lexical rows across 127 files, five concrete
compile-environment route rows across four route families, and four test-only
Dynamic Loop-PHI files totaling 1,009 lines. `boxcall` and the mechanical
reissuer are classifications inside the Legacy rows, so they are not counted
twice. This closes the R7 census design only; GUARD-I0, LegacyCallV0
retirement, compatibility migration, and caller-zero remain separate rows.

## 2026-09-11 manifest resynchronization

The selected normal admission change added four production-scope
`LegacyCallV0` observations in the existing `PublishedMirBackendView` owner.
The prior 239/248 snapshot therefore triggered the documented source-drift
reopen condition. The observation-only generator and manifest were updated at
the current HEAD without changing compiler meaning or retirement policy.

The resynchronized boundary is:

```text
legacy lexical rows: 243 occurrences / 127 files
compile-environment routes: 5 / 4 families
test-only Loop-PHI files: 4 / 1009 LOC
independent manifest rows: 252
```

Validation:

```text
python3 tools/checks/mir_r7_legacy_census_manifest.py --write
python3 tools/checks/mir_r7_legacy_census_manifest.py
  -> ok rows=252 legacy=243 env=5 loop_phi=4
```

This is a census repair only. It does not authorize LegacyCallV0 deletion,
compatibility migration, a new reader, or a caller-zero claim.

## Historical D0 closeout (2026-08-20)

The source census was complete at the historical pinned worktree HEAD: 25 literal
mentions, with 3 JSON-v0 compatibility producers, 1 canonicalizer input, 2
production reject/analysis consumers, 3 non-executable contract/comment
references, and 16 test-only fixtures/assertions. No selected-native producer
outside the canonicalizer input and no unreachable production row were found.
This historical snapshot is superseded by the 2026-09-10 reconcile above.

The next bounded row is therefore:

```text
MIR-CALL-CANONICAL-CORRIDOR-GUARD-I0
```

It may add only an observation/structural guard for the selected native
corridor. It must not edit `MirInstruction::Call`, JSON-v0, compatibility
loaders, optimizer semantics, backend lowering, or any production caller.
