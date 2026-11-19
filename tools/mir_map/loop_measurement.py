"""Collect the existing CLI's verification, EXE, and parser observations."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import hashlib
import json
import re
import time

from comparison_policy import env_fingerprint, exact_mir
from resources import activate
from support import ROOT, configured_env, read_manifest, sha, write

VERIFIED = "✅ MIR verification passed!"
DIAGNOSTIC = {
    "parser-refusal": r"(?:Parse error|Parser error|using prelude parse)",
    "internal-verification-error": (
        r"Control flow error in block bb\d+|"
        r"MIR verification failed|\[mir/verify:|/direct-verify\]|mir_verification_failed|"
        r"\[published-lifecycle-(?:program|physical-abi)/|IntegrityInvalid|"
        r"\[ordinary-new/local-commit/physical-boundary/"
    ),
    "environment-error": r"MIR VM reference execution is not available|Error reading file|Failed to load runtime",
    "named-source-refusal": (
        r"observation: Located \{ cause: (?:UnresolvedName|UnsupportedStatement|UnsupportedExpression)\b|"
        r"\[ownership/move-not-supported\]|"
        r"\[ordinary-new/borrowed-compare/integer-source-unproved\]|"
        r"using: duplicate import of|\[using/lineage\]\[DuplicateCanonicalPath\]"
    ),
}
PANIC = re.compile(r"(?m)^(?:thread .* (?:panicked at|has overflowed its stack)|fatal runtime error:|internal compiler error:)")


def diagnostic_kind(status, text):
    if status["timeout"]:
        return "timeout"
    if status["exit"] < 0 or PANIC.search(text):
        return "internal-verification-error"
    if status["exit"] == 0:
        return "success"
    for kind, pattern in DIAGNOSTIC.items():
        if re.search(pattern, text):
            return kind
    return "unknown-refusal"


def command(watch, args, directory, label, env, timeout):
    out, err = (directory / f"{label}.{suffix}" for suffix in ("out", "err"))
    started = time.monotonic()
    with out.open("wb") as stdout, err.open("wb") as stderr:
        child = watch.launch(list(map(str, args)), cwd=ROOT, env=env, stdout=stdout, stderr=stderr)
        code, timed, stopped, _ = watch.wait(child, timeout)
    if stopped:
        raise MemoryError(f"resource supervisor stopped {label}")
    return dict(exit=code, timeout=timed, stdout=str(out), stderr=str(err),
                seconds=time.monotonic() - started)


def stream_text(status):
    return "".join(Path(status[key]).read_text(errors="replace") for key in ("stdout", "stderr"))


def parse_evidence(status, phase):
    text = stream_text(status)
    kind = diagnostic_kind(status, text)
    if kind == "success" and phase in ("dump", "compile") and VERIFIED not in text:
        kind = "internal-verification-error"
    return dict(kind=kind, verified=VERIFIED in text)


class MeasurementIdentityChanged(ValueError):
    """A completed observation belongs to a different measured configuration."""


def measurement_configuration(binary, template, work):
    env = configured_env(template, binary, work)
    runtime = Path(env["NYASH_EMIT_EXE_NYRT"])
    artifacts = {"compiler": binary, "kernel": runtime / "libnyash_kernel.a",
                 "lifecycle": Path(env.get("HAKO_LIFECYCLE_NYRT", str(runtime))) / "libnyash_lifecycle_kernel.a",
                 "ffi": Path(env["HAKO_AOT_FFI_LIB"]), "llvm": Path(env["NYASH_NY_LLVM_COMPILER"])}
    return env, runtime, artifacts


def artifact_hashes(artifacts):
    return {str(path): sha(path) for path in artifacts.values()}


def collect(binary, rows, template, work, jobs, ast=False, watch=None, *, exe_baseline=None):
    binary, work = binary.resolve(), work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    env, runtime, artifacts = measurement_configuration(binary, template, work)
    hashes = artifact_hashes(artifacts)
    identity = dict(artifacts=hashes, rows=rows, ast=ast,
                    cli_sha256=hashes[str(binary)], execution_env=env_fingerprint(env),
                    artifact_roles={name: hashes[str(path)] for name, path in artifacts.items()},
                    settings={k:v for k,v in env.items() if k.startswith(("HAKO_", "NYASH_"))})
    if exe_baseline is not None:
        reuse_rows, baseline_id = exe_baseline
        identity['exe_baseline'] = baseline_id
    else:
        reuse_rows = {}
    manifest = work / "identity.json"
    if manifest.exists():
        held = json.loads(manifest.read_text())
        if (any(held[key] != identity[key] for key in ("artifacts", "settings"))
                or held.get("exe_baseline") != identity.get("exe_baseline")
                or ("execution_env" in held and held["execution_env"] != identity["execution_env"])):
            raise MeasurementIdentityChanged(f"measurement identity changed: {work}")
        complete = work / "results.json"
        if held["rows"] != rows or complete.exists() and not ast:
            if not complete.exists():
                raise ValueError(f"subset reuse requires a completed baseline: {work}")
            recorded = {row["path"]: row for row in json.loads(complete.read_text())}
            selected = []
            for row in rows:
                cached = recorded.get(row["path"])
                if not cached or cached["sha256"] != row["sha256"] or ast and "ast" not in cached:
                    raise ValueError(f"baseline lacks matching observations: {row['path']}")
                selected.append(cached | row)
            return selected
    else:
        write(manifest, identity)
    if watch is None:
        watch = activate(work, "loop-compare", max_children=jobs)

    def ensure_executable_mir(result, source, directory, receipt):
        # The closed executable route can accept source refused by the
        # standalone dump route. Ask that same route for its verified MIR.
        if result["dump"]["kind"] == "success" or "run" not in result or "executable_mir" in result:
            return result
        exe = directory / "program"
        exe.unlink(missing_ok=True)
        try:
            observation = command(watch, [binary, "--backend", "mir", "--verify", "--dump-mir",
                                           "--emit-exe", exe, "--emit-exe-nyrt", runtime, source],
                                  directory, "executable_mir", env, 180)
            observation.update(parse_evidence(observation, "compile"))
            if observation["kind"] == "success" and not exe.is_file():
                observation["kind"] = "internal-verification-error"
            result["executable_mir"] = observation
            write(receipt, result)
            return result
        finally:
            exe.unlink(missing_ok=True)

    def auxiliary(result, source, directory, receipt):
        if ast and not ("data" in result.get("ast", {}) or result.get("ast", {}).get("excluded")):
            if not re.search(rb"\bloop\b", Path(source).read_bytes()):
                result["ast"] = {"excluded": "source contains no loop token"}
            else:
                data = directory / "parser.json"
                data.unlink(missing_ok=True)
                result["ast"] = command(watch, [binary, "--emit-ast-json", data, source],
                                        directory, "parser", env, 30)
                result["ast"]["data"] = str(data)
            write(receipt, result)
        return ensure_executable_mir(result, source, directory, receipt)

    def measure(row):
        source = row["path"]
        key = hashlib.sha256(source.encode()).hexdigest()
        directory = work / "inputs" / key
        directory.mkdir(parents=True, exist_ok=True)
        receipt = directory / "result.json"
        if receipt.exists():
            result = json.loads(receipt.read_text())
            return auxiliary(result, source, directory, receipt)
        dump = command(watch, [binary, "--backend", "mir", "--dump-mir", "--verify", source],
                       directory, "dump", env, 180)
        dump.update(parse_evidence(dump, "dump"))
        prior = reuse_rows.get(source)
        if (prior and prior['sha256'] == row['sha256'] and dump['kind'] == 'success'
                and exact_mir(stream_text(prior['dump']), stream_text(dump))):
            result = row | dict(dump=dump, compile=prior['compile'], run=prior['run'],
                                exe_skipped="identical MIR")
            write(receipt, result)
            return auxiliary(result, source, directory, receipt)

        exe = directory / "program"
        exe.unlink(missing_ok=True)
        compile_result = command(
            watch, [binary, "--backend", "mir", "--verify", "--emit-exe", exe,
                    "--emit-exe-nyrt", runtime, source], directory, "compile", env, 180)
        compile_result.update(parse_evidence(compile_result, "compile"), emitted=exe.is_file())
        result = row | {"dump": dump, "compile": compile_result}
        if compile_result["kind"] == "success" and not compile_result["emitted"]:
            compile_result["kind"] = "internal-verification-error"
        if compile_result["kind"] == "success":
            try:
                run = command(watch, [exe], directory, "execution", env, 15)
                run["kind"] = "timeout" if run["timeout"] else (
                    "runtime-signal" if run["exit"] < 0 else "executed")
                result["run"] = run
            finally:
                exe.unlink(missing_ok=True)
        write(receipt, result)
        return auxiliary(result, source, directory, receipt)

    results = []
    with ThreadPoolExecutor(max_workers=jobs) as pool:
        for result in pool.map(measure, rows):
            results.append(result)
            if len(results) % 100 == 0:
                print(f"{work.name}: {len(results)}/{len(rows)}", flush=True)
    if identity["artifacts"] != artifact_hashes(artifacts):
        raise ValueError("compiler/runtime changed during measurement")
    write(work / "results.json", results)
    return results
