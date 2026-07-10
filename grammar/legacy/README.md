# Legacy Grammar Inputs

This directory holds non-authority grammar-era inputs that still have named
build consumers.

`nyash-v1.1-codegen-input.toml` is read only by the root `build.rs` to generate
the legacy keyword and operator compatibility facade. It must not contain
Language v1 contract rows, profile decisions, parser support claims, or parser
conformance fixtures.

The active Language v1 grammar authority is
`../language-v1-registry.toml`. Retire this directory only after the root
legacy generator and its generated compatibility API have an explicit removal
card and no live consumers.
