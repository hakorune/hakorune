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
| Temporary evidence | pass | stash `wip/other-worker-docs before design-registry-v1` = stash@{0}, named, owned by other worker — restore deferred to owner |
| JoinModule disposition | pass | all `JoinModule` references confined to `join_ir/lowering` (oracle/shadow per D0 receipt); no new production callers |
| Documentation parity | pass | INDEX.md rewritten, `registry/README.md`, taskboard rows, `check-scripts-index.md` helper row added |

Residual (baseline debt, owners named above): `naming_charter_guard`
red on `stage_a_route.rs:114` + dormant `\/` rg check at line 809.
