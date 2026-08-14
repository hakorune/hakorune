/*
 * Boundary-C physicalization for the selected Dynamic CheckedCallOut lane.
 *
 * This file is a transport consumer only.  The Rust MIR site-id projection
 * owns the call/shape/target facts; this owner validates those facts, emits
 * the direct static entry calls, and preserves the MIR-selected successors.
 * It does not select a provider, inspect coordinates, or own lease state.
 */

#include "../../../include/nyrt_dynamic_call_slot_v2.h"
#include "../../../include/nyrt_dynamic_text_scan_v1.h"
#include "../../../include/nyrt_dynamic_v2_lease_v1.h"

static yyjson_val* hako_llvmc_c1_metadata(yyjson_val* fn) {
  yyjson_val* metadata = fn ? yyjson_obj_get(fn, "metadata") : NULL;
  yyjson_val* value = metadata
      ? yyjson_obj_get(metadata, "dynamic_v2_aot_call_admission_v2")
      : NULL;
  return value && yyjson_is_obj(value) ? value : NULL;
}

static long long hako_llvmc_c1_value_int(yyjson_val* value, int* ok) {
  long long result;
  if (ok) *ok = 0;
  if (!value || !yyjson_is_int(value)) return 0;
  result = (long long)yyjson_get_sint(value);
  if (result < 0) return 0;
  if (ok) *ok = 1;
  return result;
}

static long long hako_llvmc_c1_int(yyjson_val* object, const char* key, int* ok) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  return hako_llvmc_c1_value_int(value, ok);
}

static int hako_llvmc_c1_u64(
    yyjson_val* object,
    const char* key,
    uint64_t* out) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  if (!value || !out || !yyjson_is_uint(value)) return 0;
  *out = yyjson_get_uint(value);
  return 1;
}

static const char* hako_llvmc_c1_text(yyjson_val* object, const char* key) {
  yyjson_val* value = object ? yyjson_obj_get(object, key) : NULL;
  return value && yyjson_is_str(value) ? yyjson_get_str(value) : NULL;
}

static int hako_llvmc_c1_text_eq(yyjson_val* object, const char* key, const char* expected) {
  const char* value = hako_llvmc_c1_text(object, key);
  return value && expected && strcmp(value, expected) == 0;
}

static yyjson_val* hako_llvmc_c1_call_for_site(yyjson_val* metadata, long long site) {
  yyjson_val* calls = metadata ? yyjson_obj_get(metadata, "calls") : NULL;
  if (!(calls && yyjson_is_arr(calls))) {
    yyjson_val* admission = metadata
        ? yyjson_obj_get(metadata, "dynamic_v2_aot_call_admission_v2")
        : NULL;
    calls = admission ? yyjson_obj_get(admission, "calls") : NULL;
  }
  if (!(calls && yyjson_is_arr(calls))) return NULL;
  for (size_t i = 0; i < yyjson_arr_size(calls); i++) {
    yyjson_val* call = yyjson_arr_get(calls, i);
    int ok = 0;
    if (hako_llvmc_c1_int(call, "site_id", &ok) == site && ok) return call;
  }
  return NULL;
}

static int hako_llvmc_c1_array_u32_equal(
    yyjson_val* actual,
    yyjson_val* expected) {
  if (!actual || !expected || !yyjson_is_arr(actual) || !yyjson_is_arr(expected) ||
      yyjson_arr_size(actual) != yyjson_arr_size(expected)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(actual); i++) {
    int actual_ok = 0;
    int expected_ok = 0;
    long long actual_value = hako_llvmc_c1_value_int(yyjson_arr_get(actual, i), &actual_ok);
    long long expected_value = hako_llvmc_c1_value_int(yyjson_arr_get(expected, i), &expected_ok);
    if (!actual_ok || !expected_ok || actual_value != expected_value) return 0;
  }
  return 1;
}

static long long hako_llvmc_c1_block_id(yyjson_val* block, int* ok) {
  return hako_llvmc_c1_int(block, "id", ok);
}

/* Count the canonical JSON terminator edges into one landing.  JSON does not
 * expose mutable predecessor state; the edge census is therefore derived
 * from the already-emitted terminators, while source/target identity still
 * comes from the Rust census metadata. */
