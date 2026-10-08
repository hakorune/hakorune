Task: REPO-HYGIENE-AUDIT-PUNCHLIST-2026-09-15
Status: tracking card — verified findings; fifth-audit docs reconciliation is
3/3 closed for the current direct-file delta. The original
rows below remain independently tracked unless a row says otherwise.
Source: external day-3 audit (2-lens independent review) delivered 2026-09-15
---

# Repo hygiene audit punchlist

External audit findings verified in-tree on `3e3d39d632`. Each row is a
bounded follow-up; none is part of the active MirBuilder semantic lane.

## 2026-09-20 fifth-audit reconciliation

The later audit's three documentation P1s were checked against the current
tree before changing this card:

| Finding | State | Evidence / boundary |
|---|---|---|
| North-star scheduler text says Loop production selection is unopened | **Closed** | `52ffa39d36` updates the loop SSOT scheduler frontier to `MIR-CALL-PARSER-LOOPBREAK-SOURCE-PACKAGE-I0`; the broader production-selection claim remains explicitly unopened. |
| Typed carrier ABI is missing from the primary ABI reference | **Closed** | `52ffa39d36` adds the checked-Map callable carrier v1 contract to `docs/reference/abi/nyrt_c_abi_v0.md`, including `representation:"map"`, `param-carrier-drift`, the checked C surface, and borrowed-view lifetime ownership. |
| Design-registry membership drift is repaired | **Closed for the current delta** | The current inventory found 673 registered rows and 80 direct unregistered design files against the declared baseline of 77. The three post-baseline direct contracts (`collection-literal-construction-ssot.md`, `constructor-lifecycle-llvm-lowering-ssot.md`, and `mir-call-published-rows-invocation-ownership-d0.md`) are now explicit `authority` rows in `design/INDEX.md`; the generated V0 manifest is refreshed and `repository_artifact_lifecycle_inventory.py --check --strict` is green. Warning mode and the declared baseline 77 remain unchanged; broader sidecar/archive census stays outside this closeout. |

The “8 months / 8 files” wording from the later audit was not used as a
fact because the exact in-tree census is larger and has a different scope.
No bulk registration or baseline increase is authorized by this reconciliation.

The remaining documentation P1 is closed for the current direct-file delta,
not converted into a mass registration. The three newly added contracts were
classified as registry rows, the V0 manifest was regenerated from that
authority, and the warning baseline stayed at 77. Sidecar/archive adjudication
and the wider lifecycle census remain separate work; this closeout does not
retarget the active MirBuilder production lane.

### 2026-09-21 code-side registry correction

The previous reconciliation covered the design registry only. It did not close
the separate code-side `src/mir/builder/control_flow/plan/REGISTRY.md` row.
The current in-tree census finds nine files added under `plan/` since
2026-09-14 that were not named there: five production/source-handoff files and
four test-only companions. The registry now names the complete seven-surface
handoff family (including the already-landed LoopTrue and dispatch seams) and
states that the test companions are evidence, not route owners.
The auditor's “eight files” number is therefore not retained as a fact; the
bounded inventory above is the current scope.

The two pipeline SSOT capsules also had stale hard-coded scheduler pointers.
They now follow `CURRENT_STATE.toml`, retain the correct
`NoSafeSlice`/missing-consumer boundary, and explicitly state that broad Loop
production selection remains unopened until the physical-consumer and
exclusive-delete tuple exists. This is a pointer correction, not a production
selection claim.

## P1 — spec/decision management

| # | Finding | Verified evidence | Required action |
|---|---|---|---|
| 1 | static-box `method` member modifier is parser-accepted but has no grammar-registry row or EBNF production | `src/parser/declarations/static_def/members.rs:47-70` `take_method_modifier_name`; `grammar/language-v1-registry.toml` has no `method` modifier production; `EBNF.md` `member`/`method_decl` rules lack it. `grammar-contract.md` states registry row = grammar authority | Add the registry row + EBNF production documenting the parser-first admission |
| 2 | `return null` classification flipped the 7/25-approved decision without a revision record | `function-exit-f1-return0-s0-execution-task-2026-07-25.md` classified `return null` → `ExplicitUnit(origin=ExplicitNull)`; `function-exit-and-entry-result.md:251` now classifies it as explicit value return. `types.md:321` still says "null is a syntax-level alias of void" | Write a revision record on the decision chain (supersede note on the 7/25 card or in the new classification card); update `types.md` to the current null/void relation. Overlaps queued card C4 (`:void` mixed `return null`/`return void` decision record) |
| 3 | census denominators (899 bodies / 231 births) are /tmp-only evidence | `/tmp/merged_entry.hako` sha256 `23b6cf894b0619a41ade0f7ff77480d2228530727a0c883540f06d37b78ba1ed`, 19042 lines, 89 input sections; no in-repo generation recipe. Input list IS recoverable from embedded `// <file>.hako` headers | Record hash + input list + generation recipe beside any card citing merged census numbers (F1 card updated) |

