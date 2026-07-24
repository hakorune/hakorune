# Compiler hygiene census and cleanup task

Status: **Taskized — observation and retirement ledgers only**  
Date: 2026-07-23  
Scope: dead-code allowances, feature-gate duplication, near-800-line files,
legacy proof ownership, stale cleanup documentation, route/phase naming, and
ignored local artifacts. This card does not authorize bulk deletion, feature
retirement, route-plan merging, phase renumbering, or changes to the active
Raw ROOT design lane.

The active compiler lane remains the Root/App design stop. Hygiene work must
be docs/guard-only until its own owner and retirement conditions are closed.

## Worker-verified baseline

The external review numbers were stale or scope-dependent. The reproducible
current baseline is:

```text
#[allow(dead_code)]
  src:       239 occurrences / 99 Rust files
  workspace: 296 occurrences / 120 Rust files
  src allowances with trailing rationale: 67
  bare src allowances: 172

vm-reference cfg attributes
  total: 73 / 19 files
  production-ish: 44
  test-only: 29

other cfg families
  legacy-tests:       35 / 5 files, test-only
  vm-legacy:            3 / 1 file, test-only
  phi-legacy:           4 / 2 files, test-only
  interpreter-legacy:   9 / 5 files, production compatibility
  llvm-inkwell-legacy:  9 / 6 files, production 4 / test 3

700 <= src Rust file lines < 800
  all files: 39
  production classification: 29
  tests/fixtures classification: 10
```

The `src` count is the relevant ASTCLEAN scope. Existing ASTCLEAN guards are
still pinned to older baselines and currently fail against 239; this is a
baseline/reason inventory problem, not evidence that all 239 allowances are
dead code. The last guard update predates the current disconnected RAW/CUT0
proof rows.

The later line-cap scan also found one test-only file above the hard cap:
`src/mir/global_call_route_plan/tests/runtime_methods/collection_builders.rs`
at 825 lines. Its final payload fixture was moved to the sibling
`collection_builder_payload.rs` without changing production code or test
semantics; the files are now 704 and 122 lines respectively. This is a
behavior-neutral M1 refactor, not a source-shape or route-policy change.

`Cargo.toml` currently defaults to `cli,plugins`; `vm-reference` is not a
default feature. Historical notes claiming a default-on migration lane must
not be treated as current policy.

## H1 — dead-code census before pruning

Task: `HYGIENE-H1-CENSUS0`.

Do not delete `#[allow(dead_code)]` in bulk. Produce an owner/rationale/
disposition manifest and reconcile the ASTCLEAN guard baselines. Classify each
allowance as:

```text
active disconnected proof
test/fixture support
compatibility bridge
documented staged migration
candidate for owner-specific retirement
unclassified (must remain a follow-up)
```

Acceptance:

```text
count and scope commands are reproducible
ASTCLEAN guard thresholds are explicit and green for the selected baseline
new allowances require an owner/rationale
active RAW/CUT0 proof allowances are not pruned early
no production behavior change
```

Follow-up `H1-PRUNE` rows are per owner and require caller-zero evidence plus
the replacement owner to be green. No global threshold relaxation without a
decision record.

## H2 — feature-gate census and split retirement

Task: `HYGIENE-H2-VMREF-CENSUS0`.

Separate the vm-reference migration from interpreter and LLVM compatibility.
First map all 73 vm-reference attributes, including the 44 production-ish
sites and 29 test-only sites, against current Cargo defaults and parity
evidence. Then decide retirement per owner. Do not remove the gates while the
active ROOT design is stopped.

`interpreter-legacy` and `llvm-inkwell-legacy` receive independent future
rows; they are not one generic “legacy cleanup” switch. Historical default
claims must be reconciled with current `Cargo.toml` before any gate is pruned.

## M1 — near-cap line monitor

Task: `HYGIENE-M1-LINECAP-MONITOR0`.

Record all 39 files with the reproducible classification (29 production, 10
tests/fixtures). Treat 700–799 as report-only warning and 800+ as fatal. The
first seven production files closest to the cap are:

```text
canonical_root_completion.rs       797
generic_method_route_plan/origin_inference.rs 794
extern_call_route_plan/route_spec.rs           793
ssa/phi_input_materializer/legacy_candidate.rs 792
string_dead_text_region_plan.rs                790
builder/control_flow/plan/normalizer/helpers_value.rs 786
builder/calls/lowering.rs                      782
```

Do not split these automatically. Active ROOT files such as
`canonical_root_completion.rs` remain with the ROOT lane; any refactor is a
separate behavior-neutral Refactor Series after the active row closes.

## M2 — legacy proof ownership

Task: `HYGIENE-M2-LEGACY-PHI-README0`.

Only `ssa/phi_input_materializer/legacy_candidate{,_cfg,_tests}` is currently
an explicit legacy-retirement candidate. Add a subtree README/ledger stating:

```text
candidate-only repair proof
zero production callers outside its own tests
no commit/publication authority
```

Retirement requires all of:

```text
module-owned replacement transaction
artifact/fact closure
fresh post-publication verifier
the two module_lifecycle callers and one JoinIR rewriter caller migrated
caller census for prepare_legacy/LegacyPhiRepair = 0
```

The other `module_*_candidate` and drained/canonical candidate products are
disconnected proof products, not generic legacy deletion targets. They need
separate owner decisions.

## M3 — stale refactoring target document

Task: `HYGIENE-M3-TARGETS-RECONCILE0`.

`tools/mir-refactoring-targets.md` is an initial-release document with stale
965/930/896/875 line claims and a retired MIR20→13 plan. It has no current
references. Replace it with pointers to the current cleanup/instruction-diet/
verification/builder SSOTs or archive it after recording that evidence. Do not
revive the old instruction-reduction plan.

## L — low-risk observation row

Task: `HYGIENE-L0-OBSERVE`.

```text
route_plan parent .rs + child directory
  keep: intentional Rust module decomposition; no merge now

phase directories
  keep: historical structure; define current/main scope before counting
  current/main inventory is 172 unique phase names / 198 phase directories
  do not rename or merge based on an unverified “207” count

large local artifacts
  __mir__.log is ~93M, ignored by *.log and untracked
  tmp/*.json and logs/*.log are ignored and untracked
  do not delete automatically
```

The observation guard should fail only for tracked or unignored large
artifacts, and should report the route/phase inventory deterministically.

## Safe execution order

```text
1. HYGIENE-H1-CENSUS0 + HYGIENE-H2-VMREF-CENSUS0
2. HYGIENE-M1-LINECAP-MONITOR0
3. HYGIENE-M2-LEGACY-PHI-README0
4. HYGIENE-M3-TARGETS-RECONCILE0
5. HYGIENE-L0-OBSERVE
6. owner-specific prune/refactor rows only after their caller-zero decisions
```

All steps preserve the active ROOT consultation and the 800-line rule. A
failed census or guard is a design/manifest issue; it is not permission to
add a broad deletion or to mix cleanup with Root lowering.

## Required evidence

```bash
git status -sb
git diff --check
bash tools/checks/current_state_pointer_guard.sh
rg -n '#\[allow\(dead_code\)\]' src --glob '*.rs'
rg -n 'vm-reference|interpreter-legacy|llvm-inkwell-legacy' src Cargo.toml
git ls-files -z | xargs -0 wc -l  # scoped production manifest derived separately
git check-ignore -v __mir__.log tmp/nyash_cli_emit_probe.json logs/probe.log
```

No source deletion or feature-gate retirement is part of this task card.
