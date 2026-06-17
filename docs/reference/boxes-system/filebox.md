# FileBox Quick Reference

Status: current practical reference  
Scope: `.hako` app file input/output surface for current VM/selfhost app work.

## Purpose

Use `FileBox` when a `.hako` app needs to read a local file.

This is the first route to try for app-level file input. Do not add a native
file-read DLL or JSON/file externcall before the `FileBox` route has been tried.

## Read A Text File

```hako
static box App {
    main(path) {
        local f = new FileBox()
        if f.open(path, "r") == 0 {
            print("[app/error] open fail: " + path)
            return 2
        }

        local text = f.read()
        f.close()

        print(text)
        return 0
    }
}
```

Some older examples use `f.open(path)` with one argument. New app code should
prefer `f.open(path, "r")` for clarity.

## Provider Mode

If plugin setup is noisy while testing a `.hako` app, force the core read-only
provider:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --backend vm app.hako
```

Use the normal provider route once the app behavior is stable.

## Known Working Anchors

```text
tools/hako_parser/cli.hako
  new FileBox()
  open(path)
  read()
  close()

tools/hako_check/hako_source_checker.hako
  new FileBox()
  open(path)
  read()
  close()

apps/selfhost-runtime/mir_loader.hako
  new FileBox()
  open(path, "r")
  read()
```

The phase-29y VM-Hako feature matrix records `newbox(FileBox)`,
`FileBox.open(path, mode)`, `FileBox.read()`, and `FileBox.close()` as ported
for the vm-hako route.

## Current Caveats

- Treat VM as the first smoke target for app file-input bring-up.
- Stdin is a separate route; do not block file-input apps on stdin support.
- File write / binary routes have separate compatibility history. Do not infer
  write support from this read-focused quick reference.
- Prefer fail-fast on missing files. Do not silently continue with empty text.

## JSON App Pattern

For JSON-driven tools, use this shape:

```text
FileBox.read()
  -> JSON parser
  -> app-specific schema reader
  -> emitter / analyzer
```

For the RustSubset converter app, the JSON parser owner is
`apps/lib/json_native`, and the schema reader belongs under
`apps/rust-subset-to-hako/lib/`.
