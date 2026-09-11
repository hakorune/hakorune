---
Status: closed__MirBuilderFinalAcceptanceScope__2026-09-11
Date: 2026-09-11
Decision: MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-D0
Parent: mirbuilder-canonical-ssa-s6c-state-boxshape-d0-2026-09-11
---

# MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-D0

Execution row: `MIRBUILDER-FINAL-ACCEPTANCE-SCOPE-R0`.

## Six-line brief

```text
Decision: execute the existing fixed real-app EXE acceptance manifest once
  and record its finite evidence without widening or rewriting the scope.
Source authority + canonical issuer: the integration suite manifest, runner,
  and each named existing smoke own source/backend/expected-result metadata.
Non-authority: raw test counts, synthetic MIR tests, changed fixtures, or a
  whole-library green inference from this suite.
Fail-fast boundary: preserve each smoke's existing failure and classify it as
  current-change, known baseline, or informational; do not retry another route.
Smallest next slice: run the exact 11-entry suite and record hashes/results in
  this card, with no source or fixture edits.
Non-claims: no production switch, language-v1 proof, Loop parity, selfhost,
  WASM, unselected backend parity, or whole-MirBuilder completion.
```

## Fixed scope

The only selected command is:

```text
tools/smokes/v2/run.sh --profile integration --owner-profile integration \
  --suite real-apps-exe-boundary
```

The manifest is
`tools/smokes/v2/suites/integration/real-apps-exe-boundary.txt`. It selects the
five typed-object probes, `boxtorrent-mini`, `binary-trees`, `mimalloc-lite`,
`allocator-stress`, the explicit unsupported-boundary probe, and
`json-stream-aggregator`, for eleven finite entries. Existing runner, backend,
toolchain, and expected-result declarations remain authoritative.

## Evidence boundary

Record the repository HEAD, manifest/script/source SHA-256 values, observed
result, and each failure's existing owner. Keep known baseline failures separate
from current-change failures. Do not change an accepted source into a reject,
add a fixture, create a parallel ledger/guard, or rerun through another backend.
The result is a finite acceptance handoff to convergence, not a production
completion claim.

## R0 execution receipt (2026-09-11)

The exact manifest was executed with the repository's existing runner. The
runner file and `selfhost_build.sh` were invoked through `bash` because the
workspace checkout exposed tracked-100755 files as mode 664; the latter and
`emit_mir_route.sh` were temporarily made executable for the run and restored
to mode 664 afterward. No file mode change is present in the commit.

Environment: `rust_vm_dynamic`, default backend, dynamic plugins, release
`hakorune` and release `ny-llvmc`, sequential suite execution (`Jobs: 1`). The
runner completed 11 entries: 3 passed, 8 failed. The unsupported-boundary probe
passed. Seven failures reproduce existing named owner/baseline boundaries from
the parent acceptance record. `typed_object_newbox_min_exe` instead stopped at
the environment boundary because the FFI library was not found; its earlier
success is not comparable in this run, and this observation does not establish
a compiler regression.

