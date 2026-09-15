Task: REPO-HYGIENE-AUDIT-PUNCHLIST-2026-09-15
Status: tracking card — verified findings, none started
Source: external day-3 audit (2-lens independent review) delivered 2026-09-15
---

# Repo hygiene audit punchlist

External audit findings verified in-tree on `3e3d39d632`. Each row is a
bounded follow-up; none is part of the active MirBuilder semantic lane.

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
