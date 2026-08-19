/* C1-A: selected Dynamic uses the exact MIR formal signature. */
if (ptfc_selected) {
  if (!hako_llvmc_ptfc_emit_selected_entry_header(f, &selection, &ptfc_candidate)) {
    note_pure_unsupported_shape(
        -1,
        0,
        "pinned_text_selected_entry_signature",
        "pinned_text_selected_parameter_signature_mismatch",
        NULL);
    HAKO_LLVMC_CLOSE_GENERIC_OUTPUT();
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }
} else if (hako_llvmc_selected_dynamic_admission_present(fn0)) {
  #include "hako_llvmc_ffi_selected_launch_emit.inc"
  if (!hako_llvmc_emit_selected_dynamic_entry_header(f, &selection, fn0)) {
    note_pure_unsupported_shape(
        -1,
        0,
        "selected_dynamic_entry_signature",
        "selected_dynamic_parameter_signature_mismatch",
        NULL);
    HAKO_LLVMC_CLOSE_GENERIC_OUTPUT();
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }
} else {
  hako_llvmc_emit_entry_header(f, &selection);
}
