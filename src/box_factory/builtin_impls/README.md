# Builtin Fallback Implementations

This directory is the quarantine area for built-in Box construction paths that
still have an explicit runtime fallback or compatibility role.

Do not add a new thin constructor wrapper here when a Box already has a ring1
provider seam. The factory should call the ring1 seam directly.

## Moved To Ring1 Seams

The following Box constructors are already owned outside this directory:

- `ArrayBox`: `crate::providers::ring1::array::new_array_box`
- `MapBox`: `crate::providers::ring1::map::new_map_box`
- `PathBox`: `crate::providers::ring1::path::new_path_box`
- `ConsoleBox`: `crate::providers::ring1::console::new_console_box`

## Still Allowed Here

- `file_box.rs`: owns the `FileBox` auto/core-ro fallback matrix. Do not move or
  delete it without preserving plugin-only fail-fast and core-ro behavior.
- `filehandle_box.rs`: owns the runtime-profile-aware `FileHandleBox` fallback.
- `null_box.rs`: owns the surface/compat `NullBox` spelling over runtime
  no-value semantics.
- `string_box.rs`, `integer_box.rs`, `bool_box.rs`: primitive constructor
  fallbacks. These are not ring1 provider seams; remove only after a separate
  primitive value/object boundary plan.

## Removal Rule

Deleting a file here requires one of the following:

- A ring1 provider seam already owns the constructor and has a local regression
  test.
- A dedicated SSOT says the public spelling is retired or moved to a different
  value/object boundary.

Keep this directory small, but do not collapse fallback semantics into the
factory match arm.
