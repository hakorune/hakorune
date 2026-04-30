# P19: Stage-B shape canary capture SSOT

Scope: route the remaining Stage-B shape canary stdout extraction through the
shared Program(JSON v0) capture helper.

## Why

Several core Stage-B shape canaries still used inline `awk '/^{/,/^}$/'`
ranges. That pattern only works when the first `{...}` range is the Program
object and does not handle noisy stdout or nested JSON with the same contract as
the main Stage-B helper path.

## Decision

Use `tools/selfhost/lib/stageb_program_json_capture.sh` from the Stage-B shape
canaries.

This is a capture cleanup only. It does not add new accepted language shapes and
does not change the Program(JSON v0) keeper buckets.

## Acceptance

```bash
bash -n tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_shape_canary_vm.sh
bash -n tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh
bash -n tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh
bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_shape_canary_vm.sh
bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh
bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
