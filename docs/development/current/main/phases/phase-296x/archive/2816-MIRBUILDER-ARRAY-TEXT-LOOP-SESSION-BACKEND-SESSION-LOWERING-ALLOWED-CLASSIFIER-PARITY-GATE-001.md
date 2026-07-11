# 2816 MIRBUILDER-ARRAY-TEXT-LOOP-SESSION-BACKEND-SESSION-LOWERING-ALLOWED-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-05

## Decision

Add a dedicated `.hako` EXE parity gate for
`array_text_loop_session_backend_session_lowering_allowed_classifier`.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_array_text_loop_session_backend_session_lowering_allowed_classifier_parity_gate.sh`

## Acceptance

The gate compiles a temporary `.hako` app through the EXE path and compares
the 7 fixture rows against the Rust oracle strings.

## Next

`MIRBUILDER-ARRAY-TEXT-LOOP-SESSION-BACKEND-SESSION-LOWERING-ALLOWED-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
