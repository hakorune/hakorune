# Hako Implementation Prompt

Use this prompt to ask another AI/worker to implement the `.hako` version of
the RustSubset JSON v0 converter.

```text
Please implement the .hako version of apps/rust-subset-to-hako.

Read these files first:
- apps/rust-subset-to-hako/README.md
- apps/rust-subset-to-hako/DESIGN.md
- apps/rust-subset-to-hako/HAKO_JSON_PLAN.md
- apps/rust-subset-to-hako/schema/RustSubset-v0.md
- apps/rust-subset-to-hako/convert.py
- apps/rust-subset-to-hako/selftest.py
- apps/rust-subset-to-hako/examples/simple_subset.json
- apps/rust-subset-to-hako/examples/simple_expected.hako
- apps/rust-subset-to-hako/examples/edge_subset.json
- apps/rust-subset-to-hako/examples/edge_expected.hako
- apps/rust-subset-to-hako/examples/invalid_unknown_kind.json
- docs/reference/boxes-system/filebox.md
- tools/hako_parser/cli.hako

Goal:
Create a .hako converter that matches the Python converter behavior for v0.

Suggested files:
- apps/rust-subset-to-hako/main.hako
- apps/rust-subset-to-hako/lib/rust_subset_json_reader.hako
- apps/rust-subset-to-hako/lib/rust_subset_emit.hako
- apps/rust-subset-to-hako/lib/rust_subset_cli.hako

Required behavior:
1. Read RustSubset JSON v0 from a file path using FileBox.
2. Emit .hako skeleton text.
3. Match examples/simple_expected.hako for examples/simple_subset.json.
4. Match examples/edge_expected.hako for examples/edge_subset.json.
5. Fail-fast for unknown item kind, using invalid_unknown_kind.json.

JSON plan:
- Reuse apps/lib/json_native first.
- Put RustSubset schema navigation in rust_subset_json_reader.hako.
- Use FileBox for path input: open(path, "r") -> read() -> close().
- Use NYASH_FILEBOX_MODE=core-ro for first VM smoke if plugin setup is noisy.
- Do not build a JSON DLL/externcall for v0.
- Do not reimplement a second JSON parser inside rust-subset-to-hako.

V0 supported item kinds:
- Struct
- Enum
- Impl
- Function
- Unsupported

V0 supported statement kinds:
- Let
- Return
- Expr
- Unsupported

V0 supported expression kinds:
- Literal
- Name
- Field
- Binary
- Call
- MethodCall
- Unsupported

Mapping rules:
- Struct(identity=false) -> record
- Struct(identity=true) -> box
- Enum -> comment block
- Impl method -> function Target_method(me: Target, ...)
- Rust self -> Hako me
- string literal -> quoted string
- bool literal -> true / false
- null -> null
- i8/i16/i32/i64/isize -> i64
- u8/u16/u32/u64 -> i64
- usize -> usize
- String / &str / str -> String
- Vec<T> -> Array

Important constraints:
- Do not implement Rust source parsing.
- Do not implement a second JSON parser.
- Do not require a JSON DLL/externcall for v0.
- Do not implement borrow checking.
- Do not implement macro expansion.
- Do not implement trait/generic resolution.
- Do not claim semantic equivalence with Rust.
- Do not silently drop unsupported declarations.
- Do not hardcode example file names or source names.
- Do not replace FileBox path input with a native file-read DLL for v0.
- Keep Python converter as reference, not runtime dependency.

Acceptance:
rust_source_parser_owned_by_hako=0
hako_converter_implemented=1
filebox_path_input_used=1
simple_fixture_matches=1
edge_fixture_matches=1
unknown_kind_fail_fast=1
full_rust_transpiler_claim=0
summary=ok
```

## Suggested Verification

Use the Python converter as the behavior oracle at first:

```bash
python3 apps/rust-subset-to-hako/selftest.py
```

Then add a Hakorune smoke once `main.hako` exists. The smoke should compare
generated output with the two golden files and assert the invalid fixture fails.

Do not remove or rewrite the Python converter until the `.hako` converter has
its own smoke.
