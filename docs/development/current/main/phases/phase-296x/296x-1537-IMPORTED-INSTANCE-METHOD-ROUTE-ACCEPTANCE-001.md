# 296x-1537 IMPORTED-INSTANCE-METHOD-ROUTE-ACCEPTANCE-001

Status: active
Date: 2026-06-21

## Purpose

Accept the smallest AOT/backend route shape needed for native `.hako` APIs to
call imported user-box instance methods directly.

Current workaround:

```text
CarrierInfoNativeApi.from_snapshot(info, "i", snapshot)
```

Desired accepted shape:

```text
info.from_snapshot("i", snapshot)
info.with_explicit_carriers_from_snapshot("i", snapshot)
```

Observed failure:

```text
unsupported pure shape for current backend recipe
reason=module_generic_prepass_failed
callee_symbol=CarrierInfoNative.from_snapshot/2
```

## Scope

```text
BoxCount: one backend/MIR acceptance shape
owner: user-box method route / backend route prepass
input: imported user-box instance receiver with known box type
output: direct method route reaches EXE/AOT
```

## Acceptance

```text
apps/tests/phase296x_carrier_info_native_snapshot_min.hako
  uses info.from_snapshot("i", snapshot)

apps/tests/phase296x_carrier_info_native_explicit_snapshot_min.hako
  uses info.with_explicit_carriers_from_snapshot("i", snapshot)

bash tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh
  from_snapshot_exe=green
  explicit_snapshot_exe=green
```

## Stop Line

```text
do_not_add_by_name_fallback=1
do_not_widen_RuntimeDataBox_dispatch=1
do_not_change_CarrierInfo_semantics=1
do_not_touch_converter_ownership_policy=1
```
