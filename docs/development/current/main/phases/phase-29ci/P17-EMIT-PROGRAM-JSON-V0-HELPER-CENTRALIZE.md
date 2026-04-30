---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: centralize the remaining `--emit-program-json-v0` shell callsites behind helper-owned keepers without changing Program(JSON v0) contract coverage.
Related:
  - docs/development/current/main/phases/phase-29ci/P7-RAW-COMPAT-CALLER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P12-REMAINING-RAW-COMPAT-CALLERS.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - tools/selfhost/lib/program_json_v0_compat.sh
  - tools/smokes/v2/lib/stageb_helpers.sh
---

# P17 Emit Program JSON v0 Helper Centralize

## Goal

Collapse the remaining raw Program(JSON v0) emit callsites into explicit helper
owners, while keeping the compatibility flag available until the caller
inventory reaches zero.

This is a structure-only cleanup:
- no new accepted Program(JSON) shapes
- no MIR route behavior change
- no deletion of the public compat flag

## Landed Shape

| Surface | Owner after P17 | Notes |
| --- | --- | --- |
| selfhost Stage1 / Stage-B Program(JSON v0) emit | `tools/selfhost/lib/program_json_v0_compat.sh` | Single selfhost owner for the raw compat emit CLI |
| phase29bq Program(JSON v0) fixture producer | `tools/smokes/v2/lib/stageb_helpers.sh` | Single smoke fixture producer helper |
| runtime deprecation text | `src/runtime/deprecations.rs` | Kept while the public compat flag exists |
| archive phase pins | `tools/smokes/v2/profiles/archive/joinir/*` | Historical keep; not a current surface |

## Remaining Keepers

The compat flag is still kept because these lanes still need Program(JSON v0)
payload evidence:

- Stage-B Program(JSON v0) producer for selfhost build.
- Explicit Stage1 contract `emit-program` probe.
- `.hako mirbuilder` Program(JSON v0) fixture and contract-pin smokes.

Deletion now requires replacing those evidence lanes, not merely removing
duplicate shell syntax.

## Acceptance

```bash
rg -n -g '!tools/historical/**' -g '!target/**' -- '--emit-program-json-v0' src tools
bash -n tools/selfhost/lib/program_json_v0_compat.sh tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/selfhost_build_stageb.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_cleanup_try_min_vm.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
```

Known observation: `phase29bq_hako_mirbuilder_phase2_min_vm.sh` still returns
rc=2 in the current `.hako mirbuilder` route. Its Stage-0 Program(JSON v0)
emission is helper-mediated after this slice; the remaining failure is not a
raw emit callsite ownership issue.
