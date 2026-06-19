# Rust Subset Converter Probes

Status: EXE/AOT app-front probes with stable gates separated from historical
investigations.

## Layout

```text
stable/
  acceptance probes called by smoke.sh

regression/
  small fixed-bug probes, not called by smoke.sh unless explicitly promoted

investigations/
  bring-up trail and current blockers; not an acceptance gate

retired/
  historical-only probes if a future cleanup needs archival
```

## Stable Gate

`stable/json_probe.hako` is the only probe called directly by
`apps/rust-subset-to-hako/smoke.sh`.

It verifies:

```text
JSON parse -> object field lookup -> array length -> EXE/AOT
```

`convert.hako`, `convert_file.hako`, and `convert_adapter_fixture.hako` are
also part of `smoke.sh`, but they are app wrappers rather than probe files.
They cover embedded fixture input, FileBox input, and host-produced adapter
fixture handoff respectively.

## Investigations

`investigations/` contains the JSON bring-up trail and disabled blockers.

## Regression Probes

`regression/` contains fixed bugs that are run when
`RUST_SUBSET_RUN_REGRESSION=1` is set.

```text
regression/schema_bool_shape_probe.hako
  purpose=bool-returning schema helper plus not call-site
  current_status=exe_aot_green

regression/bool_return_call_branch_probe.hako
  purpose=user/global bool-return call normalized before branch/not use
  current_status=exe_aot_green

regression/schema_normalizer_probe.hako
  purpose=status-code schema helper path
  current_status=exe_aot_green

regression/json_object_key_materialization_probe.hako
  purpose=critical JSON object key materialization
  current_status=exe_aot_green

regression/json_unknown_key_materialization_probe.hako
  purpose=generic unknown-key entry-table fallback
  current_status=exe_aot_green

regression/json_nonzero_number_probe.hako
  purpose=nonzero JSON integer token payload materialization
  current_status=exe_aot_green

regression/json_tokenizer_number_payload_storage_probe.hako
  purpose=NUMBER token type/value survives JsonTokenizer.tokenize()->ArrayBox storage
  current_status=exe_aot_green

regression/filebox_read_probe.hako
  purpose=FileBox minimal new/open/read/close EXE/AOT route
  current_status=exe_aot_green
```

No investigation probe is currently part of the accepted app-front route.
Keep new input-route probes in `investigations/` until their active row promotes
them.

Do not move an investigation into `stable/` until it is green on EXE/AOT and
the active app-front row explicitly promotes it.