static int hako_llvmc_c1_incoming_edge_count(
    yyjson_val* blocks,
    long long target,
    long long expected_source) {
  int count = 0;
  if (!blocks || !yyjson_is_arr(blocks)) return -1;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    int id_ok = 0;
    long long source = hako_llvmc_c1_block_id(block, &id_ok);
    if (!id_ok || !insts || !yyjson_is_arr(insts) || yyjson_arr_size(insts) == 0) continue;
    yyjson_val* term = yyjson_arr_get(insts, yyjson_arr_size(insts) - 1);
    const char* op = hako_llvmc_c1_text(term, "op");
    if (!op) return -1;
    long long edges[2] = {0, 0};
    size_t edge_count = 0;
    int edge_ok = 0;
    if (!strcmp(op, "jump")) {
      edges[0] = hako_llvmc_c1_int(term, "target", &edge_ok);
      if (!edge_ok) return -1;
      edge_count = 1;
    } else if (!strcmp(op, "branch")) {
      edges[0] = hako_llvmc_c1_int(term, "then", &edge_ok);
      if (!edge_ok) return -1;
      edges[1] = hako_llvmc_c1_int(term, "else", &edge_ok);
      if (!edge_ok) return -1;
      edge_count = 2;
    } else if (!strcmp(op, "checked_callout")) {
      edges[0] = hako_llvmc_c1_int(term, "normal", &edge_ok);
      if (!edge_ok) return -1;
      edges[1] = hako_llvmc_c1_int(term, "fault", &edge_ok);
      if (!edge_ok) return -1;
      edge_count = 2;
    }
    for (size_t ei = 0; ei < edge_count; ei++) {
      if (edges[ei] == target) {
        if (source != expected_source) return -1;
        count++;
      }
    }
  }
  return count;
}

static yyjson_val* hako_llvmc_c1_end_facts(yyjson_val* metadata) {
  yyjson_val* facts = metadata ? yyjson_obj_get(metadata, "end_facts") : NULL;
  return facts && yyjson_is_arr(facts) ? facts : NULL;
}

static int hako_llvmc_c1_end_fact_index(
    yyjson_val* facts,
    long long site,
    long long lease_slot,
    long long block,
    long long instruction_index) {
  int found = -1;
  if (!facts || !yyjson_is_arr(facts)) return -1;
  for (size_t i = 0; i < yyjson_arr_size(facts); i++) {
    yyjson_val* fact = yyjson_arr_get(facts, i);
    int ok = 0;
    long long fact_site = hako_llvmc_c1_int(fact, "site_id", &ok);
    long long fact_lease = hako_llvmc_c1_int(fact, "lease_slot", &ok);
    long long fact_block = hako_llvmc_c1_int(fact, "block", &ok);
    long long fact_index = hako_llvmc_c1_int(fact, "instruction_index", &ok);
    if (!ok) return -1;
    if (fact_site == site && fact_lease == lease_slot && fact_block == block &&
        fact_index == instruction_index) {
      if (found >= 0) return -1;
      found = (int)i;
    }
  }
  return found;
}

