import os
import llvmlite.binding as llvm

_OPT_ENV_KEYS = ("HAKO_LLVM_OPT_LEVEL", "NYASH_LLVM_OPT_LEVEL")
_FAST_NATIVE_ENV_KEYS = ("NYASH_LLVM_FAST_NATIVE", "HAKO_LLVM_FAST_NATIVE")


def parse_opt_level_env() -> int:
    """Return desired optimization level (0-3). Defaults to 2."""
    for key in _OPT_ENV_KEYS:
        raw = os.environ.get(key)
        if not raw:
            continue
        value = raw.strip()
        if not value:
            continue
        upper = value.upper()
        if upper.startswith("O"):
            value = upper[1:]
        try:
            lvl = int(value)
        except ValueError:
            continue
        if lvl < 0:
            lvl = 0
        if lvl > 3:
            lvl = 3
        return lvl
    return 2


def resolve_codegen_opt_level():
    """Map env level to llvmlite CodeGenOptLevel enum (fallback to int). Never returns None."""
    level = parse_opt_level_env()
    if level is None:
        level = 2
    try:
        names = {0: "None", 1: "Less", 2: "Default", 3: "Aggressive"}
        enum = getattr(llvm, "CodeGenOptLevel")
        attr = names.get(level, "Default")
        result = getattr(enum, attr)
        if result is None:
            return 2
        return result
    except Exception:
        return level if level is not None else 2


def _bool_env(key: str):
    raw = os.environ.get(key)
    if raw is None:
        return None
    value = raw.strip().lower()
    if value in ("1", "true", "yes", "on"):
        return True
    if value in ("0", "false", "no", "off"):
        return False
    return None


def fast_native_enabled() -> bool:
    # Fast-native target tuning is perf-lane only; default behavior stays unchanged.
    if os.environ.get("NYASH_LLVM_FAST") != "1":
        return False
    for key in _FAST_NATIVE_ENV_KEYS:
        value = _bool_env(key)
        if value is not None:
            return value
    return True


def resolve_target_machine_kwargs():
    kwargs = {"opt": resolve_codegen_opt_level()}
    if not fast_native_enabled():
        return kwargs

    try:
        cpu = llvm.get_host_cpu_name()
        if isinstance(cpu, bytes):
            cpu = cpu.decode("utf-8", "ignore")
        if cpu:
            kwargs["cpu"] = cpu
    except Exception:
        pass

    try:
        features = llvm.get_host_cpu_features()
        if hasattr(features, "flatten"):
            flattened = features.flatten()
            if flattened:
                kwargs["features"] = flattened
    except Exception:
        pass

    return kwargs


def create_target_machine_for_target(target):
    kwargs = resolve_target_machine_kwargs()
    try:
        return target.create_target_machine(**kwargs)
    except TypeError:
        # llvmlite compatibility fallback (e.g., older signatures).
        return target.create_target_machine(opt=resolve_codegen_opt_level())
