/* Shared retained-input execution for the public static V2 host.
 * No pathname reparse or V1 retry. */
static int hako_llvmc_compile_static_v2_retained(
    struct HakoLlvmcInvocation* invocation,
    const hako_llvmc_published_static_frame_v2* frame,
    const char* obj_out, char** error) {
  if (!invocation || !invocation->document || !obj_out || !*obj_out)
    return set_err_owned(error, "[freeze:contract][static-v2/input]");
  if (hako_llvmc_reject_ambient_harness_replay(error) != 0) return -1;
  if (hako_static_v2_bind(&invocation->static_v2, invocation->document, frame, error) != 0) return -1;
  int rc = -1;
  char* staging = NULL;
  if (hako_llvmc_published_call_rows_begin_v2(frame->calls, (size_t)frame->call_count, error) != 0)
    goto finish_frame;
  size_t length = strlen(obj_out);
  if (length > SIZE_MAX - 24) {
    set_err_owned(error, "[freeze:contract][static-v2/output-path]");
    goto finish_calls;
  }
  staging = malloc(length + 24);
  if (!staging) {
    set_err_owned(error, "[freeze:contract][static-v2/output-storage]");
    goto finish_calls;
  }
  snprintf(staging, length + 24, "%s.v2-XXXXXX", obj_out);
  int fd = mkstemp(staging);
  if (fd < 0) {
    set_err_owned(error, "[freeze:contract][static-v2/staging-open]");
    goto finish_calls;
  }
  if (close(fd) != 0 || remove(staging) != 0) {
    set_err_owned(error, "[freeze:contract][static-v2/staging-close]");
    goto finish_calls;
  }
  /* Early optimizer returns may bypass the generic pre-object residual check. */
  rc = compile_doc_compat_pure(invocation, NULL, staging, error);
  if (rc == 0) rc = hako_llvmc_published_static_method_rows_finish(error);
  if (rc == 0) rc = hako_static_v2_finish(&invocation->static_v2, error);
  if (rc == 0 && rename(staging, obj_out) != 0)
    rc = set_err_owned(error, "[freeze:contract][static-v2/publish]");
finish_calls:
  if (staging) { remove(staging); free(staging); }
  hako_llvmc_published_static_method_rows_end();
finish_frame:
  hako_static_v2_clear(&invocation->static_v2);
  return rc;
}

static int hako_llvmc_static_open_v2_with_contract(
    const char* bytes, size_t length,
    const hako_llvmc_physical_contract_v1* contract,
    hako_llvmc_static_invocation_v2** out, char** error) {
  struct HakoLlvmcPhysicalOptions options = {0};
  int explicit_contract = contract != NULL;
  if (!out) return set_err_owned(error, "[freeze:contract][static-v2/open-output]");
  *out = NULL;
  if (!bytes || !length) return set_err_owned(error, "[freeze:contract][static-v2/open-input]");
  if (explicit_contract) {
    if (contract->ingress_profile != HAKO_LLVMC_PHYSICAL_PROFILE_STATIC_V2)
      return set_err_owned(error, "[freeze:contract][static-v2/profile]");
    if (hako_llvmc_physical_options_copy(&options, contract, error) != 0)
      return -1;
  }
  if (hako_llvmc_reject_ambient_harness_replay(error) != 0) {
    hako_llvmc_physical_options_destroy(&options);
    return -1;
  }
  yyjson_read_err parse_error;
  yyjson_doc* document = yyjson_read_opts((char*)bytes, length, 0, NULL, &parse_error);
  if (!document) {
    hako_llvmc_physical_options_destroy(&options);
    return set_err_owned(error, "[freeze:contract][static-v2/json]");
  }
  struct HakoLlvmcInvocation* invocation = malloc(sizeof(*invocation));
  if (!invocation) {
    yyjson_doc_free(document);
    hako_llvmc_physical_options_destroy(&options);
    return set_err_owned(error, "[freeze:contract][static-v2/open-storage]");
  }
  hako_llvmc_invocation_init(
      invocation,
      document,
      hako_llvmc_capture_allocation_config(),
      HAKO_LLVMC_INGRESS_STATIC_V2);
  invocation->options = options;
  options = (struct HakoLlvmcPhysicalOptions){0};
  *out = invocation;
  return 0;
}

__attribute__((visibility("default")))
int hako_llvmc_static_open_v2(const char* bytes, size_t length,
    hako_llvmc_static_invocation_v2** out, char** error) {
  return hako_llvmc_static_open_v2_with_contract(bytes, length, NULL, out, error);
}

__attribute__((visibility("default")))
int hako_llvmc_static_open_v2_with_options(
    const char* bytes, size_t length,
    const hako_llvmc_physical_contract_v1* contract,
    hako_llvmc_static_invocation_v2** out, char** error) {
  if (!out)
    return set_err_owned(error, "[freeze:contract][static-v2/open-output]");
  *out = NULL;
  if (!contract)
    return set_err_owned(error, "[freeze:contract][static-v2/options-null]");
  return hako_llvmc_static_open_v2_with_contract(bytes, length, contract, out, error);
}

__attribute__((visibility("default")))
int hako_llvmc_static_query_v2(hako_llvmc_static_invocation_v2* invocation,
    const char* function, size_t length, uint32_t block, uint32_t instruction,
    uint32_t* consumer) {
  if (!invocation || !consumer) return NAMED_QUERY_UNADDRESSABLE;
  *consumer = UINT32_MAX;
  const struct NamedAllocationOutcome* outcome = NULL;
  enum HakoLlvmcNamedQueryStatus status = hako_llvmc_invocation_query_named(
      invocation, function, length, block, instruction, &outcome);
  if (status == NAMED_QUERY_BOUND) *consumer = (uint32_t)outcome->consumer;
  return status;
}

__attribute__((visibility("default")))
int hako_llvmc_static_compile_v2(hako_llvmc_static_invocation_v2* invocation,
    const hako_llvmc_published_static_frame_v2* frame, const char* output, char** error) {
  return hako_llvmc_compile_static_v2_retained(invocation, frame, output, error);
}

__attribute__((visibility("default")))
void hako_llvmc_static_close_v2(hako_llvmc_static_invocation_v2* invocation) {
  if (!invocation) return;
  hako_llvmc_invocation_destroy(invocation);
  free(invocation);
}