static int hako_llvmc_c1_validate_call_row(yyjson_val* call, long long site) {
  int ok = 0;
  long long entry;
  long long outcome;
  long long normal_dst;
  long long source_block;
  long long receiver;
  long long normal_landing;
  long long fault_landing;
  long long normal_result_block;
  long long normal_result_index;
  yyjson_val* args;
  const char* symbol;
  const char* shape;
  const char* lease;
  const char* expected_symbol;
  if (!call || !yyjson_is_obj(call)) return 0;
  entry = hako_llvmc_c1_int(call, "entry_id", &ok);
  if (!ok || (site == 0 && entry != HAKO_TEXT_SCAN_ENTRY_SUBSTRING) ||
      (site == 1 && entry != HAKO_TEXT_SCAN_ENTRY_INDEX_OF)) return 0;
  expected_symbol = site == 0 ? HAKO_TEXT_SCAN_SYMBOL_SUBSTRING
                              : HAKO_TEXT_SCAN_SYMBOL_INDEX_OF;
  symbol = hako_llvmc_c1_text(call, "symbol");
  if (!symbol || strcmp(symbol, expected_symbol) != 0) return 0;
  if (!hako_llvmc_c1_int(call, "abi_revision", &ok) || !ok) return 0;
  if (hako_llvmc_c1_int(call, "abi_revision", &ok) != HAKO_TEXT_SCAN_CALL_ABI_REVISION) return 0;
  if (hako_llvmc_c1_int(call, "wire_revision", &ok) != HAKO_TEXT_SCAN_CALL_OUT_WIRE_REVISION) return 0;
  args = yyjson_obj_get(call, "argument_lanes");
  if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == (site == 0 ? 2 : 1))) return 0;
  {
    const char* expected_args0 = site == 0 ? "immediate_i64" : "opaque_handle";
    const char* expected_args1 = site == 0 ? "immediate_i64" : NULL;
    /* argument_lanes[] contains scalar strings, not objects. */
    yyjson_val* arg0 = yyjson_arr_get(args, 0);
    if (!(arg0 && yyjson_is_str(arg0) &&
          strcmp(yyjson_get_str(arg0), expected_args0) == 0)) return 0;
    if (expected_args1) {
      yyjson_val* arg1 = yyjson_arr_get(args, 1);
      if (!(arg1 && yyjson_is_str(arg1) &&
            strcmp(yyjson_get_str(arg1), expected_args1) == 0)) return 0;
    }
  }
  if (site == 0 && !hako_llvmc_c1_text_eq(call, "receiver_lane", "opaque_handle")) return 0;
  if (site == 1 && !hako_llvmc_c1_text_eq(call, "receiver_lane", "opaque_handle")) return 0;
  if (site == 0 && !hako_llvmc_c1_text_eq(call, "result_lane", "opaque_handle")) return 0;
  if (site == 1 && !hako_llvmc_c1_text_eq(call, "result_lane", "immediate_i64")) return 0;
  shape = hako_llvmc_c1_text(call, "normal_shape");
  lease = hako_llvmc_c1_text(call, "lease");
  if (site == 0 && (!shape || strcmp(shape, "end_authorized_handle") != 0 ||
                    !lease || strcmp(lease, "end_authorized") != 0)) return 0;
  if (site == 1 && (!shape || strcmp(shape, "immediate_i64") != 0 ||
                    !lease || strcmp(lease, "none") != 0)) return 0;
  outcome = hako_llvmc_c1_int(call, "outcome_slot", &ok);
  if (!ok || outcome != site) return 0;
  normal_dst = hako_llvmc_c1_int(call, "normal_result_dst", &ok);
  if (!ok || normal_dst <= 0) return 0;
  if (!hako_llvmc_c1_int(call, "effects", &ok) || !ok ||
      hako_llvmc_c1_int(call, "effects", &ok) > 0xffff) return 0;
  source_block = hako_llvmc_c1_int(call, "source_block", &ok);
  if (!ok) return 0;
  receiver = hako_llvmc_c1_int(call, "receiver", &ok);
  if (!ok || receiver < 0) return 0;
  normal_landing = hako_llvmc_c1_int(call, "normal_landing", &ok);
  if (!ok) return 0;
  fault_landing = hako_llvmc_c1_int(call, "fault_landing", &ok);
  if (!ok || normal_landing == fault_landing) return 0;
  normal_result_block = hako_llvmc_c1_int(call, "normal_result_block", &ok);
  if (!ok) return 0;
  normal_result_index = hako_llvmc_c1_int(call, "normal_result_index", &ok);
  if (!ok) return 0;
  (void)source_block;
  (void)normal_result_block;
  (void)normal_result_index;
  return 1;
}

