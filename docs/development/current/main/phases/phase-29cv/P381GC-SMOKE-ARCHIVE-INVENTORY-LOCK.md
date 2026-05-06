# P381GC Smoke Archive Inventory Lock

Date: 2026-05-06
Scope: T6 smoke/archive inventory before any broad script reduction.

## Decision

No directory-level smoke deletion is allowed yet.

The five archive/manual buckets requested by T6 are mixed surfaces: some files
are historical-only, but each bucket still has suite, gate, wrapper, or docs
reachability. The next cleanup slice must classify deletion candidates per
script, not by parent directory.

## Count Snapshot

These counts are current script counts from `find ... -type f -name '*.sh'`:

| Bucket | Scripts | Current reading |
| --- | ---: | --- |
| `tools/smokes/v2/profiles/archive` | 81 | mixed archive; JoinIR/selfhost/collection/historical SSOTs still own subtrees |
| `tools/smokes/v2/profiles/integration/archive` | 13 | suite-manifest referenced; not a bulk-delete bucket |
| `tools/smokes/v2/profiles/integration/apps/archive` | 225 | mixed app archive; plugin/dev-gate and JoinIR compat references remain |
| legacy `tools/smokes` outside `v2` | 14 | mixed legacy/manual entrypoints; some docs/tool references remain |
| `tools/archive/manual-smokes` | 35 | manual archive wrappers; delete policy exists, but broad delete is not proven |

Reference totals for scale:

| Surface | Scripts |
| --- | ---: |
| `tools/smokes/v2/profiles` | 1464 |
| `tools/smokes/v2/profiles/integration` | 1220 |
| `tools/smokes/v2/profiles/integration/apps` | 359 |
| `tools/smokes/v2/profiles/quick` | 155 |

## Evidence Commands

```bash
find tools/smokes/v2/profiles/archive -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/integration/archive -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/integration/apps/archive -type f -name '*.sh' | wc -l
find tools/smokes -path 'tools/smokes/v2' -prune -o -type f -name '*.sh' -print | wc -l
find tools/archive/manual-smokes -type f -name '*.sh' | wc -l
```

```bash
find tools/smokes/v2/profiles -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/integration -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/integration/apps -type f -name '*.sh' | wc -l
find tools/smokes/v2/profiles/quick -type f -name '*.sh' | wc -l
```

## Protected / Referenced Buckets

`tools/smokes/v2/profiles/integration/archive` is currently suite-referenced:

- `tools/smokes/v2/suites/integration/phase29ck-boundary-legacy.txt`
  references the twelve `archive/phase29ck_boundary/...` entries.
- `tools/smokes/v2/suites/integration/phase29x-derust-archive.txt`
  references `archive/phase29x/derust/phase29x_backend_owner_hako_ll_compare_min.sh`.

`tools/smokes/v2/profiles/integration/apps/archive` is mixed:

- `tools/vm_plugin_smoke.sh` still executes the phase29cc PLG pilot archive
  manifest.
- `tools/checks/dev_gate.sh` still reaches `tools/vm_plugin_smoke.sh` and PLG-07
  guard wrappers.
- `tools/checks/phase29cc_plg07_filebox_binary_*_guard.sh` still reference
  PLG-07 archive smokes.
- `docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md`
  maps active JoinIR semantic wrappers to several apps/archive compat targets.

`tools/smokes/v2/profiles/archive` is mixed:

- `docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md`
  owns many `profiles/archive/joinir/...` legacy stem decisions.
- `docs/development/current/main/design/selfhost-smoke-retirement-inventory-ssot.md`
  owns `profiles/archive/selfhost/...` as the manual diagnostics home.
- `docs/how-to/smokes.md` still lists selected `archive/selfhost` opt-in
  Stage-B canaries.

Legacy `tools/smokes` outside `v2` is mixed:

- `docs/guides/exceptions-stage3.md` references
  `tools/smokes/curated_llvm_stage3.sh`.
- `docs/development/current/main/design/archive/pyvm-retreat-ssot.md` references
  `tools/smokes/fast_local.sh`.
- `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
  references `tools/smokes/selfhost_local.sh`.

`tools/archive/manual-smokes` is an archive bucket, not a permanent keeper list,
but its README already defines a delete policy. Current docs and tool READMEs
still reference entries such as:

- `tools/archive/manual-smokes/hako_check_deadcode_smoke.sh`
- `tools/archive/manual-smokes/llvm_smoke.sh`
- `tools/archive/manual-smokes/ny_stage2_bridge_smoke.sh`

## First-Wave Rule

A smoke can enter the first deletion wave only after all of these are true:

- full-path references are zero outside the script itself and inventory tooling
- basename references are zero outside historical/archive-only docs
- suite membership count is zero
- wrapper manifests and guard scripts do not execute it
- the owning SSOT or archive README classifies it as deletable

That means the first wave is a per-script candidate list. It is not:

- delete all of `profiles/archive`
- delete all of `profiles/integration/archive`
- delete all of `profiles/integration/apps/archive`
- delete all legacy `tools/smokes`
- delete all `tools/archive/manual-smokes`

## Tooling Blocker

`tools/checks/smoke_inventory_report.sh` writes `class` as column 9:

```text
path family suffix fullpath_ref_count basename_ref_count wrapper_only suite_hit_count suite_names class
```

Its summary aggregation currently reads the orphan class from column 7, which
is `suite_hit_count`. Do not trust summary orphan counts from this script until
the class-column reader is fixed.

Next concrete T6 slice:

```text
fix smoke_inventory_report.sh class-column summary reads
```

After that, rerun inventory on the five T6 buckets and produce the first
per-script delete-candidate list.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
