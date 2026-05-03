---
Status: Completed
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv generated EXE emit-mir source-side Return(Binary Int,Int) lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P330A-RETURN-BINOP-INTINT-FIELD-ORDER-TOLERANCE.md
  - docs/development/current/main/phases/phase-29cv/P327A-RETURN-INT-FIELD-ORDER-TOLERANCE.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# P331A: Return Binary IntInt Text Scalar Emit

## Problem

P330A fixed the first diagnosis: `LowerReturnBinOpBox` already owns
`Return(Binary(Int,Int))`, but the current Program(JSON) writer emits the nested
`expr` object before the `"type":"Return"` marker.

The first implementation attempt also showed the second seam: raw scalar and
dynamic operator concatenation can truncate the returned MIR text on the
source-execution DirectAbi path.

## Boundary

Do not add a new MirBuilder pattern.

Do not change Program(JSON) field ordering.

Do not add a delegate fallback.

Do not add a body-specific C emitter or a new `GlobalCallTargetShape`.

If the existing generic string concat emitter is incorrect, fix that shared
generic concat path rather than adding a Return(Binary)-specific escape hatch.

This card stays inside the existing `LowerReturnBinOpBox` owner.

## Implementation

Mirror the P327A owner-local contract:

```text
read integer payloads -> owner-local digit text
read operator -> accept only + - * /
emit operator fragments via static branches
```

The Binary object lookup accepts either type-before-expr or expr-before-type
field order within the same small Return object window.

The source-execution route also needs the existing module generic string concat
emitter to route through the shared concat helper. That helper owns concat result
finalization and chain tracking; bypassing it can drop the prefix of multi-step
text assembly before promotion sees `"functions":`.

Because the shared concat helper records pair/triple facts by MIR register id,
module generic function emission must reset that tracking state per function.
Register ids are local to each MIR function; leaking a previous function's
concat facts can make the next function emit undefined `%rN` values.

The `.hako` owner avoids generating ambiguous `dst_type:"i64"` string concat
nodes in the new Return(Binary) emitter. Numeric `dst_type:"i64"` remains owned
by the existing generic string emitter guard so mixed string helpers such as
`StringScanBox.read_char/2` keep their scalar `i + 1` lowering.
Only an already tracked shared-concat chain continuation may cross that guard;
this keeps the exception structural rather than by-name.

The emitted MIR JSON shape is unchanged.

## Acceptance

Run the generated source-execution EXE in methodize-on `emit-mir` mode:

```text
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p331a.exe
```

Expected behavior:

```text
rc=0
stdout starts with {"functions":[{"name":"main"
stdout contains "op":"binop"
stdout contains "operation":"+"
stdout is not a MirBuilder unsupported message
```

## Result

Implemented on 2026-05-03.

- `LowerReturnBinOpBox` now accepts the Return object when the Binary `expr`
  appears before `"type":"Return"`.
- Integer payloads are emitted as owner-local scalar text, avoiding
  `StringHelpers.int_to_str` and collection plumbing for this blocker.
- The module generic string emitter now routes accepted string `+` through the
  shared concat helper and resets concat tracking per emitted function so MIR
  register-local facts do not leak across function definitions.

Validation:

```text
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-exe /tmp/hakorune_p331a.exe lang/src/runner/stage1_cli_env.hako
HAKO_MIR_BUILDER_METHODIZE=1 NYASH_STAGE1_MODE=emit-mir \
  STAGE1_SOURCE_TEXT='static box Main { main() { return 1 + 2 } }' \
  /tmp/hakorune_p331a.exe
```

The acceptance run returned `rc=0` and emitted a canonical `functions` root with
`"op":"binop"` and `"operation":"+"`.
