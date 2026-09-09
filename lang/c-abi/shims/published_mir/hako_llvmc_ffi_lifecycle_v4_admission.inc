/* Physical cohort admission only. Source names never participate. */
typedef struct {
  yyjson_val *root, *functions;
  uint32_t profile;
  int native;
  uint64_t result_site;
  struct hako_lv4_function_index* index;
  size_t index_count;
  struct hako_lv4_layout_index* layouts;
  size_t layout_count;
  hako_llvmc_lifecycle_measurement* measurement;
} hako_lv4_input;

enum { LV4_BAD, LV4_I64, LV4_HANDLE, LV4_FRAME, LV4_UNIT, LV4_TEXT, LV4_BOOL, LV4_F64, LV4_TAGGED, LV4_MAP, LV4_KEY, LV4_OUTCOME };

static uint32_t lv4_u(yyjson_val* object, const char* key) {
  return (uint32_t)yyjson_get_uint(yyjson_obj_get(object, key));
}
static int lv4_is(yyjson_val* object, const char* key, const char* text) {
  return hako_physical_string(yyjson_obj_get(object, key), text);
}
static yyjson_val* lv4_term(yyjson_val* block) {
  return yyjson_obj_get(yyjson_obj_get(block, "terminator"), "instruction");
}
static yyjson_val* lv4_def(yyjson_val* fn, uint32_t value) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
      yyjson_val* dst = hako_physical_instruction_dst(ins);
      if (dst && yyjson_get_uint(dst) == value) return ins;
    }
  }
  return NULL;
}
static int lv4_type(yyjson_val* fn, uint32_t value, size_t budget) {
  yyjson_val *params = yyjson_obj_get(fn, "params"), *def;
  if (!budget) return LV4_BAD;
  if (yyjson_is_uint(yyjson_obj_get(fn, "receiver")) && lv4_u(fn, "receiver") == value)
    return LV4_HANDLE;
  for (size_t i = 0; i < yyjson_arr_size(params); i++)
    if (lv4_u(yyjson_arr_get(params, i), "value") == value)
      return LV4_TAGGED;
  def = lv4_def(fn, value);
  if (!def) return LV4_BAD;
  if (lv4_is(def, "op", "const_i64") || lv4_is(def, "op", "add") ||
      lv4_is(def, "op", "object_field_get")) return LV4_I64;
  if (lv4_is(def, "op", "const_bool")) return LV4_BOOL;
  if (lv4_is(def, "op", "const_f64_bits")) return LV4_F64;
  if (lv4_is(def, "op", "invoke_normal_result")) return LV4_HANDLE;
  if (lv4_is(def, "op", "fault_frame_enter")) return LV4_FRAME;
  if (lv4_is(def, "op", "const_unit")) return LV4_UNIT;
  if (lv4_is(def, "op", "const_string")) return LV4_TEXT;
  if (lv4_is(def, "op", "copy")) return lv4_type(fn, lv4_u(def, "src"), budget - 1);
  return LV4_BAD;
}
static size_t lv4_value_budget(yyjson_val* fn) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  size_t count = yyjson_arr_size(yyjson_obj_get(fn, "params")) + 1;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++)
    count += yyjson_arr_size(yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions"));
  return count;
}

#include "hako_llvmc_ffi_lifecycle_v4_index.inc"
#include "hako_llvmc_ffi_lifecycle_v4_native_admission.inc"

#include "hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc"

static int lv4_admit(yyjson_val* root, hako_lv4_input* out, char** error) {
  out->root = root; out->functions = yyjson_obj_get(root, "functions");
  out->native = yyjson_obj_get(root, "runtime_requirements") != NULL;
  out->result_site = yyjson_get_uint(yyjson_obj_get(root, "process_result_site"));
  out->profile = lv4_u(root, "storage_profile");
  if (out->native ? lv4_native_admit(out) : lv4_indexed_admit(out)) return 0;
  return set_err_owned(error, "[freeze:contract][published-lifecycle-v4/unsupported-cohort]");
}
