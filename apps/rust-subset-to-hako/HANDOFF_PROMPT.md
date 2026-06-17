# Handoff Prompt

Use this prompt when asking another AI/worker to implement the first slice.

```text
Please implement the first slice of apps/rust-subset-to-hako.

Read these files first:
- apps/rust-subset-to-hako/README.md
- apps/rust-subset-to-hako/DESIGN.md
- apps/rust-subset-to-hako/schema/RustSubset-v0.md
- apps/rust-subset-to-hako/examples/simple_subset.json
- apps/rust-subset-to-hako/examples/simple_expected.hako

Task:
Implement only the .hako converter from RustSubset JSON v0 to Hako skeleton
text.

Scope:
- input is RustSubset JSON v0
- output is Hako skeleton text
- match examples/simple_expected.hako for examples/simple_subset.json
- fail-fast on invalid JSON, unknown schema_version, missing required fields,
  and unknown node kind
- emit stable TODO comments for known unsupported nodes

Do not:
- implement a Rust parser
- implement borrow checking
- implement macro expansion
- implement trait/generic resolution
- claim semantic equivalence with Rust
- use source-name or example-name special cases
- silently drop unsupported declarations

Preferred first shape:
1. JSON reader / schema validator
2. declaration emitter
3. expression emitter
4. golden fixture check

Acceptance:
rust_source_parser_owned_by_hako=0
rust_subset_json_schema_defined=1
hako_converter_scope=v0_skeleton
simple_fixture_matches=1
full_rust_transpiler_claim=0
summary=ok
```
