/*
 * Caller-zero C materializer for the Rust-owned pinned-Text lifecycle carrier.
 *
 * This is deliberately a fixture boundary, not a production lowering path.
 * It consumes the strict carrier projection together with its existing frame
 * contract and writes inspectable textual LLVM.  It does not infer roots from
 * ordinals, issue a second contract, enter the target-machine session, or
 * provide an alternate route.
 */
#ifndef HAKO_LLVM_C_PTFB_RESIDENCE_CARRIER_INC
#define HAKO_LLVM_C_PTFB_RESIDENCE_CARRIER_INC

#define HAKO_PTF_RESIDENCE_CARRIER_ID "hako.pinned_text_residence_carrier@1"
#define HAKO_PTF_RESIDENCE_CARRIER_SCHEMA 1
#define HAKO_PTF_RESIDENCE_FINISH_OBLIGATION \
    "finish_every_explicit_normal_return"

struct HakoPtfCarrierView {
  long long compilation_brand;
  long long owner_slot;
  long long invocation_ordinal;
  long long plan_stamp;
  long long plan_count;
  long long frame_size;
  long long root_count;
  long long physical_callable_lane_count;
  long long enter_source;
  long long normal_landing;
  long long trap_landing;
  long long normal_exit_count;
  long long finish_sites[128];
  long long slot_lanes[128];
  long long generation_lanes[128];
};

static int hako_llvmc_ptfc_keys_only(
    yyjson_val* object,
    const char* const* keys,
    size_t key_count) {
  yyjson_obj_iter iter;
  yyjson_val* key;
  if (!(object && yyjson_is_obj(object))) return 0;
  iter = yyjson_obj_iter_with(object);
  while ((key = yyjson_obj_iter_next(&iter))) {
    const char* name = yyjson_is_str(key) ? yyjson_get_str(key) : NULL;
    int allowed = 0;
    if (!name) return 0;
    for (size_t i = 0; i < key_count; i++) {
      if (strcmp(name, keys[i]) == 0) {
        allowed = 1;
        break;
      }
    }
    if (!allowed) return 0;
  }
  return 1;
}

static int hako_llvmc_ptfc_nonnegative(
    yyjson_val* object,
    const char* key,
    long long* out) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  if (!(value && yyjson_is_int(value) && yyjson_get_sint(value) >= 0)) return 0;
  if (out) *out = (long long)yyjson_get_sint(value);
  return 1;
}

static int hako_llvmc_ptfc_int_value(yyjson_val* value, long long* out) {
  if (!(value && yyjson_is_int(value) && yyjson_get_sint(value) >= 0)) return 0;
  if (out) *out = (long long)yyjson_get_sint(value);
  return 1;
}

static int hako_llvmc_ptfc_text(
    yyjson_val* object,
    const char* key,
    const char* expected) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  const char* text = value && yyjson_is_str(value) ? yyjson_get_str(value) : NULL;
  return text && expected && strcmp(text, expected) == 0;
}

static int hako_llvmc_ptfc_owner_matches(
    yyjson_val* object,
    long long compilation_brand,
    long long owner_slot) {
  static const char* const keys[] = {"compilation_brand", "slot"};
  long long brand = 0;
  long long slot = 0;
  return hako_llvmc_ptfc_keys_only(object, keys, 2) &&
      hako_llvmc_ptfc_nonnegative(object, "compilation_brand", &brand) &&
      hako_llvmc_ptfc_nonnegative(object, "slot", &slot) &&
      brand == compilation_brand && slot == owner_slot;
}

