"""
MIRCall intrinsic registry (SSOT).

This module centralizes method-name based intrinsic classifications used by
LLVM MIRCall lowering. The goal is to avoid scattering hard-coded method lists
across lowering modules. Plugin/by-name String-return contracts also belong
here, so boxcall/method_call lowering can share one string-result SSOT.
"""

from typing import Dict, FrozenSet, Iterable, NamedTuple, Optional, Sequence, Tuple


TAG_LENGTH_LIKE = "length-like"
TAG_STRING_RECEIVER_REQUIRED = "receiver-string-tag-required"
TAG_STRING_RESULT = "string-result"
TAG_INTRINSIC_CANDIDATE = "intrinsic-candidate"
_ALLOWED_TAGS = frozenset(
    (
        TAG_LENGTH_LIKE,
        TAG_STRING_RECEIVER_REQUIRED,
        TAG_STRING_RESULT,
        TAG_INTRINSIC_CANDIDATE,
    )
)


class IntrinsicSpec(NamedTuple):
    method: str
    arity: Optional[int]
    symbol: Optional[str]
    tags: FrozenSet[str]


def _spec(
    method: str,
    *,
    arity: Optional[int] = None,
    symbol: Optional[str] = None,
    tags: Sequence[str] = (),
) -> IntrinsicSpec:
    normalized_method = str(method or "")
    normalized_symbol = None if symbol is None else str(symbol)
    normalized_arity = None if arity is None else int(arity)
    normalized_tags = frozenset(str(tag) for tag in tags)
    return IntrinsicSpec(
        method=normalized_method,
        arity=normalized_arity,
        symbol=normalized_symbol,
        tags=normalized_tags,
    )


# Declarative intrinsic table (SSOT).
_INTRINSIC_SPECS: Tuple[IntrinsicSpec, ...] = (
    _spec(
        "length",
        arity=0,
        symbol="nyash.any.length_h",
        tags=(TAG_LENGTH_LIKE, TAG_INTRINSIC_CANDIDATE),
    ),
    _spec(
        "len",
        arity=0,
        symbol="nyash.any.length_h",
        tags=(TAG_LENGTH_LIKE, TAG_INTRINSIC_CANDIDATE),
    ),
    _spec(
        "size",
        arity=0,
        symbol="nyash.any.length_h",
        tags=(TAG_LENGTH_LIKE, TAG_INTRINSIC_CANDIDATE),
    ),
    _spec(
        "substring",
        arity=2,
        symbol="nyash.string.substring_hii",
        tags=(
            TAG_STRING_RECEIVER_REQUIRED,
            TAG_STRING_RESULT,
            TAG_INTRINSIC_CANDIDATE,
        ),
    ),
    _spec(
        "indexOf",
        arity=1,
        symbol="nyash.string.indexOf_hh",
        tags=(TAG_STRING_RECEIVER_REQUIRED, TAG_INTRINSIC_CANDIDATE),
    ),
    _spec(
        "lastIndexOf",
        arity=1,
        symbol="nyash.string.lastIndexOf_hh",
        tags=(TAG_STRING_RECEIVER_REQUIRED, TAG_INTRINSIC_CANDIDATE),
    ),
    _spec("esc_json", tags=(TAG_STRING_RESULT,)),
    _spec("node_json", tags=(TAG_STRING_RESULT,)),
    _spec("dirname", tags=(TAG_STRING_RESULT,)),
    _spec("join", tags=(TAG_STRING_RESULT,)),
    _spec("read_all", tags=(TAG_STRING_RESULT,)),
    _spec("toString", tags=(TAG_STRING_RESULT,)),
    _spec("stringify", tags=(TAG_STRING_RESULT,)),
    _spec("str", tags=(TAG_STRING_RESULT,)),
    _spec("resolve_for_source", tags=(TAG_STRING_RESULT,)),
    _spec("emit_program_json_v0", tags=(TAG_STRING_RESULT,)),
    _spec("emit_from_source_v0", tags=(TAG_STRING_RESULT,)),
    _spec("toJson", tags=(TAG_STRING_RESULT,)),
)


def _normalize(method: Optional[str]) -> str:
    if method is None:
        return ""
    return str(method)


