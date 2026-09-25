# REPO-FINAL-CONVERGENCE-AUDIT0-G0 — final convergence audit

Status: active
Date: 2026-09-25

This is the final row of the cleanup/convergence series. It does not
implement; it audits landed state against declared authority and emits
the convergence receipt.

## Scope

Audit each axis and record: owner, evidence, residual blockers.

1. **Pipeline authority** — source → exact membership → AST-free
   Facts → Recipe → Verify → sole physical owner → publish. One
   producer per stage; no fallback paths.
2. **MIR root surface** — `MIR-ROOT-MODULE-SURFACE0-G0` manifest
   (215 module decls + 129 exports) still matches `src/mir/mod.rs`.
3. **Authority-role manifest** — 165-root census
   (`MIR-AUTHORITY-ROLE-MANIFEST0-D0`) still covers all roots.
4. **Context owners** — `CompilationContext` mixed-owner inventory;
   `MethodTailIndexV1` seam guard green.
5. **Live pointers** — CURRENT_STATE schema (36 keys), pointer guard,
   landed_tail hygiene.
6. **Design Registry** — V1 sharded store is sole authority
   (landed `911ede962d`..`940eb3652d`); counters: markers 0, V0 loader
   calls 0, INDEX.md 45 lines, wrong-shard/duplicate 0.
7. **Temporary evidence** — stash `wip/other-worker-docs before
   design-registry-v1` restored or explicitly owned.
8. **Legacy JoinModule disposition** — 33-row receipt holds; no new
   production callers.
9. **Documentation parity** — README/reference/index updates for every
   landed row are present.

## Named baseline debt (not this audit's regressions)

- `naming_charter_guard` red: `stage_a_route.rs:114` "Stage-A" wording
  from `5e4a5e596c` (other lane, Sep 14).
- Same guard line 809: `rg` pattern `\/` escape makes the
  `nyash-bridge-smoke|BIN=` check silently never run (dormant check).

## Exit

Convergence receipt with per-axis verdict; any residual gets owner +
reopen trigger; pointer may return to closeout mode.

## G0 receipt (2026-09-25)

| Axis | Verdict | Evidence |
|---|---|---|
| Pipeline authority | pass | M12 series retired all duplicate producer chains; loop dispatch keys on typed `CallableLoopSoleFamilyV1`; `LoopRouteId` survives only as diagnostic payload + facade test |
| MIR root surface | pass | `mir_root_facade_guard` ok: exports=129, modules=215 vs manifest |
| Authority-role manifest | pass | 165 rows; coverage re-check — only container roots (`src`,`lang`,`crates`) unlisted; every leaf covered |
| Context owners | pass | `mir_context_owner_split_guard` ok; `MethodTailIndexV1` seam holds |
| Live pointers | pass | pointer guard ok; 36-key schema; landed_tail ≤3 |
| Design Registry | pass | V1 sole authority; `check` 676 rows / 0 violations; markers 0, V0 calls 0, INDEX.md 45 lines, wrong-shard/dup 0 |
| Temporary evidence | pass | stash `wip/other-worker-docs before design-registry-v1` inspected and restored (popped at RETURN0): the two investigation-doc corrections are back in the worktree, verified accurate vs code, and committed with the review closeout. Remaining stash list is the other worker's `baseline reconcile` + `wip/h2-s2-s1-r1` — untouched |
| JoinModule disposition | pass | all `JoinModule` references confined to `join_ir/lowering` (oracle/shadow per D0 receipt); no new production callers |
| Documentation parity | pass | INDEX.md rewritten, `registry/README.md`, taskboard rows, `check-scripts-index.md` helper row added |

Residual (baseline debt, owners named above): `naming_charter_guard`
red on `stage_a_route.rs:114` + dormant `\/` rg check at line 809.

Cleanup notes for a later bounded row (not blocking):

- `LoopRouteContext` retains fields dead to `route_loop` post-M10b
  (`route_kind` — still computed per loop via `choose_route_kind`,
  `fn_body`, `debug`, `in_static_box`, `step_tree_max_loop_depth`);
  only `condition`/`body` are read by the frozen entry.