static int hako_llvmc_c1_validate_projection(yyjson_val* root) {
  hako_llvmc_selected_dynamic_identity_view identity;
  int identity_census = hako_llvmc_census_selected_dynamic_identity(root, &identity);
  yyjson_val* fn = NULL;
  yyjson_val* metadata = NULL;
  yyjson_val* formals;
  yyjson_val* params;
  yyjson_val* calls;
  yyjson_val* blocks;
  int ok = 0;
  int metadata_seen[2] = {0, 0};
  int callout_seen[2] = {0, 0};
  int result_seen[2] = {0, 0};
  int fault_seen[2] = {0, 0};
  size_t callout_count = 0;
  size_t result_count = 0;
  size_t fault_count = 0;
  size_t end_count = 0;
  int end_seen[3] = {0, 0, 0};
  yyjson_val* end_facts = NULL;
  long long expected_effects = 0;
  uint64_t registry_generation = 0;
  uint64_t compiler_domain = 0;
  uint64_t invocation_ordinal = 0;
  const char* roles[4] = {"src", "pos", "end", "pred_chars"};
  const char* lanes[4] = {"opaque_handle", "immediate_i64", "immediate_i64", "opaque_handle"};
  if (identity_census < 0) return 0;
  if (identity_census == 0) return 1; /* ordinary functions remain on the old lane */
  fn = identity.helper_fn;
  metadata = hako_llvmc_c1_metadata(fn);
  if (!metadata) return 0;
  if (hako_llvmc_c1_int(metadata, "schema_version", &ok) != 2 || !ok ||
      !hako_llvmc_c1_text_eq(metadata, "contract_id", "hako.text.scan@1") ||
      hako_llvmc_c1_int(metadata, "profile", &ok) != 1 || !ok ||
      hako_llvmc_c1_int(metadata, "abi_revision", &ok) != 1 || !ok ||
      hako_llvmc_c1_int(metadata, "wire_revision", &ok) != 2 || !ok ||
      !hako_llvmc_c1_u64(metadata, "registry_generation", &registry_generation) ||
      registry_generation == 0 ||
      !hako_llvmc_c1_text_eq(metadata, "return_type", "i64") ||
      !hako_llvmc_c1_text_eq(metadata, "return_lane", "immediate_i64")) return 0;
  {
    yyjson_val* stamp = yyjson_obj_get(metadata, "plan_stamp");
    if (!stamp ||
        !hako_llvmc_c1_u64(stamp, "compiler_domain", &compiler_domain) ||
        compiler_domain == 0 ||
        !hako_llvmc_c1_u64(stamp, "invocation_ordinal", &invocation_ordinal) ||
        invocation_ordinal == 0) return 0;
  }
  formals = yyjson_obj_get(metadata, "formal_parameters");
  params = fn ? yyjson_obj_get(fn, "params") : NULL;
  if (!(formals && yyjson_is_arr(formals) && yyjson_arr_size(formals) == 4 &&
        params && yyjson_is_arr(params) && yyjson_arr_size(params) == 4)) return 0;
  for (size_t i = 0; i < 4; i++) {
    yyjson_val* formal = yyjson_arr_get(formals, i);
    long long value_id = hako_llvmc_c1_int(formal, "value_id", &ok);
    if (!formal || !hako_llvmc_c1_text_eq(formal, "role", roles[i]) ||
        !hako_llvmc_c1_text_eq(formal, "lane", lanes[i]) || !ok ||
        value_id != hako_llvmc_c1_value_int(yyjson_arr_get(params, i), &ok) || !ok) return 0;
  }
  calls = yyjson_obj_get(metadata, "calls");
  if (!(calls && yyjson_is_arr(calls) && yyjson_arr_size(calls) == 2)) return 0;
  end_facts = hako_llvmc_c1_end_facts(metadata);
  if (!end_facts || yyjson_arr_size(end_facts) != 3) return 0;
  for (size_t i = 0; i < 2; i++) {
    yyjson_val* call = yyjson_arr_get(calls, i);
    long long site = hako_llvmc_c1_int(call, "site_id", &ok);
    long long effects = hako_llvmc_c1_int(call, "effects", &ok);
    if (!ok || effects < 0 ||
        site < 0 || site > 1 || metadata_seen[site] ||
        !hako_llvmc_c1_validate_call_row(call, site)) return 0;
    expected_effects |= effects;
    metadata_seen[site] = 1;
  }
  {
    long long function_effects = hako_llvmc_c1_int(metadata, "function_effects", &ok);
    if (!ok || function_effects != expected_effects) {
      return 0;
    }
  }
  blocks = fn ? yyjson_obj_get(fn, "blocks") : NULL;
  if (!(blocks && yyjson_is_arr(blocks))) {
    return 0;
  }
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    int block_ok = 0;
    long long block_id = hako_llvmc_c1_block_id(block, &block_ok);
    if (!(insts && yyjson_is_arr(insts))) return 0;
    if (!block_ok) return 0;
    for (size_t ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = hako_llvmc_c1_text(ins, "op");
      long long site = hako_llvmc_c1_int(ins, "site_id", &ok);
      if (!op) return 0;
      if (!strcmp(op, "checked_callout")) {
        yyjson_val* args = yyjson_obj_get(ins, "args");
        long long normal = hako_llvmc_c1_int(ins, "normal", &ok);
        long long fault = hako_llvmc_c1_int(ins, "fault", &ok);
        yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, site);
        long long source = call ? hako_llvmc_c1_int(call, "source_block", &ok) : -1;
        long long expected_receiver = call ? hako_llvmc_c1_int(call, "receiver", &ok) : -1;
        long long actual_receiver = hako_llvmc_c1_int(ins, "receiver", &ok);
        long long expected_effect = call ? hako_llvmc_c1_int(call, "effects", &ok) : -1;
        long long actual_effect = hako_llvmc_c1_int(ins, "effects", &ok);
        if (!ok || site < 0 || site > 1 || callout_count >= 2 || metadata_seen[site] != 1 ||
            callout_seen[site] || block_id != source || ii + 1 != yyjson_arr_size(insts) ||
            actual_receiver != expected_receiver || actual_effect != expected_effect ||
            !args || !yyjson_is_arr(args) || yyjson_arr_size(args) != (site == 0 ? 2 : 1) ||
            !hako_llvmc_c1_array_u32_equal(args, yyjson_obj_get(call, "arguments")) ||
            normal != hako_llvmc_c1_int(call, "normal_landing", &ok) || !ok ||
            fault != hako_llvmc_c1_int(call, "fault_landing", &ok) || !ok ||
            normal == fault) {
          return 0;
        }
        callout_seen[site] = 1;
        callout_count++;
      } else if (!strcmp(op, "checked_callout_normal_result")) {
        long long dst = hako_llvmc_c1_int(ins, "dst", &ok);
        yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, site);
        long long expected_block = call ? hako_llvmc_c1_int(call, "normal_result_block", &ok) : -1;
        long long expected_index = call ? hako_llvmc_c1_int(call, "normal_result_index", &ok) : -1;
        if (!ok || site < 0 || site > 1 || result_seen[site] || !call ||
            block_id != expected_block || (long long)ii != expected_index ||
            dst != hako_llvmc_c1_int(call, "normal_result_dst", &ok) || !ok) {
          return 0;
        }
        result_seen[site] = 1;
        result_count++;
      } else if (!strcmp(op, "checked_callout_fault")) {
        yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, site);
        long long expected_block = call ? hako_llvmc_c1_int(call, "fault_terminal_block", &ok) : -1;
        if (!ok || site < 0 || site > 1 || fault_seen[site] || !call ||
            block_id != expected_block || ii + 1 != yyjson_arr_size(insts)) {
          return 0;
        }
        fault_seen[site] = 1;
        fault_count++;
      } else if (!strcmp(op, "checked_callout_end")) {
        long long lease_slot = hako_llvmc_c1_int(ins, "lease_slot", &ok);
        int end_index = hako_llvmc_c1_end_fact_index(
            end_facts, site, lease_slot, block_id, (long long)ii);
        if (!ok || site != 0 || lease_slot != 0 || end_index < 0 || end_seen[end_index]) {
          return 0;
        }
        end_seen[end_index] = 1;
        end_count++;
      }
    }
  }
  for (int site = 0; site < 2; site++) {
    yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, site);
    int source_ok = 0;
    int normal_ok = 0;
    int fault_ok = 0;
    long long source = hako_llvmc_c1_int(call, "source_block", &source_ok);
    long long normal = hako_llvmc_c1_int(call, "normal_landing", &normal_ok);
    long long fault = hako_llvmc_c1_int(call, "fault_landing", &fault_ok);
    if (!call || !source_ok || !normal_ok || !fault_ok ||
        hako_llvmc_c1_incoming_edge_count(blocks, normal, source) != 1 ||
        hako_llvmc_c1_incoming_edge_count(blocks, fault, source) != 1) {
      return 0;
    }
  }
  return callout_count == 2 && result_count == 2 && fault_count == 2 && end_count == 3 &&
         metadata_seen[0] && metadata_seen[1] && callout_seen[0] && callout_seen[1] &&
         result_seen[0] && result_seen[1] && fault_seen[0] && fault_seen[1] &&
         end_seen[0] && end_seen[1] && end_seen[2];
}

