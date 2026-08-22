# lang/src/runtime/meta/generated

Generated compiler metadata derived from `lang/src/runtime/meta` owner boxes.

Rules:

- Do not hand-edit generated artifacts.
- Regenerate CoreMethodContract manifest with:
  `python3 tools/core_method_contract_manifest_codegen.py --write`
- Check drift with:
  `bash tools/checks/core_method_contract_manifest_guard.sh`
- Check the normalized TextScan contract with:
  `python3 tools/provider_slot_contract_manifest_codegen.py --check`
  and `bash tools/checks/dynamic_v2_aot_activation_authority_guard.sh`.