static int hako_llvmc_ptfc_parse_carrier(
    yyjson_val* frame,
    yyjson_val* carrier,
    struct HakoPtfCarrierView* out,
    char** err_out) {
  static const char* const carrier_keys[] = {
      "contract_id", "schema_revision", "frame_contract", "owner",
      "invocation_ordinal", "target", "residence_abi_revision",
      "plan_stamp", "enter", "finish_obligation", "normal_exit_count",
      "finish_sites", "roots"};
  static const char* const frame_keys[] = {"contract_id", "schema_revision"};
  static const char* const target_keys[] = {"profile_id", "triple", "data_layout"};
  static const char* const enter_keys[] = {"source", "normal", "trap"};
  static const char* const root_keys[] = {
      "frame_row", "logical_ordinal", "source_binding", "slot_lane",
      "generation_lane"};
  static const char* const binding_keys[] = {"owner", "binding_id"};
  yyjson_val* owner;
  yyjson_val* frame_contract;
  yyjson_val* target;
  yyjson_val* enter;
  yyjson_val* finish_sites;
  yyjson_val* roots;
  long long schema = 0;
  long long frame_schema = 0;
  long long frame_plan_stamp = 0;
  long long frame_plan_count = 0;
  long long invocation = 0;
  long long plan_stamp = 0;
  long long normal_exit_count = 0;
  long long root_count = 0;
  long long frame_size = 0;
  long long frame_roots = 0;
  long long physical_lanes = 0;
  long long compilation_brand = 0;
  long long owner_slot = 0;
  long long source = 0;
  long long normal = 0;
  long long trap = 0;
  const char* residence_abi;

  if (!out || !frame || !carrier || !yyjson_is_obj(frame) ||
      !yyjson_is_obj(carrier) ||
      hako_llvmc_ptfb_validate_contract(frame, err_out) != 0) {
    return -1;
  }
  memset(out, 0, sizeof(*out));
  if (!hako_llvmc_ptfc_keys_only(
          carrier, carrier_keys, sizeof(carrier_keys) / sizeof(carrier_keys[0])) ||
      !hako_llvmc_ptfc_text(carrier, "contract_id", HAKO_PTF_RESIDENCE_CARRIER_ID) ||
      !hako_llvmc_ptfc_nonnegative(carrier, "schema_revision", &schema) ||
      schema != HAKO_PTF_RESIDENCE_CARRIER_SCHEMA ||
      !hako_llvmc_ptfc_nonnegative(carrier, "invocation_ordinal", &invocation) ||
      invocation == 0 || !hako_llvmc_ptfc_nonnegative(carrier, "plan_stamp", &plan_stamp) ||
      plan_stamp == 0 || !hako_llvmc_ptfc_nonnegative(
          carrier, "normal_exit_count", &normal_exit_count) || normal_exit_count == 0 ||
      normal_exit_count > 128) {
    return set_err_owned(err_out, "invalid pinned Text residence carrier header");
  }
  frame_contract = yyjson_obj_get(carrier, "frame_contract");
  if (!hako_llvmc_ptfc_keys_only(frame_contract, frame_keys, 2) ||
      !hako_llvmc_ptfc_text(frame_contract, "contract_id", HAKO_PTFB_CONTRACT_ID) ||
      !hako_llvmc_ptfc_nonnegative(frame_contract, "schema_revision", &frame_schema) ||
      frame_schema != HAKO_PTFB_SCHEMA_REVISION) {
    return set_err_owned(err_out, "pinned Text residence carrier frame contract mismatch");
  }

  owner = yyjson_obj_get(frame, "owner");
  if (!hako_llvmc_ptfc_keys_only(owner, (const char* const[]){"compilation_brand", "slot"}, 2) ||
      !hako_llvmc_ptfc_nonnegative(owner, "compilation_brand", &compilation_brand) ||
      compilation_brand == 0 ||
      !hako_llvmc_ptfc_nonnegative(owner, "slot", &owner_slot) ||
      !hako_llvmc_ptfc_owner_matches(yyjson_obj_get(carrier, "owner"), compilation_brand, owner_slot)) {
    return set_err_owned(err_out, "pinned Text residence carrier owner mismatch");
  }
  if (!hako_llvmc_ptfc_nonnegative(frame, "invocation_ordinal", &frame_plan_stamp) ||
      frame_plan_stamp != invocation ||
      !hako_llvmc_ptfc_nonnegative(frame, "plan_stamp", &frame_plan_stamp) ||
      frame_plan_stamp != plan_stamp) {
    return set_err_owned(err_out, "pinned Text residence carrier invocation mismatch");
  }
  if (!hako_llvmc_ptfc_nonnegative(frame, "exact_text_root_count", &frame_roots) ||
      !hako_llvmc_ptfc_nonnegative(frame, "physical_callable_lane_count", &physical_lanes) ||
      !hako_llvmc_ptfc_nonnegative(frame, "plan_count", &frame_plan_count) ||
      frame_roots <= 0 || frame_roots > 128 || physical_lanes <= 0 ||
      physical_lanes > 1024 || frame_plan_count <= 0 || frame_plan_count > 128) {
    return set_err_owned(err_out, "pinned Text residence carrier frame census mismatch");
  }
  {
    yyjson_val* residence = yyjson_obj_get(frame, "residence");
    if (!hako_llvmc_ptfc_nonnegative(residence, "derived_frame_size", &frame_size) ||
        frame_size <= 0 || frame_size > HAKO_PTFB_MAX_FRAME_BYTES) {
      return set_err_owned(err_out, "pinned Text residence carrier frame size mismatch");
    }
  }
  target = yyjson_obj_get(carrier, "target");
  if (!hako_llvmc_ptfc_keys_only(target, target_keys, 3) ||
      !hako_llvmc_ptfc_text(target, "profile_id", HAKO_PTFB_PROFILE_ID) ||
      !hako_llvmc_ptfc_text(target, "triple", HAKO_PTFB_TARGET_TRIPLE) ||
      !hako_llvmc_ptfc_text(target, "data_layout", HAKO_PTFB_DATA_LAYOUT) ||
      !hako_llvmc_ptfc_text(carrier, "residence_abi_revision", HAKO_PTFB_RESIDENCE_ABI)) {
    return set_err_owned(err_out, "pinned Text residence carrier target mismatch");
  }
  residence_abi = yyjson_get_str(yyjson_obj_get(carrier, "residence_abi_revision"));
  (void)residence_abi;

  enter = yyjson_obj_get(carrier, "enter");
  if (!hako_llvmc_ptfc_keys_only(enter, enter_keys, 3) ||
      !hako_llvmc_ptfc_nonnegative(enter, "source", &source) ||
      !hako_llvmc_ptfc_nonnegative(enter, "normal", &normal) ||
      !hako_llvmc_ptfc_nonnegative(enter, "trap", &trap) ||
      source == normal || source == trap || normal == trap ||
      !hako_llvmc_ptfc_text(carrier, "finish_obligation", HAKO_PTF_RESIDENCE_FINISH_OBLIGATION)) {
    return set_err_owned(err_out, "pinned Text residence carrier enter/finish mismatch");
  }
  finish_sites = yyjson_obj_get(carrier, "finish_sites");
  roots = yyjson_obj_get(carrier, "roots");
  if (!(finish_sites && yyjson_is_arr(finish_sites)) ||
      yyjson_arr_size(finish_sites) != (size_t)normal_exit_count ||
      !(roots && yyjson_is_arr(roots)) ||
      yyjson_arr_size(roots) != (size_t)frame_roots) {
    return set_err_owned(err_out, "pinned Text residence carrier site/root census mismatch");
  }
  for (size_t i = 0; i < yyjson_arr_size(finish_sites); i++) {
    long long site = 0;
    if (!hako_llvmc_ptfc_int_value(yyjson_arr_get(finish_sites, i), &site) ||
        site == trap) {
      return set_err_owned(err_out, "pinned Text residence carrier has trap Finish");
    }
    for (size_t j = 0; j < i; j++) {
      long long prior = 0;
      hako_llvmc_ptfc_int_value(yyjson_arr_get(finish_sites, j), &prior);
      if (prior == site) return set_err_owned(err_out, "duplicate pinned Text Finish site");
    }
    out->finish_sites[i] = site;
  }
  for (size_t i = 0; i < yyjson_arr_size(roots); i++) {
    yyjson_val* row = yyjson_arr_get(roots, i);
    yyjson_val* binding;
    long long frame_row = 0;
    long long ordinal = 0;
    long long slot_lane = 0;
    long long generation_lane = 0;
    if (!hako_llvmc_ptfc_keys_only(row, root_keys, 5) ||
        !hako_llvmc_ptfc_nonnegative(row, "frame_row", &frame_row) ||
        frame_row != (long long)i ||
        !hako_llvmc_ptfc_nonnegative(row, "logical_ordinal", &ordinal) ||
        ordinal != (long long)i ||
        !hako_llvmc_ptfc_nonnegative(row, "slot_lane", &slot_lane) ||
        !hako_llvmc_ptfc_nonnegative(row, "generation_lane", &generation_lane) ||
        slot_lane + 1 != generation_lane || slot_lane < 0 ||
        generation_lane >= physical_lanes) {
      return set_err_owned(err_out, "pinned Text residence carrier root mapping mismatch");
    }
    binding = yyjson_obj_get(row, "source_binding");
    if (!hako_llvmc_ptfc_keys_only(binding, binding_keys, 2) ||
        !hako_llvmc_ptfc_owner_matches(yyjson_obj_get(binding, "owner"), compilation_brand, owner_slot) ||
        !hako_llvmc_ptfc_nonnegative(binding, "binding_id", NULL)) {
      return set_err_owned(err_out, "pinned Text residence carrier source binding mismatch");
    }
    out->slot_lanes[i] = slot_lane;
    out->generation_lanes[i] = generation_lane;
    (void)ordinal;
  }
  out->compilation_brand = compilation_brand;
  out->owner_slot = owner_slot;
  out->invocation_ordinal = invocation;
  out->plan_stamp = plan_stamp;
  out->plan_count = frame_plan_count;
  out->frame_size = frame_size;
  out->root_count = frame_roots;
  out->physical_callable_lane_count = physical_lanes;
  out->enter_source = source;
  out->normal_landing = normal;
  out->trap_landing = trap;
  out->normal_exit_count = normal_exit_count;
  return 0;
}

