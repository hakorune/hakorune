ring1/path — Active Scope

Status
- Decision: `accepted` (wired).
- SSOT: `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`
- Promotion template: `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
- Dry-run task pack: `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`

Current contract
- Runtime wiring is enabled via:
  - `src/runtime/provider_lock/mod.rs` (`PathService`, `set_pathbox_provider`, `get_pathbox_provider_instance`)
  - `src/runtime/plugin_host.rs` (`Ring1PathService` registration path)
  - `src/boxes/path_box.rs` (provider-backed `PathBox` consumer)
  - `src/backend/mir_interpreter/handlers/boxes_path.rs` (`PathBox` method dispatch)
- Contract smoke:
  - `tools/smokes/v2/profiles/integration/apps/ring1_path_provider_vm.sh`
- Contract guard:
  - `tools/checks/ring1_path_provider_guard.sh`

Rules
- Keep ring1 implementation free from ring2/plugin dependencies.
