Status: Archived

# Manual Tools Archive

This folder stores unreferenced root-level manual utilities that are no longer
active product, build, smoke, or compat-capsule entrypoints.

Archived helpers:

- `archive_rust_llvm.sh`
- `clean_root_artifacts.sh`
- `compare_harness_on_off.sh`
- `codex-keep-two.sh`
- `dep_tree.sh`
- `dev_numeric_core_prep.sh`
- `egui_win_smoke.ps1`
- `joinir_ab_test.sh`
- `llvmlite_check_deny_direct.sh`
- `mir13-migration-helper.sh`
- `parallel-refactor-nyash.sh`
- `python_unit.sh`
- `trace_last_fn_from_log.sh`
- `using_combine.py`
- `vm_stats_diff.sh`

Current reading:

- active smoke wrappers live under `tools/smokes/v2/` or explicit current
  root keepers
- historical smoke wrappers live under `tools/archive/manual-smokes/`
- root-hygiene artifacts live under `tools/archive/root-hygiene/`

## Delete Policy

This folder is an archive bucket, not a permanent keeper list.

New archive entries should record these fields in the card that moves them, and
may mirror them here when the entry needs a long-lived restore note:

- `original_path`
- `archived_on`
- `archived_by_card`
- `last_known_owner`
- `delete_after`
- `restore_command`
- `delete_blocker`

An archived tool becomes a delete candidate after 30-60 days or two cleanup
batches when all of these remain true:

- no active refs from current docs, tools, src, lang, Makefile, or root README
- no current PASS gate owns it
- no compat capsule README owns it with a reproduction command
- no protected platform, build, CI, generator, release, or docs-guard owner
  claims it

Restore from git history only with a new owner pointer and a current acceptance
command. The lifecycle SSOT is
`docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md`.
