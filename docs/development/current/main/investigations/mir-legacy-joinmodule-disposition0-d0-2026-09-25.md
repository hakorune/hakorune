# MIR-LEGACY-JOINMODULE-DISPOSITION0-D0 — legacy JoinModule disposition receipt

Parent: `repo-physical-structure-cleanup-ssot.md` step 5 (physical debt
retirement) and the final-audit dependency list
(`legacy_joinmodule_disposition`).

## Scope boundary

```text
起点: join_ir::JoinModule definition at src/mir/join_ir/mod.rs:492
終点: per-file retain/quarantine/retire disposition receipt
      (classification only — no route deleted or activated)
includes: every src/** file producing, consuming, or defining
          JoinModule (32 files) + edge_args doc reference
excludes: builder/control_flow/joinir/** merge machinery — it uses
          join_ir *types* (JoinFuncId, error_tags), not JoinModule;
          it is the retained production merge path
```

## Receipt

`docs/development/current/main/design/fixtures/mir-legacy-joinmodule-disposition-d0-v1.tsv`

33 rows; all `retain` — every unit has a live caller:

- Definition: `src/mir/join_ir/mod.rs` (`JoinModule` + impls + inline
  tests).
- Producers — `src/mir/join_ir/lowering/**` (17 files): oracle-only.
  Sole callers are `src/tests/mir_joinir_{min,funcscanner_trim,
  skip_ws}.rs` and `src/tests/joinir/lowering/`; no production route
  reaches them. removal_when: retire with the mir_joinir oracle
  family or when a non-JoinModule fixture replaces it.
- Producers/consumers — `src/mir/control_tree/normalized_shadow/**`
  (10 files + `control_tree/mod.rs` decl): strict/dev-gated StepTree
  shadow observability. Single entry:
  `run_function_body_step_tree_guard_v1` →
  `dev_pipeline::StepTreeDevPipelineBox::run`, which returns `Ok(())`
  when `!strict && !dev` — no production lowering behavior.
  removal_when: retire with the strict/dev StepTree shadow gate.
- Oracle consumers — `src/tests/mir_joinir_*.rs` (3 files).
- `src/mir/edge_args.rs` — doc comment only; CFG consumers own their
  transport independently of JoinModule lowering.

## Rules recorded

- `default_fallback = none` on every row: the dev pipeline is opt-in
  gated, and the lowering oracle family has no default production
  route.
- No route was deleted or activated; the receipt only blocks the
  final audit from treating this boundary as implicit.
- `reference_delta = 0`; owner README (`src/mir/join_ir/README.md`,
  `src/mir/control_tree/README.md`) already names these lanes.

## Next

`REPO-FINAL-CONVERGENCE-AUDIT0-G0` — the receipt feeds audit item 6
(`legacy_joinmodule_disposition`) alongside the authority-role
manifest (`MIR-AUTHORITY-ROLE-MANIFEST0-D0`).
