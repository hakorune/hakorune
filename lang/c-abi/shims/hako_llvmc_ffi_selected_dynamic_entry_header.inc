/* C1-A: selected Dynamic uses the exact MIR formal signature. */
if (hako_llvmc_selected_dynamic_admission_present(fn0)) {
  #include "hako_llvmc_ffi_selected_launch_emit.inc"
  if (!hako_llvmc_emit_selected_dynamic_entry_header(f, &selection, fn0)) {
    note_pure_unsupported_shape(
        -1,
        0,
        "selected_dynamic_entry_signature",
        "selected_dynamic_parameter_signature_mismatch",
        NULL);
    fclose(f);
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }
} else {
  hako_llvmc_emit_entry_header(f, &selection);
}
