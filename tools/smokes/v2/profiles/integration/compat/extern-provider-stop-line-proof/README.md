# extern-provider stop-line root-first proof

This bucket is the semantic home for the exact root-first lowering proof that
unblocks `W4 / 99O4`.

It stays under `integration/compat` because it proves a current compat/proof
stop-line surface, not a daily owner API.

## Bucket

1. extern-provider stop-line proof
   - `extern_provider_codegen_emit_object_root_first_vm.sh`
   - purpose:
     - prove one `vm-hako` lane can reach the current `extern_provider` codegen
       stop-line and still produce an object path plus linked executable
     - act as the exact proof gate before any `.hako` caller demotion starts

## Bucket Runner

- `tools/smokes/v2/suites/integration/compat/extern-provider-stop-line-proof.txt`
  - dedicated suite manifest for the proof bucket
- `tools/smokes/v2/profiles/integration/compat/extern-provider-stop-line-proof/run_extern_provider_stop_line_proof.sh`
  - runs only the exact proof smoke via the dedicated suite manifest

## Cleanup Rule

- this bucket is proof-only, not a new owner lane
- keep it separate from the compat selfhost wrapper stack
- if the stop-line is removed later, archive or delete this bucket together with
  the associated `99O4` docs rows