| entry | result | observed terminal | source SHA-256 | smoke SHA-256 |
| --- | --- | --- | --- | --- |
| `json_stream_aggregator_exe_runtime_boundary` | fail | `callable-loop/route-not-front-selected` (`GenericLoopV1NotSelected`) | `b5d4461b43a9b1b9e975192d1b701d5468524e61c5bc353303e62c7ece58e370` | `fa378922fc21740551d54f8f1e453b08c9b119edb6b7eda591e05e31951c5e08` |
| `typed_object_newbox_min_exe` | fail (infra) | FFI library not found; earlier pass not comparable | `80b07fe145e5fe61b2d326620fd5b1929c452124e3fe5c5528d4aeb96be54a7b` | `014f30f18f30f1fb4759f074cd53eefca99553d1520fafad9db745402ae98d0f` |
| `typed_object_birth_param_min_exe` | fail | `ordinary-new/local-commit/root-call-entry-missing` | `6c9feec4c2bbaab48be2f3c71f07e0fcff0fd686b36b1aec387f298fa36ca7b7` | `bb98b3149a2c3e66e19d2aa5b6fe837a5c94057773844e8c65d1c56b1577c50e` |
| `boxtorrent_mini_exe` | fail | `ParameterContract/UnsupportedDeclaredType` | `0c22fe686c826f718889e6ac1ff96ff3f8a3ff22628401c4d7f93d75ebbf1ae7` | `2d8f1f7e41b5f496a1992cbfb4cff19a79999ae17ebdd158060e463521aa3b4d` |
| `binary_trees_exe` | fail | callable-semantic incomplete consumption | `27de7bedca3e05bfb9facbbc5417ac1eb623a720b53c150b42e0e8646f12fa1c` | `4fe22d3a4bc1fd4350be090eb0a26783ed103bdcfc5dabf31a00a73ff69feecc` |
| `real_apps_exe_boundary_probe` | pass | exact unsupported boundary | n/a | `489c6f28bd174962d21c2631dd0c79b172c379a77c90410e80d684aa3cade33d` |
| `mimalloc_lite_exe` | fail | `ParameterContract/UnsupportedDeclaredType` | `63f688bef954d29ef91930e861b0721293789c4e74af2c6f1dae25b6ed84b772` | `03009535b3a11929aa1f8ccfd386b2bcc388af161888dcfd7bf91968d6552871` |
| `allocator_stress_exe` | fail | `ParameterContract/UnsupportedDeclaredType` | `9ce97be74bbe747d95d40ef0471ed44c4043b5be3f3dd9143a2b12954ef8f94f` | `42ccf8ba637c4a8c82a226342d2ec66b9249688f64986d1b279e4e7bc2db44cb` |
| `typed_object_method_min_exe` | pass | exit 30 | `f6e962309f166f4d2cc8993c4c388ec45d7ab3173e70b1641d9389fbe22ee16d` | `5e1e6ab7dc806cddaad18abf03b0273665385ecfdbfca8bb9ab03e442b35e451` |
| `typed_object_birth_min_exe` | pass | exit 30 | `bbd957398d16ae9e77c1b2bd3c510e8b6ba03c694d97ed14807f8ffe320aa0a4` | `610d8f506261942b78196a47a78c3ebd7bb0e3c82836bea3f3a6f744a7678ef1` |
| `typed_object_untyped_field_min_exe` | fail | MIR JSON unsupported terminator `Invoke` | `456e7de7f583b9cbc36f3b5abfb479fd64baec822259631a581ce68f442febdb` | `3239bcf7ec156cd8830346c036a08b76226b76fcff5e9871ede82bc0c6308eeb` |

Manifest SHA-256:
`92ad589d44a51c28ed66cd31064851b229419408ed8de4610e4fbcbac32806e1`.
Runner SHA-256:
`eb829d6ab7c062974adc912428622dbb9dccf602fa8991197298b16865b88ee0`.
The direct helper hashes are `selfhost_build.sh`
`9f5519c18dc292995dcbae28af7532a5bb71f7b9052e868ec73e03c11b9fc691` and
`emit_mir_route.sh`
`8352041abdd43bc306c601264c2e567216c14596aacdd06d03ddc0f6048508d9`.
The generated release tool hashes were `hakorune`
`8779319d24381e8169b6acbc994cbf4818afd7ee7f4fd8f8b952c0e51cf8d02f` and
`ny-llvmc`
`fd4a23b1dc61cad419f1f400a6d07183d0f93b73bfe554cdb94a8f93b019cfd6`.

The fixed acceptance scope is therefore complete as a finite evidence handoff
with 3/11 current observed passes, seven separately owned baseline boundaries,
and one FFI-environment boundary. It does not close those owners, infer a
compiler regression from the unavailable FFI library, or claim production
cutover or whole-MirBuilder completion.