static void hako_llvmc_ptfc_emit_i64_args(
    FILE* out,
    long long lane_count) {
  for (long long i = 0; i < lane_count; i++) {
    if (i) fputs(", ", out);
    fprintf(out, "i64 %%lane%lld", i);
  }
}

static int hako_llvmc_ptfc_emit_fixture(
    const char* ll_out,
    const struct HakoPtfCarrierView* carrier,
    char** err_out) {
  FILE* out;
  if (!ll_out || !carrier) return set_err_owned(err_out, "missing pinned Text carrier fixture output");
  out = fopen(ll_out, "wb");
  if (!out) return set_err_owned(err_out, "cannot open pinned Text carrier fixture output");
  fputs("; caller-zero pinned Text Residence carrier fixture\n", out);
  fputs("declare i32 @hako_text_formal_residence_enter_v1(ptr, i32, ptr, i32)\n", out);
  fputs("declare void @hako_text_formal_residence_finish_or_abort_v1(ptr)\n\n", out);
  fputs("define i64 @hako_pinned_text_residence_carrier_fixture(", out);
  hako_llvmc_ptfc_emit_i64_args(out, carrier->physical_callable_lane_count);
  fputs(") {\nentry:\n", out);
  for (long long i = 0; i < carrier->root_count; i++) {
    fprintf(out, "  %%pair%lld = alloca { i64, i64 }, align 8\n", i);
    fprintf(out, "  store i64 %%lane%lld, ptr %%pair%lld, align 8\n",
            carrier->slot_lanes[i], i);
    fprintf(out, "  %%pair%lld_gen = getelementptr inbounds { i64, i64 }, ptr %%pair%lld, i32 0, i32 1\n",
            i, i);
    fprintf(out, "  store i64 %%lane%lld, ptr %%pair%lld_gen, align 8\n",
            carrier->generation_lanes[i], i);
  }
  fputs("  %frame = alloca i8, i64 ", out);
  fprintf(out, "%lld, align 8\n", carrier->frame_size);
  fprintf(out, "  ; carrier Enter source=%lld normal=%lld trap=%lld\n",
          carrier->enter_source, carrier->normal_landing, carrier->trap_landing);
  fputs("  %status = call i32 @hako_text_formal_residence_enter_v1(ptr %pair0, i32 ", out);
  fprintf(out, "%lld, ptr %%frame, i32 %lld)\n", carrier->root_count, carrier->frame_size);
  fputs("  %ok = icmp eq i32 %status, 0\n  br i1 %ok, label %normal0, label %trap\n\ntrap:\n  unreachable\n\nnormal0:\n", out);
  for (long long i = 0; i < carrier->normal_exit_count; i++) {
    if (i > 0) fprintf(out, "normal%lld:\n", i);
    fprintf(out, "  ; source Finish site block %lld\n", carrier->finish_sites[i]);
    fputs("  call void @hako_text_formal_residence_finish_or_abort_v1(ptr %frame)\n", out);
    fprintf(out, "  ret i64 %lld\n", i);
    if (i + 1 < carrier->normal_exit_count) fputs("\n", out);
  }
  fputs("}\n", out);
  if (fclose(out) != 0) return set_err_owned(err_out, "cannot close pinned Text carrier fixture output");
  return 0;
}

