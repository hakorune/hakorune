ring1/array — Active Scope

Status
- Decision: `accepted` (wired).
- SSOT: `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`
- Promotion template: `docs/development/current/main/design/ring1-core-provider-promotion-template-ssot.md`
- Dry-run task pack: `docs/development/current/main/phases/phase-29y/85-RING1-PROMOTION-DRYRUN-TASK-PACKS.md`

Current contract
- Runtime wiring is enabled via:
  - `src/runtime/provider_lock/mod.rs` (`set_arraybox_provider`, `new_arraybox_provider_instance`)
  - `src/runtime/plugin_host.rs` (`Ring1ArrayService` registration path)
- Contract smoke:
  - `tools/smokes/v2/profiles/integration/apps/ring1_array_provider_vm.sh`
- Contract guard:
  - `tools/checks/ring1_array_provider_guard.sh`

Rules
- Keep ring1 implementation free from ring2/plugin dependencies.
- Keep provider logic thin (ArrayBox behavior mirror; no language-semantics branching).
