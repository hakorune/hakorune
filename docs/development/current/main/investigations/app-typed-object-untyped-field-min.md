# APP-typed-object-untyped-field-min
Status: PASS. Date: 2026-10-24
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [accepted untyped-storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [ownership law](../../../../reference/language/ownership.md#untyped-strong-stored-slots).

## Result
- The unchanged app emits and runs its canonical Lifecycle V4 EXE; expected exit is 7. Its generic MIR JSON precheck still rejects lifecycle `Invoke` as designed.
- Fixed `real-apps-exe-boundary` suite: 9/11 PASS, 2 source failures (`boxtorrent-mini`, `binary-trees`), no timeout or SKIP, with the current optimized quick CLI.

## Contract closed
- Kept untyped slots source-owned and tagged. A Dynamic Array write carries `array` plus `empty_array_home`; Normal transfers the exact live Home, Fault leaves it for existing cleanup.
- Dynamic I64 `FieldGet` uses the checked Invoke path and the source-owned cleanup frame. Direct `ObjectFieldGet` remains for statically typed storage.
- The C physical-key validator now checks all supplied static keys without an eight-key ceiling; the same exact-key and independent V4 ownership checks remain in force.

## Verification
- `cargo quick-build --bin hakorune` succeeded; `bash tools/build_hako_llvmc_ffi.sh` succeeded.
- `typed_object_untyped_field_min_exe.sh` PASS; fixed suite 9/11 PASS. No app source, test, fixture, checker, or guard was added.

## Next
- Continue with `APP-binary-trees`. Current stop: `[ordinary-new/borrowed-result/source-not-i64]` at the `iterationCheck` sum using `itemCheck()` inside the loop. Read the existing source and VM MIR oracle; preserve the app and use the same semantic result.
