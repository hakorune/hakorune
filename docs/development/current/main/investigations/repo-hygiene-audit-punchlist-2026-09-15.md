Task: REPO-HYGIENE-AUDIT-PUNCHLIST-2026-09-15
Status: tracking card — verified findings; fifth-audit docs reconciliation is
2/3 closed, with design-registry classification still open. The original
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

## P1 — spec/decision management

| # | Finding | Verified evidence | Required action |
|---|---|---|---|
| 1 | static-box `method` member modifier is parser-accepted but has no grammar-registry row or EBNF production | `src/parser/declarations/static_def/members.rs:47-70` `take_method_modifier_name`; `grammar/language-v1-registry.toml` has no `method` modifier production; `EBNF.md` `member`/`method_decl` rules lack it. `grammar-contract.md` states registry row = grammar authority | Add the registry row + EBNF production documenting the parser-first admission |
| 2 | `return null` classification flipped the 7/25-approved decision without a revision record | `function-exit-f1-return0-s0-execution-task-2026-07-25.md` classified `return null` → `ExplicitUnit(origin=ExplicitNull)`; `function-exit-and-entry-result.md:251` now classifies it as explicit value return. `types.md:321` still says "null is a syntax-level alias of void" | Write a revision record on the decision chain (supersede note on the 7/25 card or in the new classification card); update `types.md` to the current null/void relation. Overlaps queued card C4 (`:void` mixed `return null`/`return void` decision record) |
| 3 | census denominators (899 bodies / 231 births) are /tmp-only evidence | `/tmp/merged_entry.hako` sha256 `23b6cf894b0619a41ade0f7ff77480d2228530727a0c883540f06d37b78ba1ed`, 19042 lines, 89 input sections; no in-repo generation recipe. Input list IS recoverable from embedded `// <file>.hako` headers | Record hash + input list + generation recipe beside any card citing merged census numbers (F1 card updated) |

## P2 — structural

| # | Finding | Verified evidence | Required action |
|---|---|---|---|
| 4 | workstream card at the 1,000-line boundary; R7 D0 card at 999 | `workstreams/mirbuilder-inplace-replacement-current.md` = 1000 lines; `mir-call-compatibility-retire-r7-d0-2026-09-11.md` = 999 lines | Tombstone-compress both before the next append |
| 5 | `mir-call-resolver-if-expression-contract-d0-2026-09-14.md` `NextCard:` dangles to `mir-call-resolver-if-i64-return-contract-d0-2026-09-14.md`, deleted in `1310f61c56` without supersede note | git log confirms replacement by `mir-call-resolver-if-value-join-contract-d0-2026-09-14.md`; 11 other dangling `NextCard:` refs exist repo-wide (mostly never-created cards) | Repoint to the value-join card with a supersede note; optionally sweep the other dangling refs |
| 6 | `exec.rs:141` uses fixed temp name `tmp/nyash_cli_emit_harness.json` (shared-dir collision/staleness) | `src/runner/modes/common_util/exec.rs:144` — the sibling `ny_llvmc` path already uses `nyash_cli_emit_{pid}.json` | Align to the pid-namespaced pattern |
| 7 | "StringBox findings" referenced as out-of-scope across `mir-call-static-compatibility-*` cards but no tracking card owns them | a3-package-admission-d0:116 and siblings exclude them; no findings card exists | Create the tracking card or record an explicit discard decision |
| 8 | Windows child-process "0" injection | `lang/c-abi/shims/hako_aot_child_process.inc:21-26` — `hako_aot_capture_direct_harness_env` injects literal `"0"` when `HAKO_LLVM_OPT_LEVEL`/`NYASH_LLVM_OPT_LEVEL` are unset (`hako ? hako : "0"`); unset-vs-"0" is observable to the child (located by auditor on days 2-3, re-verified in-tree) | Decide fix (propagate unset) vs record |
| 9 | Three reusable guards have stale structural anchors | `normal_callable_semantic_source_row_i0_guard.sh` expects the pre-ledger `with_selected_source_scope` call shape; `script_direct_static_target_guard.sh` expects the type-op classifier in `calls/special_handlers.rs` although its current owner is `calls/build.rs`; `callable_compatibility_source_transport_guard.sh` expects `TypedCompatibility` in the lifecycle owner although the current enum is in `normal_default_program_root.rs` | Classify as `informational census`/guard drift; refresh only the guard assertions in a later mechanical slice, without changing the active static-publication semantics |
| 10 | `BorrowedMapArrayResidence` accepts arbitrary non-null raw pointers through a safe public constructor and later dereferences them from `read_map`; the `Send`/`Sync` promise also lacks a typed root-lifetime contract | `src/boxes/map_array_residence.rs:153-187`; production caller `crates/nyash_kernel/src/exports/fault_checked_map.rs:328`; direct exercised tests `crates/nyash_kernel/src/exports/fault_checked_map_tests.rs:58` and `:118` cover duplicate-root ownership and foreign-element rejection | **Open, not selected for the active parser lane**: the owner and direct evidence already exist, so the former `ParkedSealed` trigger is satisfied. When selected, choose an unsafe boundary plus explicit lifetime/root token or remove the borrowed API in favor of the owned residence; add direct positive/negative lifetime evidence before counting the borrowed API as closed |

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
