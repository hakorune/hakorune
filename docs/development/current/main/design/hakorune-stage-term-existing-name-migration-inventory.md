# Hakorune Stage Term Existing Name Migration Inventory

Token: `STAGE-TERM-EXISTING-NAME-INVENTORY-001`

Status: classification-only inventory.

Parent task: `STAGE-TERM-EXISTING-NAME-MIGRATION-001`

This inventory classifies existing `stage` names before any migration work.
It does not rename files, flags, modules, routes, or docs. direct renames are forbidden until the affected occurrence is classified and its compatibility surface is known.

## Method

Snapshot command:

```bash
rg -n '\b(Stage-[AB]|Stage[0-9]|stage[0-9]|stage-[ab]|--stage[0-9]|--stage-[ab]|stage1_using_resolver|stage3)\b' \
  . \
  -g '!target/**' \
  -g '!.git/**' \
  -g '!docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md'
```

This is a representative path-family inventory, not a line-by-line rename
manifest. It is intended to prevent accidental broad replacements.

## Current Distribution Snapshot

The largest current buckets are:

```text
docs/development/**         historical/current design references
tools/selfhost/**           selfhost route scripts and bootstrap helpers
lang/src/**                 HHako source and compatibility labels
tools/smokes/**             route/gate scripts
src/**                      RHako modules, env, runner, MIR, tests
docs/tools/**               guard index and script index
crates/nyash_kernel/**      kernel export/config compatibility modules
tests/**                    fixture and compatibility tests
```

## Classification Table

| Path family | Observed terms | Classification | Current decision |
| --- | --- | --- | --- |
| `tools/selfhost/**`, `Makefile`, `target/selfhost/*stage1*` references | `stage0`, `stage1`, `stage2`, `stage3`, `Stage1`, `Stage2`, `Stage3` | bootstrap sequence / selfhost artifact labels | Keep for now. Rename only through a bootstrap compatibility card. |
| `tools/selfhost/proof/run_stageb_compiler_vm.sh`, `tools/smokes/v2/lib/stageb_helpers.sh`, related docs | `Stage-B`, `stage-b`, `--stage-b` | compiler mode / compatibility route | Do not rename directly. Future migration must add mode-B aliases before removing compatibility spellings. |
| `nyash.toml` pipeline_v2 entries | `stage1_*` | internal pipeline module compatibility | Do not rename directly. Requires manifest/module migration plan. |
| `crates/nyash_kernel/**`, `src/config/env/stage1.rs`, `src/stage1/**` | `stage1` modules and debug/export labels | ABI/module/debug compatibility path | Do not rename directly. Requires ABI/log compatibility inventory. |
| `docs/tools/check-scripts-index.md`, `docs/tools/script-index.md` | `Stage1`, `Stage0`, `Stage-B`, `Stage-3` | guard index / historical route docs | Rename only when owning scripts are migrated in the same slice. |
| `projects/nyash-wasm/enhanced_playground.html` | `Stage1`, `Stage2`, `Stage3` | frontend demo UI label | Out of compiler naming migration. Needs UI/product review before any rename. |
| `docs/guides/exception-handling.md`, `docs/reference/**` | `Stage0`, `Stage1`, `stage3` | reference/history wording | Rename only after a reference-doc decision records the new term. |
| `tools/ny_stage2_shortcircuit_smoke.sh` and temp file names | `stage2` | legacy compatibility script/temp naming | Do not rename directly. Archive or compatibility task required. |
| `tests/**` and parser fixture names | `stage3`, `Stage-B`, `stage1` | test fixture / compatibility flag | Rename only after replacement flags or aliases exist. |
| `tools/plugins/stage-built.sh` | `stage-built` | ordinary English / build staging | Not part of the compiler stage-term migration. |

## Landed Alias Slice

`STAGE-TERM-SYNTAX3-ALIAS-001` adds `--syntax-3` as the canonical frontend
syntax-level CLI spelling while keeping `--stage3` as a compatibility alias.

Scope:

```text
Rust CLI:
  --syntax-3 is a visible alias for the existing stage3 parser flag

HHako compiler entry:
  --syntax-3 and --stage3 both enable the same parser surface

Rust selfhost child spawn:
  new child invocations use --syntax-3

Selfhost proof/quickstart:
  representative proof command and quickstart examples use --syntax-3

Reference docs:
  --syntax-3 is documented first; --stage3 remains compatibility wording
```

Non-claims:

```text
--stage3 removed = 0
NYASH_NY_COMPILER_STAGE3 renamed = 0
parser internal stage3 API renamed = 0
```

## Migration Eligibility

An occurrence is eligible for a future rename only if all conditions hold:

- it is classified as `compiler mode`, `runner phase`, `lowering pass`, or
  `frontend syntax level`;
- the compatibility surface is known;
- an alias or replacement route exists when external users may reference it;
- the owning guard can prove the old and new names are not both new SSOTs;
- the slice changes one naming layer only.

## Forbidden Moves

```text
direct global replacement = 0
stage term rename without classification = 0
bootstrap stage rename without compatibility plan = 0
Stage-B route removal without mode-B alias = 0
stage1 module removal without ABI/log inventory = 0
--stage3 removal without syntax-level replacement = 0
```

## Next Safe Work

The next migration slice may pick exactly one classified layer:

```text
compiler mode:
  Stage-A / Stage-B -> mode-A / mode-B, with compatibility aliases

runner phase:
  runner stage1 -> runner phase-1, after route ownership is known

lowering pass:
  stage1_using_resolver -> lower-resolver

frontend syntax level:
  --stage3 -> --syntax-3, after CLI aliasing is defined
```

If more than one layer appears necessary, keep this inventory as the boundary
and split the work.

## Acceptance

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