__attribute__((visibility("default")))
int hako_llvmc_emit_pinned_text_residence_carrier_fixture(
    const char* json_in,
    const char* ll_out,
    char** err_out) {
  yyjson_read_err read_error;
  yyjson_doc* document;
  yyjson_val* root;
  yyjson_val* functions;
  yyjson_val* function;
  yyjson_val* metadata;
  yyjson_val* frame;
  yyjson_val* carrier;
  struct HakoPtfCarrierView fixture;
  if (!json_in || !ll_out) return set_err_owned(err_out, "missing pinned Text carrier fixture path");
  document = yyjson_read_file(json_in, 0, NULL, &read_error);
  if (!document) return set_err_owned(err_out, "cannot read pinned Text carrier fixture JSON");
  root = yyjson_doc_get_root(document);
  functions = root ? yyjson_obj_get(root, "functions") : NULL;
  if (!(functions && yyjson_is_arr(functions) && yyjson_arr_size(functions) == 1)) {
    yyjson_doc_free(document);
    return set_err_owned(err_out, "pinned Text carrier fixture requires one function");
  }
  function = yyjson_arr_get(functions, 0);
  metadata = function ? yyjson_obj_get(function, "metadata") : NULL;
  frame = metadata ? yyjson_obj_get(metadata, "pinned_text_backend_frame_v1") : NULL;
  carrier = metadata ? yyjson_obj_get(metadata, "pinned_text_residence_carrier_v1") : NULL;
  if (!frame || !carrier) {
    yyjson_doc_free(document);
    return set_err_owned(err_out, "pinned Text residence carrier metadata is missing");
  }
  if (hako_llvmc_ptfc_parse_carrier(frame, carrier, &fixture, err_out) != 0) {
    yyjson_doc_free(document);
    return -1;
  }
  yyjson_doc_free(document);
  return hako_llvmc_ptfc_emit_fixture(ll_out, &fixture, err_out);
}

#endif
