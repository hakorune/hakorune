#!/usr/bin/env python3
"""Collect native-Linux separate-process S6C PMU evidence atomically."""

import argparse
import copy
import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics
import subprocess
import sys
import tempfile


CASE = "mixed/4096/first"
PAIR_COUNT = 51
RUN_COUNT = 3
MIN_ELAPSED_NS = 30_000_000
PRIMARY = ("cycles:u", "instructions:u", "branches:u", "branch-misses:u")
FRONTEND = (
    "cycles:u", "stalled-cycles-frontend:u", "L1-icache-load-misses:u", "iTLB-load-misses:u"
)
EPOCHS = {"primary": PRIMARY, "frontend": FRONTEND}
T_CRITICAL_95_DF50 = 2.009575


class NoSafeSlice(RuntimeError):
    pass


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def command(*argv: str, check: bool = True) -> str:
    result = subprocess.run(argv, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if check and result.returncode:
        raise NoSafeSlice(f"command failed ({' '.join(argv)}): {result.stderr.strip()}")
    return result.stdout.strip()


def virtualization_reason(uname: str, lscpu: str, cgroup: str, detected: str) -> str | None:
    combined = f"{uname}\n{lscpu}\n{cgroup}".lower()
    if re.search(r"^hypervisor vendor:", lscpu, re.MULTILINE | re.IGNORECASE):
        return "lscpu exposes Hypervisor vendor"
    if re.search(r"^flags:.*\bhypervisor\b", lscpu, re.MULTILINE | re.IGNORECASE):
        return "CPU hypervisor flag present"
    if "microsoft" in combined or "wsl" in combined:
        return "WSL kernel/environment detected"
    if re.search(r"/(docker|lxc|kubepods|containerd)(/|$)", cgroup, re.IGNORECASE):
        return "container cgroup detected"
    if detected and detected != "none":
        return f"systemd-detect-virt={detected}"
    return None


def environment(cpu: int, clang: Path) -> dict[str, object]:
    uname = command("uname", "-a")
    lscpu = command("lscpu")
    cgroup = Path("/proc/1/cgroup").read_text()
    detected_result = subprocess.run(
        ["systemd-detect-virt"], text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE
    )
    detected = detected_result.stdout.strip()
    reason = virtualization_reason(uname, lscpu, cgroup, detected)
    if reason:
        raise NoSafeSlice(reason)
    allowed = os.sched_getaffinity(0)
    if cpu not in allowed:
        raise NoSafeSlice(f"CPU {cpu} is outside allowed affinity {sorted(allowed)}")
    paranoid = int(Path("/proc/sys/kernel/perf_event_paranoid").read_text().strip())
    if paranoid > 2:
        raise NoSafeSlice(f"perf_event_paranoid={paranoid} blocks user counters")
    model_match = re.search(r"^Model name:\s*(.+)$", lscpu, re.MULTILINE)
    if not model_match:
        raise NoSafeSlice("CPU model missing from lscpu")
    return {
        "uname": uname,
        "kernel": os.uname().release,
        "cpu_model": model_match.group(1).strip(),
        "lscpu_sha256": hashlib.sha256(lscpu.encode()).hexdigest(),
        "perf_version": command("perf", "--version"),
        "perf_event_paranoid": paranoid,
        "clang_version": command(str(clang), "--version").splitlines()[0],
        "rustc_version": command("rustc", "--version"),
        "cargo_version": command("cargo", "--version"),
        "hypervisor_absent": {
            "lscpu_hypervisor_vendor": False,
            "cpu_hypervisor_flag": False,
            "systemd_detect_virt": detected or "none",
            "wsl_markers": False,
            "container_cgroup": False,
        },
    }


def symbol_evidence(binary: Path) -> tuple[str, dict[str, dict[str, object]]]:
    notes = command("readelf", "-n", str(binary))
    build_match = re.search(r"Build ID:\s*([0-9a-f]+)", notes)
    if not build_match:
        raise NoSafeSlice("binary build-id missing")
    nm = command("nm", "-n", str(binary))
    symbols: dict[str, dict[str, object]] = {}
    for name in ("hako_s6c_meso", "hako_s6c_c_meso"):
        matches = re.findall(rf"^([0-9a-fA-F]+)\s+[Tt]\s+{name}$", nm, re.MULTILINE)
        if len(matches) != 1:
            raise NoSafeSlice(f"missing/duplicate symbol {name}")
        address = int(matches[0], 16)
        disassembly = command("objdump", "-d", "--no-show-raw-insn", f"--disassemble={name}", str(binary))
        body = []
        for line in disassembly.splitlines():
            match = re.match(r"^\s*[0-9a-f]+:\s+(.+)$", line)
            if match:
                body.append(re.sub(r"\b[0-9a-f]+\s+<[^>]+>", "TARGET", match.group(1)))
        if not body or any(re.search(r"\bcallq?\b", row) for row in body):
            raise NoSafeSlice(f"empty body or call/trampoline in {name}")
        symbols[name] = {
            "address": address,
            "address_mod_64": address % 64,
            "body_sha256": hashlib.sha256("\n".join(body).encode()).hexdigest(),
        }
    if any(row["address_mod_64"] != 0 for row in symbols.values()):
        raise NoSafeSlice("Hako/C symbols are not both 64-byte aligned")
    return build_match.group(1), symbols


def validate_alignment_manifest(path: Path, symbols: dict[str, dict[str, object]]) -> None:
    try:
        manifest = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise NoSafeSlice(f"alignment manifest unreadable: {error}") from error
    if manifest.get("schema") != "s6c-pinned-corridor-meso-alignment-evidence-v1":
        raise NoSafeSlice("alignment manifest schema drift")
    if manifest.get("symbols") != symbols:
        raise NoSafeSlice("alignment manifest does not match measured binary")


def validate_sample(sample: dict[str, object], arm: str, iterations: int, cpu: int) -> None:
    if sample.get("schema") != "s6c-meso-separate-arm-sample-v1":
        raise NoSafeSlice("sample schema drift")
    expected = {"arm": arm, "case": CASE, "iterations": iterations, "cpu": cpu, "affinity_count": 1}
    for key, value in expected.items():
        if sample.get(key) != value:
            raise NoSafeSlice(f"{key} drift: expected {value}, got {sample.get(key)}")
    if sample.get("result") != sample.get("parity_result"):
        raise NoSafeSlice("result mismatch")
    if not isinstance(sample.get("input_fingerprint"), str) or len(sample["input_fingerprint"]) != 16:
        raise NoSafeSlice("input fingerprint missing")
    for epoch, names in EPOCHS.items():
        group = sample.get(epoch)
        if not isinstance(group, dict) or group.get("group_event_count") != len(names):
            raise NoSafeSlice(f"{epoch} missing/excess event")
        if group.get("time_enabled") != group.get("time_running") or not group.get("time_enabled"):
            raise NoSafeSlice(f"{epoch} multiplex/time scaling")
        for zero_field in ("lost_samples", "context_switches", "migrations"):
            if group.get(zero_field) != 0:
                raise NoSafeSlice(f"{epoch} {zero_field} is nonzero")
        if not isinstance(group.get("elapsed_ns"), int) or group["elapsed_ns"] < MIN_ELAPSED_NS:
            raise NoSafeSlice(f"{epoch} call loop shorter than 30ms")
        events = group.get("events")
        if not isinstance(events, list) or [row.get("name") for row in events] != list(names):
            raise NoSafeSlice(f"{epoch} event order/set drift")
        ids = []
        for row in events:
            if row.get("expected_id") != row.get("read_id"):
                raise NoSafeSlice(f"{epoch} event ID drift")
            if not isinstance(row.get("count"), int) or row["count"] < 0:
                raise NoSafeSlice(f"{epoch} invalid raw count")
            ids.append(row["read_id"])
        if len(set(ids)) != len(names):
            raise NoSafeSlice(f"{epoch} duplicate event ID")


def run_arm(binary: Path, arm: str, iterations: int, cpu: int) -> dict[str, object]:
    process = subprocess.run(
        ["taskset", "-c", str(cpu), str(binary), "--arm", arm, "--case", CASE,
         "--iterations", str(iterations)],
        text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )
    if process.returncode:
        detail = process.stderr.strip() or f"exit {process.returncode}"
        raise NoSafeSlice(f"{arm} process rejected: {detail}")
    lines = process.stdout.splitlines()
    if len(lines) != 1:
        raise NoSafeSlice(f"{arm} process did not emit exactly one atomic sample")
    try:
        sample = json.loads(lines[0])
    except json.JSONDecodeError as error:
        raise NoSafeSlice(f"{arm} sample JSON invalid: {error}") from error
    validate_sample(sample, arm, iterations, cpu)
    return sample


def event_counts(sample: dict[str, object]) -> dict[str, int]:
    counts = {}
    for epoch, names in EPOCHS.items():
        rows = sample[epoch]["events"]
        for name, row in zip(names, rows):
            counts[f"{epoch}/{name}"] = row["count"]
    return counts


def paired_ratios(hako: dict[str, object], c_arm: dict[str, object], iterations: int) -> dict[str, object]:
    if any(hako[key] != c_arm[key] for key in ("case", "iterations", "input_fingerprint", "result")):
        raise NoSafeSlice("arm/input/result/iteration mismatch within pair")
    hako_counts, c_counts = event_counts(hako), event_counts(c_arm)
    ratios = {}
    per_invocation = {"hako": {}, "c": {}}
    for name in hako_counts:
        ratios[name] = hako_counts[name] / c_counts[name] \
            if c_counts[name] > 0 and hako_counts[name] > 0 else None
        per_invocation["hako"][name] = hako_counts[name] / iterations
        per_invocation["c"][name] = c_counts[name] / iterations
    return {"ratios": ratios, "per_invocation": per_invocation}


def interval(ratios: list[float | None]) -> dict[str, float | str | int | bool | None]:
    if len(ratios) != PAIR_COUNT:
        raise NoSafeSlice("paired ratio sample count drift")
    invalid = sum(value is None or value <= 0 or not math.isfinite(value) for value in ratios)
    if invalid:
        return {"method": "paired-log-ratio-t95", "estimable": False,
                "invalid_zero_count_pairs": invalid, "geometric_mean": None, "median": None,
                "lower": None, "upper": None}
    logs = [math.log(value) for value in ratios]
    mean = statistics.mean(logs)
    margin = T_CRITICAL_95_DF50 * statistics.stdev(logs) / math.sqrt(len(logs))
    return {
        "method": "paired-log-ratio-t95",
        "estimable": True,
        "invalid_zero_count_pairs": 0,
        "geometric_mean": math.exp(mean),
        "median": statistics.median(ratios),
        "lower": math.exp(mean - margin),
        "upper": math.exp(mean + margin),
    }


def direction(row: dict[str, object]) -> int:
    if not row.get("estimable"):
        return 0
    if row["lower"] > 1.0:
        return 1
    if row["upper"] < 1.0:
        return -1
    return 0


def classify(summary: dict[str, dict[str, object]]) -> dict[str, object]:
    primary_cycle = direction(summary["primary/cycles:u"])
    frontend_cycle = direction(summary["frontend/cycles:u"])
    if not primary_cycle or frontend_cycle != primary_cycle:
        return {"name": "NoSafeSlice", "drivers": [], "direction": 0}
    schedule = [name for name in ("primary/instructions:u", "primary/branches:u")
                if direction(summary[name]) == primary_cycle]
    if schedule:
        return {"name": "physical instruction schedule candidate", "drivers": schedule,
                "direction": primary_cycle}
    branch = "primary/branch-misses:u"
    if direction(summary[branch]) == primary_cycle:
        return {"name": "branch layout candidate", "drivers": [branch], "direction": primary_cycle}
    if any(direction(summary[name]) for name in ("primary/instructions:u", "primary/branches:u")):
        return {"name": "NoSafeSlice", "drivers": [], "direction": 0}
    frontend = [name for name in (
        "frontend/stalled-cycles-frontend:u", "frontend/L1-icache-load-misses:u",
        "frontend/iTLB-load-misses:u") if direction(summary[name]) == primary_cycle]
    if frontend:
        return {"name": "frontend placement candidate", "drivers": frontend,
                "direction": primary_cycle}
    return {"name": "NoSafeSlice", "drivers": [], "direction": 0}


def collect(binary: Path, iterations: int, cpu: int) -> tuple[list[dict[str, object]], dict[str, object]]:
    runs = []
    for run_index in range(RUN_COUNT):
        pairs = []
        ratio_sets = {f"{epoch}/{name}": [] for epoch, names in EPOCHS.items() for name in names}
        for pair_index in range(PAIR_COUNT):
            order = ("hako", "c") if pair_index % 2 == 0 else ("c", "hako")
            samples = {arm: run_arm(binary, arm, iterations, cpu) for arm in order}
            paired = paired_ratios(samples["hako"], samples["c"], iterations)
            for name, value in paired["ratios"].items():
                ratio_sets[name].append(value)
            pairs.append({"pair": pair_index, "order": list(order), "hako": samples["hako"],
                          "c": samples["c"], **paired})
        summary = {name: interval(values) for name, values in ratio_sets.items()}
        runs.append({"run": run_index, "pairs": pairs, "summary": summary,
                     "classification": classify(summary)})
    classes = [run["classification"] for run in runs]
    names = {row["name"] for row in classes}
    directions = {row["direction"] for row in classes}
    shared_drivers = set(classes[0]["drivers"]).intersection(*(set(row["drivers"]) for row in classes[1:]))
    if len(names) == 1 and "NoSafeSlice" not in names and len(directions) == 1 and shared_drivers:
        final = {"name": classes[0]["name"], "direction": classes[0]["direction"],
                 "reproduced_runs": RUN_COUNT, "drivers": sorted(shared_drivers),
                 "pc_attribution_candidate": True,
                 "next_task": "S6C-MESO-HWCOUNTER-PC-ATTRIBUTION-A0"}
    else:
        final = {"name": "NoSafeSlice", "direction": 0, "reproduced_runs": 0,
                 "drivers": [], "pc_attribution_candidate": False,
                 "next_task": "NoSafeSlice"}
    return runs, final


def atomic_publish(path: Path, build) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    try:
        payload = build()
        temporary.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
        os.replace(temporary, path)
    except Exception:
        temporary.unlink(missing_ok=True)
        raise


def valid_fixture() -> dict[str, object]:
    sample = {"schema": "s6c-meso-separate-arm-sample-v1", "arm": "hako", "case": CASE,
              "iterations": 10, "cpu": 2, "affinity_count": 1, "input_fingerprint": "0" * 16,
              "result": 0, "parity_result": 0, "sink": 1}
    next_id = 1
    for epoch, names in EPOCHS.items():
        events = []
        for name in names:
            events.append({"name": name, "expected_id": next_id, "read_id": next_id, "count": 100})
            next_id += 1
        sample[epoch] = {"group_event_count": 4, "time_enabled": 40, "time_running": 40,
                         "lost_samples": 0, "context_switches": 0, "migrations": 0,
                         "elapsed_ns": MIN_ELAPSED_NS, "events": events}
    return sample


def self_test() -> None:
    base = valid_fixture()
    validate_sample(base, "hako", 10, 2)
    negatives = {
        "wrong arm": lambda row: row.update(arm="c"),
        "wrong case": lambda row: row.update(case="mixed/4096/miss"),
        "iteration drift": lambda row: row.update(iterations=11),
        "result mismatch": lambda row: row.update(parity_result=1),
        "event ID drift": lambda row: row["primary"]["events"][0].update(read_id=99),
        "missing event": lambda row: row["primary"].update(events=row["primary"]["events"][:-1]),
        "multiplex/time scaling": lambda row: row["primary"].update(time_running=39),
        "migration/context-switch": lambda row: row["frontend"].update(context_switches=1),
    }
    for name, mutate in negatives.items():
        row = copy.deepcopy(base)
        mutate(row)
        try:
            validate_sample(row, "hako", 10, 2)
        except NoSafeSlice:
            continue
        raise AssertionError(f"negative accepted: {name}")
    if not virtualization_reason("Linux", "Hypervisor vendor: KVM", "0::/", "kvm"):
        raise AssertionError("hypervisor negative accepted")
    with tempfile.TemporaryDirectory() as directory:
        report = Path(directory) / "evidence.json"
        try:
            atomic_publish(report, lambda: (_ for _ in ()).throw(NoSafeSlice("partial")))
        except NoSafeSlice:
            pass
        if report.exists() or report.with_name(report.name + ".tmp").exists():
            raise AssertionError("partial report publication")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--alignment-manifest", type=Path)
    parser.add_argument("--report", type=Path)
    parser.add_argument("--commit")
    parser.add_argument("--cpu", type=int)
    parser.add_argument("--iterations", type=int)
    parser.add_argument("--clang", type=Path)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--write-alignment-manifest", action="store_true")
    parser.add_argument("--probe", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        print("[s6c-native-hwcounter] self-test ok")
        return 0
    if args.write_alignment_manifest:
        if not args.binary or not args.alignment_manifest:
            parser.error("alignment publication requires binary and alignment manifest")
        try:
            build_id, symbols = symbol_evidence(args.binary)
            atomic_publish(args.alignment_manifest, lambda: {
                "schema": "s6c-pinned-corridor-meso-alignment-evidence-v1",
                "binary_sha256": sha256(args.binary), "build_id": build_id, "symbols": symbols,
            })
            return 0
        except (NoSafeSlice, OSError, ValueError) as error:
            print(f"[s6c-native-hwcounter] NoSafeSlice: {error}", file=sys.stderr)
            return 1
    if args.probe:
        if any(value is None for value in (
                args.binary, args.alignment_manifest, args.cpu, args.iterations, args.clang)):
            parser.error("probe requires binary, alignment manifest, cpu, iterations, clang")
        try:
            environment(args.cpu, args.clang)
            _, symbols = symbol_evidence(args.binary)
            validate_alignment_manifest(args.alignment_manifest, symbols)
            hako = run_arm(args.binary, "hako", args.iterations, args.cpu)
            c_arm = run_arm(args.binary, "c", args.iterations, args.cpu)
            paired = paired_ratios(hako, c_arm, args.iterations)
            print(json.dumps({"hako": hako, "c": c_arm, **paired}, sort_keys=True))
            return 0
        except (NoSafeSlice, OSError, ValueError, KeyError, json.JSONDecodeError) as error:
            print(f"[s6c-native-hwcounter] NoSafeSlice: {error}", file=sys.stderr)
            return 1
    required = (args.binary, args.alignment_manifest, args.report, args.commit,
                args.cpu, args.iterations, args.clang)
    if any(value is None for value in required):
        parser.error("collection requires binary, alignment manifest, report, commit, cpu, iterations, clang")
    try:
        host = environment(args.cpu, args.clang)
        build_id, symbols = symbol_evidence(args.binary)
        validate_alignment_manifest(args.alignment_manifest, symbols)

        def build_report() -> dict[str, object]:
            runs, classification = collect(args.binary, args.iterations, args.cpu)
            fingerprints = {pair[arm]["input_fingerprint"] for run in runs for pair in run["pairs"]
                            for arm in ("hako", "c")}
            if len(fingerprints) != 1:
                raise NoSafeSlice("input fingerprint drift across samples")
            return {
                "schema": "s6c-meso-native-hwcounter-evidence-v1",
                "authority": "promotion-evidence-only",
                "commit": args.commit,
                "environment": host,
                "affinity": {"cpu": args.cpu, "method": "taskset separate process",
                             "allowed_at_collection": sorted(os.sched_getaffinity(0))},
                "binary": {"path": str(args.binary.resolve()), "sha256": sha256(args.binary),
                           "build_id": build_id},
                "symbols": symbols,
                "workload": {"corpus": CASE, "iterations": args.iterations,
                             "input_fingerprint": next(iter(fingerprints)), "paired_runs": RUN_COUNT,
                             "pairs_per_run": PAIR_COUNT, "unchanged_meso_threshold": 1.15},
                "counter_contract": {"epochs": {key: list(value) for key, value in EPOCHS.items()},
                                     "exclude_kernel": True, "exclude_hv": True,
                                     "raw_event_fallback": False, "read_format": ["GROUP", "ID",
                                     "TOTAL_TIME_ENABLED", "TOTAL_TIME_RUNNING"]},
                "runs": runs,
                "classification": classification,
                "decision": "PC attribution candidate" if classification["pc_attribution_candidate"]
                            else "NoSafeSlice",
            }

        atomic_publish(args.report, build_report)
        print(f"[s6c-native-hwcounter] ok: {args.report}")
        return 0
    except (NoSafeSlice, OSError, ValueError, KeyError, json.JSONDecodeError) as error:
        if args.report:
            args.report.with_name(args.report.name + ".tmp").unlink(missing_ok=True)
        print(f"[s6c-native-hwcounter] NoSafeSlice: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
