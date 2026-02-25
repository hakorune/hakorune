ring1/console — Active Core Provider

Status
- Decision: `accepted` (runtime wired).
- SSOT: `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`
- Promotion template: `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
- Dry-run task pack: `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`

Runtime contract
- Provider implementation: `src/providers/ring1/console/mod.rs` (`Ring1ConsoleService`)
- Provider lock entry:
  - `set_consolebox_provider`
  - `new_consolebox_provider_instance`
- Host initialization SSOT: `src/runtime/plugin_host.rs`

Contract pin
- Fixture: `apps/tests/ring1_console_provider/console_warn_error_min.hako`
- Smoke: `tools/smokes/v2/profiles/integration/apps/ring1_console_provider_vm.sh`
- Guard: `tools/checks/ring1_console_provider_guard.sh`
