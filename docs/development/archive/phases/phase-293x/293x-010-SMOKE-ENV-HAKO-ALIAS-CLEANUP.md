# 293x-010 Smoke Env Hako Alias Cleanup

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: reduce repeated release-VM smoke environment assignments and begin
  safe Hako-facing naming without breaking historical `NYASH_*` compatibility.

## Inventory

Worker inventory found the full release app VM env block in the five current
real-app smoke scripts:

- `boxtorrent_mini_vm.sh`
- `binary_trees_vm.sh`
- `mimalloc_lite_vm.sh`
- `allocator_stress_vm.sh`
- `json_stream_aggregator_vm.sh`

Near-repeat blocks also exist in older ring1/tool/archive smokes, but those
have different plugin/tooling needs and are not migrated in this slice.

## Decision

- Add `HAKO_ROOT` / `HAKO_BIN` as the smoke-facing preferred aliases.
- Keep `NYASH_ROOT` / `NYASH_BIN` as compatibility bridges.
- Add `run_hako_vm_release` as the release-style app VM helper.
- Keep the underlying compatibility env names inside the helper until the
  runtime/config layer owns broader `HAKO_*` aliases.
- Do not change global env defaults in `env.sh`; dev/strict planner gates still
  rely on those defaults.

## Changes

- Centralized the current real-app release VM env block in
  `tools/smokes/v2/lib/test_runner.sh`.
- Updated the five real-app smoke scripts to use `HAKO_ROOT` and
  `run_hako_vm_release`.
- Updated smoke env docs to prefer Hako-facing helper names.
- Updated the executable-missing error wording to say Hakorune while still
  showing the compatibility `NYASH_BIN`.

## Verification

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
