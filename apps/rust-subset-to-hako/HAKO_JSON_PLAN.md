# Hako JSON Access Plan

Status: design handoff

## Decision

Do not start with a JSON DLL / externcall.

Use the existing `.hako` JSON library first:

```text
apps/lib/json_native/parser/parser.hako
apps/lib/json_native/core/node.hako
apps/lib/json_native/core/compat.hako
```

The first `.hako` converter slice should build a thin RustSubset-specific
reader on top of that library.

```text
JSON text
  -> JsonParserUtils.parse_json(text)
  -> JsonNode
  -> RustSubsetJsonReader
  -> RustSubset emitter
```

## Why Not DLL First

A DLL/native JSON bridge would be useful later, but it is the wrong first slice
for this app.

Reasons:

```text
1. apps/lib/json_native already exists and exercises Hakorune itself.
2. The goal is to test .hako compiler/app capability, not hide JSON behind host code.
3. A DLL would skip the string/array/map/object traversal pressure we want.
4. Native JSON can be added later as an acceleration or compatibility backend.
```

## Layer Split

```text
json_native:
  generic JSON parsing and JsonNode operations

RustSubsetJsonReader:
  schema_version / kind checks
  required field helpers
  typed navigation helpers
  fail-fast messages for RustSubset schema violations

RustSubsetEmitter:
  record/box/function/comment generation
  type mapping
  expression/statement rendering
```

## Suggested Files

```text
apps/rust-subset-to-hako/main.hako
apps/rust-subset-to-hako/lib/rust_subset_json_reader.hako
apps/rust-subset-to-hako/lib/rust_subset_emit.hako
apps/rust-subset-to-hako/lib/rust_subset_cli.hako
```

## RustSubsetJsonReader Minimal API

The reader should hide raw JsonNode traversal from the emitter.

```text
read_module(text) -> module node or fail-fast
kind(node) -> String
str_field(node, key) -> String or fail-fast
bool_field_or(node, key, default) -> Bool
array_field_or_empty(node, key) -> Array
node_field(node, key) -> node or fail-fast
optional_node_field(node, key) -> node|null
```

For v0, the reader may return JsonNode wrappers directly if that keeps the
implementation smaller. The important boundary is that all fail-fast schema
checks live in the reader, not scattered through the emitter.

## Parser Fidelity

The existing `json_native` parser is enough for the current fixtures if it can
parse:

```text
object
array
string
integer
bool
null
nested object/array
```

If a parser bug appears, fix `json_native` with a small fixture first. Do not
work around parser bugs inside the RustSubset emitter.

## File Input

Use `FileBox` for the first real `.hako` file-input route.

Known working precedent:

```text
tools/hako_parser/cli.hako:
  new FileBox()
  open(path)
  read()
  close()

phase-29y feature matrix:
  newbox(FileBox), FileBox.open(path, mode), FileBox.read(), FileBox.close()
  are ported for the vm-hako route.
```

Preferred v0 input flow:

```text
path argument
  -> FileBox.open(path, "r")
  -> FileBox.read()
  -> FileBox.close()
  -> JsonParserUtils.parse_json(text)
  -> RustSubsetJsonReader
```

Use `NYASH_FILEBOX_MODE=core-ro` in the first smoke if plugin setup is noisy.
The first committed smoke may be VM-only; EXE/AOT parity can be added after the
converter behavior is fixed.

Stdin support is separate. Do not block v0 on stdin if `FileBox` path input is
working.

Temporary bring-up fallback, if `FileBox` path input regresses:

```text
main.hako contains or receives a JSON string fixture
converter emits to stdout
```

This fallback must be documented as a bring-up fallback, not as the final app
shape.

## Native JSON Backend Later

A DLL/externcall backend can be considered later only as a replaceable backend:

```text
JsonAccess interface
  -> HakoJsonNative backend
  -> NativeJsonDll backend
```

Do not make the RustSubset converter depend directly on a DLL.

## Acceptance

```text
json_dll_required_for_v0=0
json_native_reused=1
filebox_path_input_is_v0_route=1
rust_subset_json_reader_defined=1
schema_fail_fast_owned_by_reader=1
emitter_raw_json_navigation_allowed=0
summary=ok
```

## Stop Lines

```text
do not reimplement a second JSON parser inside rust-subset-to-hako
do not add a JSON DLL before the .hako parser route is tried
do not replace FileBox path input with a native file-read DLL for v0
do not scatter required-field checks across emitter functions
do not silently coerce missing fields to empty strings
do not make the Python converter a runtime dependency
```