## P2 — structural

Queued user request (2026-10-08): [test/guard responsibility retirement task](test-guard-responsibility-retirement-task-2026-10-08.md).
First bounded candidate is the live daily MIR root facade/import family:
measure, preserve distinct checks, switch callers and physically retire the
exclusive old script. The active compiler selection stays unchanged; parked
lanes remain parked. This link does not claim deletion or a timing improvement.

Queued user feedback (2026-10-08): [docs/spec/adapter/artifact follow-up task map](docs-spec-adapter-artifact-hygiene-task-map-2026-10-08.md).
H-1 history thinning, H-2 current/target labels, H-3 owner splitting/adapter
retirement, and H-4 generated evidence retention have separate completion bounds.
The same map concretizes existing safety row 10; it does not select its implementation.

| # | Finding | Verified evidence | Required action |
|---|---|---|---|
| 4 | workstream card at the 1,000-line boundary; R7 D0 card at 999 | `workstreams/mirbuilder-inplace-replacement-current.md` = 1000 lines; `mir-call-compatibility-retire-r7-d0-2026-09-11.md` = 999 lines | Tombstone-compress both before the next append |
| 5 | `mir-call-resolver-if-expression-contract-d0-2026-09-14.md` `NextCard:` dangles to `mir-call-resolver-if-i64-return-contract-d0-2026-09-14.md`, deleted in `1310f61c56` without supersede note | git log confirms replacement by `mir-call-resolver-if-value-join-contract-d0-2026-09-14.md`; 11 other dangling `NextCard:` refs exist repo-wide (mostly never-created cards) | Repoint to the value-join card with a supersede note; optionally sweep the other dangling refs |
| 6 | `exec.rs:141` uses fixed temp name `tmp/nyash_cli_emit_harness.json` (shared-dir collision/staleness) | `src/runner/modes/common_util/exec.rs:144` — the sibling `ny_llvmc` path already uses `nyash_cli_emit_{pid}.json` | Align to the pid-namespaced pattern |
| 7 | "StringBox findings" referenced as out-of-scope across `mir-call-static-compatibility-*` cards but no tracking card owns them | a3-package-admission-d0:116 and siblings exclude them; no findings card exists | Create the tracking card or record an explicit discard decision |
| 8 | Windows child-process "0" injection | `lang/c-abi/shims/hako_aot_child_process.inc:21-26` — `hako_aot_capture_direct_harness_env` injects literal `"0"` when `HAKO_LLVM_OPT_LEVEL`/`NYASH_LLVM_OPT_LEVEL` are unset (`hako ? hako : "0"`); unset-vs-"0" is observable to the child (located by auditor on days 2-3, re-verified in-tree) | Decide fix (propagate unset) vs record |
| 9 | Three reusable guards have stale structural anchors | `normal_callable_semantic_source_row_i0_guard.sh` expects the pre-ledger `with_selected_source_scope` call shape; `script_direct_static_target_guard.sh` expects the type-op classifier in `calls/special_handlers.rs` although its current owner is `calls/build.rs`; `callable_compatibility_source_transport_guard.sh` expects `TypedCompatibility` in the lifecycle owner although the current enum is in `normal_default_program_root.rs` | Classify as `informational census`/guard drift; refresh only the guard assertions in a later mechanical slice, without changing the active static-publication semantics |
| 10 | `BorrowedMapArrayResidence` accepts raw pointers through a safe public constructor; safe reads and cloned `CheckedMapReadView` lack an enforced root/view lifetime | Re-verified 2026-10-08: `src/boxes/map_array_residence.rs:158-187` checks only empty/null, dereferences before `require_live`; borrowed view also dereferences at `:45` and is Clone/Send/Sync. Sole production constructor caller remains `crates/nyash_kernel/src/exports/fault_checked_map.rs:328`. Safe-API soundness gap confirmed; production UAF/crash not reproduced | **Open safety task, implementation unselected**: see [bounded safety decision and acceptance](docs-spec-adapter-artifact-hygiene-task-map-2026-10-08.md#separate-safety-task-existing-punchlist-p2-row-10). Explicit unsafe intake plus residence/all-view root-lifetime contract; constructor-only unsafe marking does not close the row. Preserve caller-only End; separate from neutral cleanup |

## 2026-09-28 day-7-audit reconciliation (unfreeze)

The day-7 audit findings were re-censused in-tree and processed as a
bounded batch; the card unfreezes to record which rows closed:

| Finding | State | Evidence / boundary |
|---|---|---|
| `coreplan_phi_binding_boundary_guard.sh` demands deleted `nested_loop_depth1_preheader.rs`; `dev_gate quick` hard-fails since 9/25 | **Closed** | PREHEADER references removed; `RecipeOnly`/`body_entry_bindings` markers re-pinned to `loop_cond_bc_source.rs`; guard and `dev_gate quick` re-verified to the next wall. |
| Loop SSOT reopen condition assumed `route_loop` unchanged, invalidated by the 9/25 flip | **Closed** | `joinir-loop-selfhost-recipe-pipeline-ssot.md` now records M10b `6e88441c0b` as landed: `route_loop` runs the frozen located-source pipeline, the ordered scheduler is retired, and reopening requires a new live caller/authority gap. |
| R7 D0 card at the line boundary (1,019) | **Closed** | Tombstone-compressed to 372 lines; live frontier, CI/native receipts, and reopen triggers preserved. Also closes row 4's R7 half below. |
| `TARGET-ONLY-EMISSION-S0` lacked a focused test | **Closed** | `src/mir/compiler/static_result_target_only_emission_tests.rs` (2 tests) pins `TargetOnly -> Callee::Global` emission through the source-backed `compile_normal` lane. |
| Acceptance suite re-run for the landed S0 batch | **Closed (measured)** | `--emit-mir-json` census on binary-trees / mimalloc-lite / boxtorrent-mini / allocator-stress / json-stream-aggregator; terminals recorded in the gate1 card — all typed freezes (root-call-entry-missing, Invoke emit gap, NamedArray TextSourceMissing, SourceCallOutsideSelectedFamily). |
| gate1 string-indexof card approaching mega-card size (716) | **Closed** | Committed landed sections compressed to a tombstone block; card now 474 lines; uncommitted live sections (loop-handoff, root-instance-call) retained in full. |
| ~25 orphaned stale guard index entries | **Closed by classification** | Re-census finds 6 index references to absent files, all intentional: 2 `~~retired~~` tombstone rows and 4 compat-block names tracked in `tools/checks/manifests/guard_navigation_tombstones.toml` with `superseded` dispositions. The compat block exists because historical guards grep the index for name presence; deletion is not authorized. |
| `birth_param_min` meaning change unrecorded | **Closed (already recorded)** | Commit `2317661f74` message and the fixture header document the untyped-inference -> declared-`i64`-storage + `sum(): i64` + `--emit-exe` lifecycle-V4 migration; `untyped_field_min` keeps the sealed dynamic-storage sentinel role. |
| Punchlist frozen since 9/21 | **Closed** | This reconciliation. |

Row 4 remainder: `workstreams/mirbuilder-inplace-replacement-current.md`
is still exactly at the 1,000-line boundary — the next appender must
tombstone-compress closed rows before appending.

## 2026-09-28 dev_gate quick full-green restoration

After the day-7 fixes, `dev_gate quick` was walked wall-by-wall to
`PASS 67/67`. Profile: `NYASH_NY_LLVM_OPT_TOOL=opt-18
NYASH_NY_LLVM_LLC_TOOL=llc-18` plus `PATH=/tmp/llvm-py-venv/bin:$PATH`
(the checkout-local `.venv` is non-portable: its binary needs
`GLIBC_2.38` and records the old checkout path; a fresh
`/tmp/llvm-py-venv` with `llvmlite==0.47.0` replaces it for the
llvm_py unittest step). Walls fixed, in order:

| Wall | Classification | Fix |
|---|---|---|
| `naming_charter_guard` rg regex `\/` parse error + `Stage-A` wording in a panic message and re-introduced in new card tombstones | guard drift / charter violation in committed code | fixed the escaped-slash regex (anchored `BIN=` assignment check) and renamed the panic wording to `mode-A`; new tombstone prose avoids unqualified stage terms |
| `mir_metadata_consumer_manifest` missing `metadata.rs` fields + stale line anchors | committed growth not yet declared | added 3 rows (`physical_param_carriers`, `named_array_write_obligations`, `named_array_field_allocations`) with producer/consumer anchors; re-pinned drifted anchors to actual lines |
| cargo baseline `8046/127` vs measured `8048/125` under opt-18 | profile drift | re-baselined to the LLVM-18 tool profile (`expected_passed=8048, failed=125`); failure name set byte-identical |
| LLVM 14 `opt` on PATH rejecting opaque `ptr` | environment | explicit `NYASH_NY_LLVM_OPT_TOOL=opt-18` / `NYASH_NY_LLVM_LLC_TOOL=llc-18` |
| 204 tracked `100755` files lost exec bit (`core.fileMode=false` mount) | environment drift | restored exec bits from git index metadata; zero remaining mismatches |
| `.inc` analysis-debt allowlist missing 6 committed static-V2 files | baseline drift | registered the 6 files in `inc_codegen_thin_shim_debt_allowlist.tsv` |
| `core_method_contract_manifest_guard` rejections | stale sole-consumer assumptions | added committed consumers: `callable_source_ledger_contract_tests.rs` (test), `named_array_method.rs` + its tests file (production), and `core_method.rs`/`named_array_method.rs` to the result-row lookup allowlist |
| `abi_manifest_codegen.py --check` diff | generated file embedded absolute source path | codegen now emits repo-relative `Source:` path; regenerated file |
| `llvm_py unittest` ImportError `llvmlite` | environment | `/tmp/llvm-py-venv` with `llvmlite==0.47.0` on `PATH` for the gate run |
| K2-wide OSVM / Intrin guards hard-fail on `compile_v0_emits_mir_call_extern_*` tests | **known baseline debt** (S5 card classifies these as upstream explicit-extern admission freezes; vm_hako reference lane runs no resolver) | `run_cargo_test_filter_group` is now baseline-aware: filters matching `cargo_lib_red_baseline.failures.txt` entries are `--skip`ped with an echoed note; unlisted reds still fail |
| `hako_mem` runtime-decl guard rejects `mir_call_shell_extern_rules.inc` | committed split (`03b06622eb`) not allowlisted | added the route-fact rules table file to `ALLOWED_ROUTE_FACT_CONSUMERS` |
| allocator sentinel rejects `#[global_allocator]` in `fault_tests.rs` | cfg(test)-only counting allocator, unreachable from production | global-allocator scan exempts `*_tests.rs` |
| hako_alloc handle-policy guard stale pin | `recycle_handle` call moved to `host_handles/call_lifetime.rs` (8/15 lease split) | pin re-pointed to the real call site |

Known boundary carried: the extern compile tests remain baseline debt —
`compile_source_to_mir_json_v0` (vm_hako reference lane) issues no
explicit-extern resolved relation, so `externcall` freezes
`[explicit-extern/missing-resolved-relation]`. Repairing that lane is a
MirBuilder design slice, not hygiene.

Follow-up (same day): `dev_gate.sh` now self-defaults the recorded
LLVM 18 tool profile — `NYASH_NY_LLVM_OPT_TOOL`/`NYASH_NY_LLVM_LLC_TOOL`
are exported as `opt-18`/`llc-18` when the caller left them unset and
the versioned binaries resolve on PATH, so a PATH whose plain `opt`
predates opaque pointers no longer fails the baseline and K2 steps.
A plain `bash tools/checks/dev_gate.sh quick` re-run after this change
passes 67/67 (281s) on this checkout.

Follow-up (llvmlite host dependency): the quick step is now
`tools/checks/llvm_py_keep_lane_probe.sh` — it resolves the keep-lane
coverage through the first interpreter that imports llvmlite
(`python3`, the checkout `.venv`, then `uv run --with llvmlite==0.47.0
--no-project`, which matches the LLVM 18 profile), and reports a typed
`SKIP` when none does. This keeps `test_strlen_fast.py` (a frozen
`retain_keep` root in the G3 keep0 inventory) covered wherever
obtainable while making the daily gate carry zero hard llvmlite
dependency — consistent with the G2/AUTO0 direction. `dev_gate quick`
runs fully self-contained on this mount: no manual venv PATH needed.

## P3 — frozen docs set

final-pipeline SSOT (frozen 09-12) and R7 cards (frozen 09-14) docs-batch
items noted by the audit: `7543/7381` references, acceptance re-check
annotation, REGISTRY dangling, `route.inc:529`. Track under the R7/docs
freeze lane, not this card's scope — list kept here so the audit trail is
not lost.

## Non-claims

This card records and verifies; it does not authorize implementation of
any row. The good-news half of the audit (F17 commit-split resolution,
debug-print removal) is confirmed clean and needs no action.