- `--backend vm` (deprecated legacy lane) freezes on loop fixtures at
  callable-main lowering — deliberate cutover; source-backed gates run
  `--backend mir`. Verified on a `vm-reference`-enabled build below.

## G0 completion evidence (2026-09-25, second pass)

The required matrix and dedicated guard now exist and pass:

- Matrix: `repo-final-convergence-audit0-g0-disposition.toml` — 34
  records; every record carries the required ten fields; unknown
  fields, duplicate paths, unclassified records, and missing evidence
  are rejected by the guard. Records reuse the existing authority-role
  manifest, context census, JoinModule disposition, consumer census,
  facade manifests, registry store, parity fixtures, and this card as
  evidence — no second semantic ledger.
- Guard: `tools/checks/repo_final_convergence_guard.sh` — validates the
  matrix schema and the pinned claims (one pipeline-order authority,
  `compiler-pipeline-ssot.md` historical, `src/mir/mod.rs` durable
  facade, current-entry parity set, explicit loop/JoinModule
  dispositions, durable registry store), cross-checks the referenced
  TSV receipts for unclassified rows, verifies guard-index
  registration, and delegates pointer parity to
  `current_state_pointer_guard.sh`. Registered in
  `docs/tools/check-scripts-index.md`.
- Import hygiene: the two 2026-09-22 wildcard imports were replaced
  with explicit imports
  (`named_array_emission_tests.rs` — 12 named symbols,
  `runner/mir_json_emit/root.rs` — 8 named symbols);
  `mir_root_import_hygiene_guard.sh` ok; `cargo check --tests` clean.

Acceptance set (all green):

```text
bash tools/checks/current_state_pointer_guard.sh          ok
bash tools/checks/mir_root_facade_guard.sh                ok exports=129 modules=215
bash tools/checks/mir_root_import_hygiene_guard.sh        ok
bash tools/checks/repo_final_convergence_guard.sh         ok 34 records
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict   current
git diff --check                                          ok
```

Post-cutover execution evidence (NYASH_BIN=target/quick/hakorune,
`vm-reference` feature enabled):

```text
phase29bq_portable_owner_source_backed_gate_mir.sh   PASS  (15/15)
phase29bq_typed_terminal_source_backed_gate_mir.sh   PASS  (164/164)
real-apps-exe-boundary suite (11 fixed entries)      2 pass / 9 fail
```

EXE suite failures are all typed fail-fast terminals in the selfhost
emit lane (no crash, no fallback): `[freeze:contract]` classes
`mir/main-import-view/selected-header-missing`,
`mir/callable-main/qualified-preflight` (UnsupportedFirstFamilyShape),
`callable-loop/facts-absent`, `callable-semantic-package/issue`
(ParameterContract / NamedArray TextSourceMissing), and one phase84-5
type-inference panic. None is caused by this slice (import-only
change). Owner: `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE` — the queued
closeout owner in `mirbuilder-inplace-replacement-current.md`, which
pins this exact manifest revision for evaluation; the suite is a
fixed selection for that owner, not a claim of current green.

## G0 closeout — closed

G0 is complete: the matrix, dedicated guard, import-hygiene fix, and
post-cutover evidence all hold. This closes the repo-structure cleanup
lane only; MirBuilder overall continues under the unified resume order.

Named residuals assigned to existing owners (not this audit):

- Call/R7 aggregate writer/reader/reissuer/re-entry deletion: open,
  family-local frontier pause; staged R6-S0..R7 queue remains unopened
  until an exact boundary is selected.
- B3 D2: `NoSafeSlice` — production substring route/codepoint outcome
  and ArrayPush provider/failure/commit authorities missing.
- Selfhost resume gates 2–4 (language-v1 conformance matrix,
  `MIRBUILDER-HAKO-MIMALLOC-PROMOTION-GATE0`,
  `MIRBUILDER-FACT-OWNER-PARITY-TEMPLATE-PILOT-SELECTION-001`): queued.

Next execution row: `SELFHOST-RESUME-ENTRY-RECHECK0-P0` — reconcile the
Call/R7 and B3 residuals against the selfhost resume entry conditions
(`selfhost-parser-mirbuilder-migration-order-ssot.md` unified resume
order) and select the next bounded owner.
