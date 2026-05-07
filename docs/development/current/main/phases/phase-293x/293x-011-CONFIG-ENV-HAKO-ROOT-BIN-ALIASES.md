# 293x-011 Config Env Hako Root/Bin Aliases

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: move the code-side root/binary path seam toward Hako-facing names
  without breaking existing `NYASH_*` users.

## Decision

- `HAKO_ROOT` is the preferred repo-root hint.
- `HAKO_BIN` is the preferred hakorune executable hint.
- `NYASH_ROOT` and `NYASH_BIN` remain compatibility aliases.
- Path hints are centralized in `src/config/env/paths.rs`; low-risk callers use
  the helper instead of direct `std::env::var`.
- This slice does not rename broad public `NYASH_*` configuration families.

## Changes

- Added `hako_root()` / `hako_bin()` helpers and compatibility wrappers.
- Updated config catalog and environment-variable reference docs.
- Migrated using resolver, SSOT child invocation, runner root resolution, stage1
  module cache path, llvmlite harness lookup, and compare-driver binary lookup
  to the helper seam.
- Kept resolver tests on `NYASH_ROOT` to cover legacy compatibility, while
  isolating any external `HAKO_ROOT`.

## Verification

```bash
cargo test -q test_env_vars_no_duplicates
cargo test -q populate_from_toml
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
