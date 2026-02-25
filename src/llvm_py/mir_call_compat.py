"""
Canonical callee normalization for MIR Call instructions.

Legacy keys are rejected (sunset):
- method
- box_type
- function_value
- func
"""

from typing import Dict, Any


class MirCallCompat:
    """Canonical callee normalizer for MIR Call instructions."""

    LEGACY_KEYS = frozenset({"method", "box_type", "function_value", "func"})

    @staticmethod
    def _reject_legacy_keys(callee_json: Dict[str, Any]) -> None:
        legacy = sorted(k for k in callee_json.keys() if k in MirCallCompat.LEGACY_KEYS)
        if legacy:
            raise ValueError(
                f"Legacy mir_call callee keys are not supported: {legacy} (callee={callee_json})"
            )

    @staticmethod
    def normalize_callee(callee_json: Dict[str, Any]) -> Dict[str, Any]:
        """
        Normalize canonical callee format to unified internal format.

        Args:
            callee_json: Callee dict from MIR Call instruction

        Returns:
            Normalized dict with consistent keys:
            {
                "type": "Global" | "Method" | "Constructor" | "Closure" | "Value" | "Extern",
                "name": str (function/method name),
                "box_name": str (for Method/Constructor),
                "receiver": int (for Method),
                ... other fields
            }

        Examples:
            {"name": "log", "box_name": "ConsoleBox", "type": "Method"}
            → {"type": "Method", "name": "log", "box_name": "ConsoleBox"}
        """
        if not callee_json:
            return {}
        MirCallCompat._reject_legacy_keys(callee_json)

        # Extract type (canonical)
        callee_type = callee_json.get("type")

        # Build normalized result
        normalized = {
            "type": callee_type,
        }

        # Normalize fields by callee kind (canonical keys only).
        if callee_type == "Method":
            name = callee_json.get("name")
            box_name = callee_json.get("box_name")
            if name is not None:
                normalized["name"] = name
            if box_name is not None:
                normalized["box_name"] = box_name
        elif callee_type == "Constructor":
            ctor_name = callee_json.get("name")
            if ctor_name is not None:
                normalized["name"] = ctor_name
        elif callee_type == "Value":
            func_value = callee_json.get("value")
            if func_value is not None:
                normalized["value"] = func_value
        else:
            # Global / Extern / fallback
            name = callee_json.get("name")
            if name is not None:
                normalized["name"] = name

        # Copy other fields as-is
        for key in ["receiver", "params", "captures", "me_capture", "certainty"]:
            if key in callee_json:
                normalized[key] = callee_json[key]

        return normalized

    @staticmethod
    def detect_format_version(callee_json: Dict[str, Any]) -> int:
        """
        Detect whether callee JSON is canonical format.

        Args:
            callee_json: Callee dict from MIR Call instruction

        Returns:
            1 for canonical format.

        Raises:
            ValueError: If format is legacy or invalid
        """
        if not callee_json:
            raise ValueError("Empty callee JSON")

        MirCallCompat._reject_legacy_keys(callee_json)

        # Check canonical markers
        if "name" in callee_json or "box_name" in callee_json or "value" in callee_json:
            return 1

        # No clear markers - check callee type
        if callee_json.get("type") in ["Global", "Method", "Constructor", "Extern"]:
            # Canonical format
            return 1

        raise ValueError(f"Unknown callee format: {callee_json}")

    @staticmethod
    def is_v0_format(callee_json: Dict[str, Any]) -> bool:
        """Legacy v0 is sunset; always False for valid inputs."""
        try:
            MirCallCompat.detect_format_version(callee_json)
            return False
        except ValueError:
            return False

    @staticmethod
    def is_v1_format(callee_json: Dict[str, Any]) -> bool:
        """Check if callee JSON uses v1 format."""
        try:
            return MirCallCompat.detect_format_version(callee_json) == 1
        except ValueError:
            return False
