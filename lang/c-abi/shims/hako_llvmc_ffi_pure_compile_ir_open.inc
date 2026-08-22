// Open the unpublished IR file and emit artifact-bound module facts before
// function declarations.  Included inside the generic pure compile scope.
if (ptfc_selected) {
  if (hako_llvmc_ptfc_open_selected_draft(&ptfc_draft, err_out) != 0) {
    yyjson_doc_free(d);
    goto GEN_ABORT;
  }
  f = ptfc_draft.stream;
} else {
  snprintf(llpath, sizeof(llpath), "%s/hako_pure_gen_%d.ll", "/tmp", (int)getpid());
  f = fopen(llpath, "wb");
}
if (!f) {
  note_pure_unsupported_shape(
      -1,
      0,
      "generic_pure_ir",
      "ir_temp_file_open_failed",
      NULL);
  yyjson_doc_free(d);
  break;
}
#define HAKO_LLVMC_CLOSE_GENERIC_OUTPUT() do { \
  if (ptfc_selected) hako_llvmc_ptfc_discard_selected_draft(&ptfc_draft); \
  else if (f) fclose(f); \
  f = NULL; \
} while (0)
fprintf(f, "; nyash pure IR (generic)\n");
if (ptfc_selected) {
  fprintf(f, "target triple = \"%s\"\n", HAKO_PTFB_TARGET_TRIPLE);
  fprintf(f, "target datalayout = \"%s\"\n\n", HAKO_PTFB_DATA_LAYOUT);
} else if (ptfb_contract_bound) {
  fprintf(f, "target triple = \"%s\"\n", hako_llvmc_ptfb_session_triple(&ptfb_session));
  fprintf(f, "target datalayout = \"%s\"\n\n", hako_llvmc_ptfb_session_data_layout(&ptfb_session));
} else {
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
}
if (!ptfc_selected && !hako_llvmc_c1_emit_artifact_descriptor(f, fn_metadata)) {
  note_pure_unsupported_shape(
      -1,
      0,
      "selected_dynamic_artifact_descriptor",
      "artifact_descriptor_projection_failed",
      NULL);
  HAKO_LLVMC_CLOSE_GENERIC_OUTPUT();
  yyjson_doc_free(d);
  goto GEN_ABORT;
}