static void hako_llvmc_c1_emit_declarations(FILE* f, yyjson_val* metadata) {
  if (!f || !metadata) return;
  fprintf(f, "declare i32 @\"%s\"(i64, i64, i64, ptr)\n", HAKO_TEXT_SCAN_SYMBOL_SUBSTRING);
  fprintf(f, "declare i32 @\"%s\"(i64, i64, ptr)\n", HAKO_TEXT_SCAN_SYMBOL_INDEX_OF);
  fprintf(f, "declare i32 @\"nyrt_dynamic_v2_lease_consume_end_authorized_v1\"(i64)\n");
  fprintf(f, "declare void @llvm.trap()\n");
}

static int hako_llvmc_c1_emit_callout(
    FILE* f, yyjson_val* metadata, yyjson_val* ins, long long block_id, size_t instruction_index) {
  int ok = 0;
  long long site = hako_llvmc_c1_int(ins, "site_id", &ok);
  long long receiver = hako_llvmc_c1_int(ins, "receiver", &ok);
  long long normal = hako_llvmc_c1_int(ins, "normal", &ok);
  long long fault = hako_llvmc_c1_int(ins, "fault", &ok);
  yyjson_val* args = ins ? yyjson_obj_get(ins, "args") : NULL;
  yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, site);
  long long entry;
  const char* symbol;
  size_t argc;
  if (!f || !call || !ok || site < 0 || site > 1 || normal == fault ||
      !(args && yyjson_is_arr(args)) || !hako_llvmc_c1_validate_call_row(call, site)) return -1;
  entry = hako_llvmc_c1_int(call, "entry_id", &ok);
  symbol = entry == HAKO_TEXT_SCAN_ENTRY_SUBSTRING ? HAKO_TEXT_SCAN_SYMBOL_SUBSTRING
                                                    : HAKO_TEXT_SCAN_SYMBOL_INDEX_OF;
  argc = yyjson_arr_size(args);
  fprintf(f, "  %%c1_out_%lld = alloca [48 x i8], align 8\n", site);
  fprintf(f, "  %%c1_out_%lld_status_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 0\n", site, site);
  fprintf(f, "  %%c1_out_%lld_fault_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 4\n", site, site);
  fprintf(f, "  %%c1_out_%lld_tag_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 8\n", site, site);
  fprintf(f, "  %%c1_out_%lld_disp_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 12\n", site, site);
  fprintf(f, "  %%c1_out_%lld_forwarded_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 16\n", site, site);
  fprintf(f, "  %%c1_out_%lld_reserved_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 20\n", site, site);
  fprintf(f, "  %%c1_out_%lld_value_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 24\n", site, site);
  fprintf(f, "  %%c1_out_%lld_lease_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 32\n", site, site);
  fprintf(f, "  %%c1_out_%lld_continuation_ptr = getelementptr inbounds [48 x i8], ptr %%c1_out_%lld, i64 0, i64 40\n", site, site);
  fprintf(f, "  %%c1_transport_%lld_%lld_%zu = call i32 @\"%s\"(i64 %%r%lld", site, block_id, instruction_index, symbol, receiver);
  for (size_t i = 0; i < argc; i++) {
    int arg_ok = 0;
    long long arg = hako_llvmc_c1_value_int(yyjson_arr_get(args, i), &arg_ok);
    if (!arg_ok) return -1;
    fprintf(f, ", i64 %%r%lld", arg);
  }
  fprintf(f, ", ptr %%c1_out_%lld)\n", site);
  fprintf(f, "  %%c1_transport_ok_%lld_%lld_%zu = icmp eq i32 %%c1_transport_%lld_%lld_%zu, 0\n",
          site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  br i1 %%c1_transport_ok_%lld_%lld_%zu, label %%c1_decode_%lld_%lld_%zu, label %%c1_trap_transport_%lld_%lld_%zu\n",
          site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "c1_trap_transport_%lld_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n",
          site, block_id, instruction_index);
  fprintf(f, "c1_decode_%lld_%lld_%zu:\n", site, block_id, instruction_index);
  fprintf(f, "  %%c1_status_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_status_ptr\n",
          site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_normal_%lld_%lld_%zu = icmp eq i32 %%c1_status_%lld_%lld_%zu, %u\n",
          site, block_id, instruction_index, site, block_id, instruction_index, HAKO_DYNAMIC_V2_STATUS_NORMAL);
  fprintf(f, "  %%c1_fault_%lld_%lld_%zu = icmp eq i32 %%c1_status_%lld_%lld_%zu, %u\n",
          site, block_id, instruction_index, site, block_id, instruction_index, HAKO_DYNAMIC_V2_STATUS_FAULT);
  fprintf(f, "  br i1 %%c1_normal_%lld_%lld_%zu, label %%c1_validate_normal_%lld_%lld_%zu, label %%c1_check_fault_%lld_%lld_%zu\n",
          site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "c1_check_fault_%lld_%lld_%zu:\n  br i1 %%c1_fault_%lld_%lld_%zu, label %%c1_validate_fault_%lld_%lld_%zu, label %%c1_trap_status_%lld_%lld_%zu\n",
          site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "c1_trap_status_%lld_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n",
          site, block_id, instruction_index);
  fprintf(f, "c1_validate_normal_%lld_%lld_%zu:\n", site, block_id, instruction_index);
  fprintf(f, "  %%c1_tag_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_tag_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_disp_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_disp_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_code_norm_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_fault_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_forwarded_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_forwarded_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_reserved_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_reserved_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_lease_%lld_%lld_%zu = load i64, ptr %%c1_out_%lld_lease_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_continuation_%lld_%lld_%zu = load i64, ptr %%c1_out_%lld_continuation_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_tag_ok_%lld_%lld_%zu = icmp eq i32 %%c1_tag_%lld_%lld_%zu, %u\n", site, block_id, instruction_index, site, block_id, instruction_index, site == 0 ? HAKO_DYNAMIC_V2_TAG_HOST_HANDLE : HAKO_DYNAMIC_V2_TAG_IMMEDIATE_I64);
  fprintf(f, "  %%c1_disp_ok_%lld_%lld_%zu = icmp eq i32 %%c1_disp_%lld_%lld_%zu, %u\n", site, block_id, instruction_index, site, block_id, instruction_index, site == 0 ? HAKO_DYNAMIC_V2_DISPOSITION_END_AUTHORIZED : HAKO_DYNAMIC_V2_DISPOSITION_NONE);
  fprintf(f, "  %%c1_lease_ok_%lld_%lld_%zu = icmp %s i64 %%c1_lease_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site == 0 ? "ne" : "eq", site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_none_%lld_%lld_%zu = icmp eq i32 %%c1_fault_code_norm_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_forwarded_none_%lld_%lld_%zu = icmp eq i32 %%c1_forwarded_%lld_%lld_%zu, %u\n", site, block_id, instruction_index, site, block_id, instruction_index, HAKO_DYNAMIC_V2_FORWARDED_NONE_V1);
  fprintf(f, "  %%c1_reserved_zero_%lld_%lld_%zu = icmp eq i32 %%c1_reserved_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_continuation_zero_%lld_%lld_%zu = icmp eq i64 %%c1_continuation_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_norm_ok_%lld_%lld_%zu = and i1 %%c1_tag_ok_%lld_%lld_%zu, %%c1_disp_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_norm_ok2_%lld_%lld_%zu = and i1 %%c1_norm_ok_%lld_%lld_%zu, %%c1_lease_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_norm_ok3_%lld_%lld_%zu = and i1 %%c1_norm_ok2_%lld_%lld_%zu, %%c1_fault_none_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_norm_ok4_%lld_%lld_%zu = and i1 %%c1_norm_ok3_%lld_%lld_%zu, %%c1_forwarded_none_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_norm_ok5_%lld_%lld_%zu = and i1 %%c1_norm_ok4_%lld_%lld_%zu, %%c1_reserved_zero_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_norm_ok6_%lld_%lld_%zu = and i1 %%c1_norm_ok5_%lld_%lld_%zu, %%c1_continuation_zero_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  br i1 %%c1_norm_ok6_%lld_%lld_%zu, label %%bb%lld, label %%c1_trap_normal_%lld_%lld_%zu\n", site, block_id, instruction_index, normal, site, block_id, instruction_index);
  fprintf(f, "c1_trap_normal_%lld_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n", site, block_id, instruction_index);
  fprintf(f, "c1_validate_fault_%lld_%lld_%zu:\n", site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_code_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_fault_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_code_ok_%lld_%lld_%zu = icmp ne i32 %%c1_fault_code_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_tag_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_tag_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_disp_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_disp_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_forwarded_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_forwarded_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_reserved_%lld_%lld_%zu = load i32, ptr %%c1_out_%lld_reserved_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_value_%lld_%lld_%zu = load i64, ptr %%c1_out_%lld_value_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_lease_%lld_%lld_%zu = load i64, ptr %%c1_out_%lld_lease_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_continuation_%lld_%lld_%zu = load i64, ptr %%c1_out_%lld_continuation_ptr\n", site, block_id, instruction_index, site);
  fprintf(f, "  %%c1_fault_tag_ok_%lld_%lld_%zu = icmp eq i32 %%c1_fault_tag_%lld_%lld_%zu, %u\n", site, block_id, instruction_index, site, block_id, instruction_index, HAKO_DYNAMIC_V2_TAG_INVALID);
  fprintf(f, "  %%c1_fault_disp_ok_%lld_%lld_%zu = icmp eq i32 %%c1_fault_disp_%lld_%lld_%zu, %u\n", site, block_id, instruction_index, site, block_id, instruction_index, HAKO_DYNAMIC_V2_DISPOSITION_NONE);
  fprintf(f, "  %%c1_fault_forwarded_ok_%lld_%lld_%zu = icmp eq i32 %%c1_fault_forwarded_%lld_%lld_%zu, %u\n", site, block_id, instruction_index, site, block_id, instruction_index, HAKO_DYNAMIC_V2_FORWARDED_NONE_V1);
  fprintf(f, "  %%c1_fault_reserved_ok_%lld_%lld_%zu = icmp eq i32 %%c1_fault_reserved_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_value_ok_%lld_%lld_%zu = icmp eq i64 %%c1_fault_value_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_lease_ok_%lld_%lld_%zu = icmp eq i64 %%c1_fault_lease_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_continuation_ok_%lld_%lld_%zu = icmp eq i64 %%c1_fault_continuation_%lld_%lld_%zu, 0\n", site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_shape_%lld_%lld_%zu = and i1 %%c1_fault_tag_ok_%lld_%lld_%zu, %%c1_fault_disp_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_shape2_%lld_%lld_%zu = and i1 %%c1_fault_shape_%lld_%lld_%zu, %%c1_fault_forwarded_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_shape3_%lld_%lld_%zu = and i1 %%c1_fault_shape2_%lld_%lld_%zu, %%c1_fault_reserved_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_shape4_%lld_%lld_%zu = and i1 %%c1_fault_shape3_%lld_%lld_%zu, %%c1_fault_value_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_shape5_%lld_%lld_%zu = and i1 %%c1_fault_shape4_%lld_%lld_%zu, %%c1_fault_lease_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_shape6_%lld_%lld_%zu = and i1 %%c1_fault_shape5_%lld_%lld_%zu, %%c1_fault_continuation_ok_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  %%c1_fault_ready_%lld_%lld_%zu = and i1 %%c1_fault_code_ok_%lld_%lld_%zu, %%c1_fault_shape6_%lld_%lld_%zu\n", site, block_id, instruction_index, site, block_id, instruction_index, site, block_id, instruction_index);
  fprintf(f, "  br i1 %%c1_fault_ready_%lld_%lld_%zu, label %%bb%lld, label %%c1_trap_fault_%lld_%lld_%zu\n", site, block_id, instruction_index, fault, site, block_id, instruction_index);
  fprintf(f, "c1_trap_fault_%lld_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n", site, block_id, instruction_index);
  return 0;
}

static int hako_llvmc_c1_emit_normal_result(FILE* f, yyjson_val* metadata, yyjson_val* ins) {
  int ok = 0;
  long long site = hako_llvmc_c1_int(ins, "site_id", &ok);
  long long dst = hako_llvmc_c1_int(ins, "dst", &ok);
  yyjson_val* call = hako_llvmc_c1_call_for_site(metadata, site);
  if (!f || !ok || !call || site < 0 || site > 1 || !hako_llvmc_c1_validate_call_row(call, site)) return -1;
  fprintf(f, "  %%r%lld = load i64, ptr %%c1_out_%lld_value_ptr\n", dst, site);
  return site == 0 ? 1 : 2;
}

static int hako_llvmc_c1_emit_end(FILE* f, yyjson_val* ins, long long block_id, size_t instruction_index) {
  int ok = 0;
  long long site = hako_llvmc_c1_int(ins, "site_id", &ok);
  long long lease_slot = hako_llvmc_c1_int(ins, "lease_slot", &ok);
  if (!f || !ok || site != 0 || lease_slot != 0) return -1;
  fprintf(f, "  %%c1_end_token_%lld_%zu = load i64, ptr %%c1_out_0_lease_ptr\n", block_id, instruction_index);
  fprintf(f, "  %%c1_end_status_%lld_%zu = call i32 @\"nyrt_dynamic_v2_lease_consume_end_authorized_v1\"(i64 %%c1_end_token_%lld_%zu)\n", block_id, instruction_index, block_id, instruction_index);
  fprintf(f, "  %%c1_end_ok_%lld_%zu = icmp eq i32 %%c1_end_status_%lld_%zu, 0\n", block_id, instruction_index, block_id, instruction_index);
  fprintf(f, "  br i1 %%c1_end_ok_%lld_%zu, label %%c1_end_continue_%lld_%zu, label %%c1_trap_end_%lld_%zu\n", block_id, instruction_index, block_id, instruction_index, block_id, instruction_index);
  fprintf(f, "c1_trap_end_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n", block_id, instruction_index);
  fprintf(f, "c1_end_continue_%lld_%zu:\n", block_id, instruction_index);
  return 0;
}