def validate_intrinsic_specs(specs: Iterable[IntrinsicSpec]) -> Tuple[str, ...]:
    errors = []
    seen_method_arity = {}

    for idx, spec in enumerate(specs):
        method = _normalize(spec.method)
        if not method:
            errors.append(f"spec[{idx}] has empty method")

        if spec.arity is not None and int(spec.arity) < 0:
            errors.append(f"spec[{idx}] has negative arity: {spec.arity}")

        if spec.symbol is not None and not str(spec.symbol).strip():
            errors.append(f"spec[{idx}] has empty symbol")

        unknown_tags = sorted(tag for tag in spec.tags if tag not in _ALLOWED_TAGS)
        if unknown_tags:
            errors.append(f"spec[{idx}] has unknown tags: {','.join(unknown_tags)}")

        if TAG_INTRINSIC_CANDIDATE in spec.tags:
            if spec.arity is None:
                errors.append(
                    f"spec[{idx}] intrinsic-candidate requires explicit arity: method={method}"
                )
            if spec.symbol is None:
                errors.append(
                    f"spec[{idx}] intrinsic-candidate requires symbol: method={method}"
                )

        if spec.symbol is not None and spec.arity is None:
            errors.append(
                f"spec[{idx}] symbol requires explicit arity: method={method} symbol={spec.symbol}"
            )

        if TAG_LENGTH_LIKE in spec.tags and spec.arity != 0:
            errors.append(
                f"spec[{idx}] length-like must be arity=0: method={method} arity={spec.arity}"
            )

        if spec.arity is not None:
            key = (method, int(spec.arity))
            prev_idx = seen_method_arity.get(key)
            if prev_idx is not None:
                errors.append(
                    "duplicate method/arity entry: "
                    f"method={method} arity={spec.arity} first=spec[{prev_idx}] second=spec[{idx}]"
                )
            else:
                seen_method_arity[key] = idx

    return tuple(errors)


def _build_indexes(specs: Iterable[IntrinsicSpec]):
    by_method = {}
    by_method_arity = {}
    for spec in specs:
        method = _normalize(spec.method)
        if not method:
            continue
        bucket = by_method.get(method)
        if bucket is None:
            bucket = []
            by_method[method] = bucket
        bucket.append(spec)
        if spec.arity is not None:
            by_method_arity[(method, int(spec.arity))] = spec

    by_method_out = {}
    for method, bucket in by_method.items():
        by_method_out[method] = tuple(bucket)

    return by_method_out, by_method_arity


_REGISTRY_ERRORS = validate_intrinsic_specs(_INTRINSIC_SPECS)
if _REGISTRY_ERRORS:
    raise RuntimeError(
        "MIRCall intrinsic registry contract violation: " + "; ".join(_REGISTRY_ERRORS)
    )

_SPECS_BY_METHOD, _SPECS_BY_METHOD_ARITY = _build_indexes(_INTRINSIC_SPECS)


def get_registry_consistency_errors() -> Tuple[str, ...]:
    return _REGISTRY_ERRORS


def iter_intrinsic_specs() -> Tuple[IntrinsicSpec, ...]:
    return _INTRINSIC_SPECS


def lookup_intrinsic_spec(method: Optional[str], arity: Optional[int] = None) -> Optional[IntrinsicSpec]:
    method_name = _normalize(method)
    if not method_name:
        return None

    if arity is not None:
        try:
            normalized_arity = int(arity)
        except (TypeError, ValueError):
            return None
        return _SPECS_BY_METHOD_ARITY.get((method_name, normalized_arity))

    specs = _SPECS_BY_METHOD.get(method_name)
    if not specs:
        return None
    return specs[0]


def _method_has_tag(method: Optional[str], tag: str) -> bool:
    method_name = _normalize(method)
    if not method_name:
        return False
    specs = _SPECS_BY_METHOD.get(method_name, ())
    for spec in specs:
        if tag in spec.tags:
            return True
    return False


def is_length_like_method(method: Optional[str]) -> bool:
    return _method_has_tag(method, TAG_LENGTH_LIKE)


def requires_string_receiver_tag(method: Optional[str]) -> bool:
    return _method_has_tag(method, TAG_STRING_RECEIVER_REQUIRED)


def produces_string_result(method: Optional[str]) -> bool:
    return _method_has_tag(method, TAG_STRING_RESULT)
