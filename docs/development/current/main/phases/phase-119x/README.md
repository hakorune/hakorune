# Phase 119x: vm debug/observability surface review

- 目的: vm-family の debug/observability surface を keep-now と candidate-thin に分け、public/front-door からはさらに離して narrow keep に固定する。
- 対象:
  - `tools/smokes/v2/profiles/integration/phase29x/observability/*`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `tools/dev/bug_origin_triage.sh`
  - current pointer docs
- success:
  - `route observability keep` / `strict-dev priority keep` / `non-strict compat boundary keep` の3役が current docs で exact に読める
  - generic debug/probe と live keep の境界が current pointers で崩れない
  - current pointers が `phase-119x` に揃う

## First-pass inventory

- keep-now debug/observability
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_strict_dev_priority_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh`
- candidate-thin
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_compat_bypass_guard_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_pin_guard_vm.sh`
  - `tools/dev/bug_origin_triage.sh`
- reading
  - suite pin lives in `tools/smokes/v2/suites/integration/phase29x-vm-route.txt`
  - keep-now surface is route observability + strict-dev priority + explicit compat boundary + vm-hako strict replay
  - candidate-thin surface should read as internal engineering guard/triage, not as a general front-door runtime path
