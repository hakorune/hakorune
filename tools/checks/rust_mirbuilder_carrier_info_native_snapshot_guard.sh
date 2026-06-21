#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

SNAPSHOT_SOURCE="apps/tests/phase296x_carrier_info_native_snapshot_min.hako"
EXPLICIT_SOURCE="apps/tests/phase296x_carrier_info_native_explicit_snapshot_min.hako"
SNAPSHOT_EXE="/tmp/phase296x_carrier_info_native_snapshot_min.exe"
EXPLICIT_EXE="/tmp/phase296x_carrier_info_native_explicit_snapshot_min.exe"

rm -f "$SNAPSHOT_EXE" "$EXPLICIT_EXE"

python3 - <<'PY'
from pathlib import Path

source = Path("apps/lib/hakorune_mir_builder/carrier_info.hako").read_text()
assert "box CarrierInfoNative" in source
assert "static box CarrierInfoNativeApi" in source
assert "from_snapshot(loop_var_name, snapshot: OrderedMapBox): i64" in source
assert "with_explicit_carriers_from_snapshot(loop_var_name, snapshot: OrderedMapBox): i64" in source
assert "from_snapshot(info: CarrierInfoNative, loop_var_name, snapshot: OrderedMapBox): i64" in source
assert "with_explicit_carriers_from_snapshot(info: CarrierInfoNative, loop_var_name, snapshot: OrderedMapBox): i64" in source
assert "return ctx.variable_map" not in source
assert "OrderedMapReadViewBox" not in source

snapshot_test = Path("apps/tests/phase296x_carrier_info_native_snapshot_min.hako").read_text()
explicit_test = Path("apps/tests/phase296x_carrier_info_native_explicit_snapshot_min.hako").read_text()
assert 'info.from_snapshot("i", snapshot)' in snapshot_test
assert "CarrierInfoNativeApi.from_snapshot(info" not in snapshot_test
assert 'info.with_explicit_carriers_from_snapshot("i", snapshot)' in explicit_test
assert 'missing_info.with_explicit_carriers_from_snapshot("i", snapshot)' in explicit_test
assert "CarrierInfoNativeApi.with_explicit_carriers_from_snapshot(info" not in explicit_test
PY

./target/release/hakorune --emit-exe "$SNAPSHOT_EXE" "$SNAPSHOT_SOURCE" >/tmp/phase296x_carrier_info_native_snapshot_min.build.log 2>&1
"$SNAPSHOT_EXE" >/tmp/phase296x_carrier_info_native_snapshot_min.run.log 2>&1
grep -Fq "carrier_info_native_snapshot=ok" /tmp/phase296x_carrier_info_native_snapshot_min.run.log

./target/release/hakorune --emit-exe "$EXPLICIT_EXE" "$EXPLICIT_SOURCE" >/tmp/phase296x_carrier_info_native_explicit_snapshot_min.build.log 2>&1
"$EXPLICIT_EXE" >/tmp/phase296x_carrier_info_native_explicit_snapshot_min.run.log 2>&1
grep -Fq "carrier_info_native_explicit_snapshot=ok" /tmp/phase296x_carrier_info_native_explicit_snapshot_min.run.log
grep -Fq "carrier_info_native_missing_requested_carrier=fail" /tmp/phase296x_carrier_info_native_explicit_snapshot_min.run.log

cat <<'REPORT'
output_contract=rust-mirbuilder-carrier-info-native-snapshot-v0
carrier_info_native_source=green
from_snapshot_exe=green
explicit_snapshot_exe=green
imported_instance_method_route=green
missing_requested_carrier_fail_fast=green
snapshot_context_output_alias_isolation=green
raw_variable_map_alias=0
ordered_map_read_view_box=0
summary=ok
REPORT
