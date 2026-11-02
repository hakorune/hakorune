/* Physical interval contract only; no source, Home or whole-flow admission. */
#include "../shims/hako_llvmc_ffi.c"
#include <assert.h>

static int interval_with_slot_parent_at(const char* release, const char* left, const char* right,
    const char* finish, int discharged, int slot_parent, size_t position, const char* rows, int backedge) {
  char json[8192];
  int size = snprintf(json, sizeof(json),
      "{\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[{\"target\":1}]},"
      "{\"id\":1,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[{\"target\":2},{\"target\":3}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[{\"target\":4}]},"
      "{\"id\":3,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[{\"target\":4}]},"
      "{\"id\":4,\"instructions\":[%s],\"terminator\":{\"instruction\":%s},\"edges\":[%s]}]}",
      release, left, right, rows, finish, backedge ? "{\"target\":1}" : "");
  assert(size > 0 && (size_t)size < sizeof(json));
  yyjson_doc* doc = yyjson_read(json, (size_t)size, 0);
  assert(doc);
  hako_lv4_function_index fn = {0};
  hako_lv4_block blocks[5];
  hako_lv4_value values[2] = {{0}};
  hako_lv4_value* parent = &values[0];
  fn.function = yyjson_doc_get_root(doc); fn.blocks = blocks; fn.block_count = 5;
  /* Physical identity for same-path comparison; this test issues no lease. */
  parent->id = 1; parent->kind = slot_parent ? LV4_SLOT_BORROW : LV4_HANDLE;
  values[1].id = 2; values[1].kind = LV4_SLOT_BORROW;
  fn.values = values; fn.value_count = 2;
  for (size_t i = 0; i < 5; i++) {
    blocks[i].id = (uint32_t)i; blocks[i].ordinal = i;
    blocks[i].block = yyjson_arr_get(yyjson_obj_get(fn.function, "blocks"), i);
  }
  int valid = position != SIZE_MAX ?
      lv4_field_interval_at(&fn, 4, position, 0, parent, 7, 3, 11, (hako_lv4_field_interval_kind)discharged) :
      discharged >= LV4_FIELD_ARRAY_DRAIN ?
      lv4_field_interval(&fn, 4, 0, parent, 7, 3, 11, (hako_lv4_field_interval_kind)discharged) :
      discharged ? lv4_field_dependency_discharged(&fn, 4, 0, parent, 7, 3, 11) :
      lv4_field_borrow_suffix(&fn, 4, 0, parent, 7, 3);
  yyjson_doc_free(doc);
  return valid;
}

static int interval_with_slot_parent(const char* release, const char* left, const char* right,
    const char* finish, int discharged, int slot_parent) {
  return interval_with_slot_parent_at(release, left, right, finish, discharged, slot_parent, SIZE_MAX, "", 0);
}

static int interval(const char* release, const char* left, const char* right,
    const char* finish, int discharged) {
  return interval_with_slot_parent(release, left, right, finish, discharged, 0);
}

/* This is physical CFG evidence; no source admission or Home is issued. */
static int map_guard(const char* predicate, uint32_t use, int extra_edge, int reentry, int foreign) {
  char json[8192];
  int size = snprintf(json, sizeof(json),
      "{\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[],\"terminator\":{\"instruction\":{\"op\":\"invoke\",\"operation\":{\"kind\":\"map_field_get_indexed\"}}},\"edges\":[{\"target\":1}]},"
      "{\"id\":1,\"instructions\":[{\"instruction\":{\"op\":\"invoke_normal_result\",\"invoke_block\":0,\"dst\":1}},{\"instruction\":{\"op\":\"const_null\",\"dst\":2}},{\"instruction\":{\"op\":\"borrowed_null_compare\",\"dst\":3,\"lhs\":%d,\"rhs\":2,\"predicate\":\"%s\"}}],\"terminator\":{\"instruction\":{\"op\":\"branch\",\"condition\":3,\"then\":2,\"else\":3}},\"edges\":[{\"target\":2},{\"target\":3}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[{\"target\":4}%s]},"
      "{\"id\":3,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[{\"target\":5}%s]},"
      "{\"id\":4,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[]},"
      "{\"id\":5,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[]},"
      "{\"id\":6,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[]}]}",
      foreign ? 4 : 1, predicate, reentry ? ",{\"target\":0}" : "", extra_edge ? ",{\"target\":2}" : "");
  assert(size > 0 && (size_t)size < sizeof(json));
  yyjson_doc* doc = yyjson_read(json, (size_t)size, 0); assert(doc);
  hako_lv4_function_index fn = {0}; hako_lv4_block blocks[7]; hako_lv4_value values[4] = {{0}};
  fn.function = yyjson_doc_get_root(doc); fn.blocks = blocks; fn.block_count = 7;
  fn.values = values; fn.value_count = 4;
  for (size_t i = 0; i < 7; i++) {
    blocks[i].id = (uint32_t)i; blocks[i].ordinal = i;
    blocks[i].block = yyjson_arr_get(yyjson_obj_get(fn.function, "blocks"), i);
  }
  yyjson_val* rows = yyjson_obj_get(blocks[1].block, "instructions");
  for (size_t i = 0; i < 4; i++) values[i].id = (uint32_t)i + 1;
  for (size_t i = 0; i < 3; i++) {
    values[i].instruction = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
    values[i].definition_block = blocks[1].block;
  }
  values[0].kind = LV4_SLOT_BORROW; values[3].kind = LV4_SLOT_BORROW;
  int valid = lv4_map_slot_guarded(&fn, &values[0], use);
  yyjson_doc_free(doc); return valid;
}

/* The indexed source producers establish disjointness; slot/formal ID
 * inequality cannot do so. This component lends no whole-function admission. */
static void read_support_aliases(void) {
  const char* json = "{\"receiver\":null,\"params\":[{\"value\":4},{\"value\":5}],\"blocks\":["
      "{\"id\":0,\"terminator\":{\"instruction\":{\"operation\":{\"kind\":\"new_box\"}}}},"
      "{\"id\":1,\"terminator\":{\"instruction\":{\"operation\":{\"kind\":\"new_box\"}}}}],"
      "\"rows\":[{\"op\":\"invoke_normal_result\",\"invoke_block\":0},{\"op\":\"invoke_normal_result\",\"invoke_block\":1},"
      "{\"op\":\"const_i64\"},{\"op\":\"copy\",\"src\":1}],"
      "\"write\":{\"kind\":\"field_set\",\"base\":2,\"value\":3},"
      "\"escape\":{\"kind\":\"object_field_set\",\"base\":2,\"value\":1},"
      "\"unknown\":{\"kind\":\"field_set\",\"base\":4,\"value\":3}}";
  yyjson_doc* doc = yyjson_read(json, strlen(json), 0); assert(doc);
  yyjson_val* root = yyjson_doc_get_root(doc);
  hako_lv4_function_index fn = {0}; hako_lv4_block blocks[2]; hako_lv4_value values[6] = {{0}};
  fn.function = root; fn.blocks = blocks; fn.block_count = 2; fn.values = values; fn.value_count = 6;
  for (size_t i = 0; i < 2; i++) {
    blocks[i] = (hako_lv4_block){(uint32_t)i, i, yyjson_arr_get(yyjson_obj_get(root, "blocks"), i)};
    values[i].kind = LV4_HANDLE; values[i].origin = (int)i;
    values[i].instruction = yyjson_arr_get(yyjson_obj_get(root, "rows"), i);
  }
  for (size_t i = 0; i < 6; i++) values[i].id = (uint32_t)i + 1;
  values[2].kind = LV4_I64; values[2].instruction = yyjson_arr_get(yyjson_obj_get(root, "rows"), 2);
  values[3].kind = values[4].kind = LV4_TAGGED; values[3].origin = values[4].origin = -1;
  values[5].kind = LV4_HANDLE; values[5].instruction = yyjson_arr_get(yyjson_obj_get(root, "rows"), 3);
  assert(lv4_read_support_disjoint(&fn, 2, &values[0]));
  assert(lv4_read_support_disjoint(&fn, 2, &values[3]));
  assert(lv4_read_support_disjoint(&fn, 4, &values[0]));
  assert(!lv4_read_support_disjoint(&fn, 4, &values[4]));
  assert(!lv4_read_support_disjoint(&fn, 6, &values[0]));
  assert(lv4_read_support_effect(&fn, yyjson_obj_get(root, "write"), &values[0], 1));
  assert(!lv4_read_support_effect(&fn, yyjson_obj_get(root, "escape"), &values[0], 1));
  assert(!lv4_read_support_effect(&fn, yyjson_obj_get(root, "unknown"), &values[4], 1));
  yyjson_doc_free(doc);
}

#define END "{\"op\":\"invoke\",\"operation\":{\"kind\":\"object_field_release\",\"base\":1,\"object_id\":7,\"field_ordinal\":3,\"child_object_id\":11}}"
#define EMPTY "{}"

int main(void) {
  read_support_aliases();
  assert(map_guard("ne", 4, 0, 0, 0));
  assert(map_guard("eq", 5, 0, 0, 0));
  assert(!map_guard("eq", 4, 0, 0, 0));
  assert(!map_guard("slt", 4, 0, 0, 0));
  assert(!map_guard("ne", 6, 0, 0, 0));
  assert(!map_guard("ne", 4, 1, 0, 0));
  assert(!map_guard("ne", 4, 0, 1, 0));
  assert(!map_guard("ne", 4, 0, 0, 1));
  /* One predecessor End discharges both its Normal and Fault landings. */
  assert(interval(END, EMPTY, EMPTY, EMPTY, 1));
  assert(!interval(END, EMPTY, EMPTY, EMPTY, 0));
  assert(interval(EMPTY, EMPTY, EMPTY, EMPTY, 0));
  assert(!interval(EMPTY, EMPTY, EMPTY, EMPTY, 1));
  /* Independent diamonds: both alternatives owe one End. A block-only
   * visited set must not collapse the missing-End alternative. */
  assert(interval(EMPTY, END, END, EMPTY, 1));
  assert(!interval(EMPTY, END, EMPTY, EMPTY, 1));
  assert(!interval(EMPTY, EMPTY, END, EMPTY, 1));
  assert(!interval(END, END, END, EMPTY, 1));
  assert(!interval(END, END, EMPTY, EMPTY, 1));
  /* The current consumer's own terminator cannot pay its prior obligation. */
  assert(!interval(EMPTY, EMPTY, EMPTY, END, 1));
  const char* invalid[] = {
    "{\"operation\":{\"kind\":\"object_field_release\",\"base\":1,\"object_id\":7,\"field_ordinal\":3,\"child_object_id\":12}}",
    "{\"operation\":{\"kind\":\"object_field_release\",\"base\":1,\"object_id\":7,\"field_ordinal\":4,\"child_object_id\":11}}",
    "{\"operation\":{\"kind\":\"object_field_release\",\"base\":2,\"object_id\":7,\"field_ordinal\":3,\"child_object_id\":11}}",
    "{\"operation\":{\"kind\":\"field_residence_release\",\"base\":1,\"object_id\":7,\"field_ordinal\":3}}",
    "{\"operation\":{\"kind\":\"object_field_set\",\"base\":1,\"object_id\":7,\"field_ordinal\":3,\"child_object_id\":11}}",
  };
  for (size_t i = 0; i < sizeof(invalid) / sizeof(*invalid); i++)
    assert(!interval(invalid[i], EMPTY, EMPTY, EMPTY, 1));
  /* A different slot's End is legal, but cannot replace this slot's End. */
  assert(interval(END, invalid[1], invalid[1], EMPTY, 1));
  const char* dynamic = "{\"operation\":{\"kind\":\"field_residence_release\",\"transaction\":\"take_dynamic\",\"base\":1,\"object_id\":7,\"field_ordinal\":3}}";
  const char* write = "{\"operation\":{\"kind\":\"field_set\",\"base\":1,\"object_id\":7,\"field_ordinal\":3}}";
  const char* call = "{\"operation\":{\"kind\":\"ordinary_call\"}}";
  const char* lower = "{\"operation\":{\"kind\":\"field_residence_release\",\"transaction\":\"take_dynamic\",\"base\":1,\"object_id\":7,\"field_ordinal\":2}}";
  /* A slot borrow survives only an unchanged, still-live parent interval. */
  assert(interval(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  assert(!interval(EMPTY, call, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  assert(!interval(EMPTY, write, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  assert(!interval(EMPTY, END, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  assert(!interval(EMPTY, invalid[1], EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  assert(!interval(EMPTY, "{\"operation\":{\"kind\":\"object_end_if_owned\"}}", EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  /* Another get may alias this payload; a same-field write must refuse. */
  const char* other_slot_write = "{\"operation\":{\"kind\":\"field_set\",\"base\":2,\"object_id\":7,\"field_ordinal\":3}}";
  const char* other_slot_end = "{\"operation\":{\"kind\":\"home_release\",\"value\":2}}";
  assert(interval_with_slot_parent(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 1));
  assert(!interval_with_slot_parent(EMPTY, other_slot_write, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 1));
  assert(!interval_with_slot_parent(EMPTY, other_slot_end, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 1));
  assert(!interval(EMPTY, other_slot_write, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT));
  assert(interval(dynamic, EMPTY, EMPTY, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(interval(write, dynamic, dynamic, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(!interval(dynamic, write, EMPTY, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(!interval(dynamic, call, EMPTY, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(!interval(dynamic, dynamic, EMPTY, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(!interval(EMPTY, dynamic, EMPTY, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(!interval(END, EMPTY, EMPTY, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(interval(dynamic, lower, lower, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  assert(!interval(lower, dynamic, dynamic, EMPTY, LV4_FIELD_DYNAMIC_DRAIN));
  /* Same-block writes after a read are outside its lease; earlier writes and
   * writes reached through a later backedge still invalidate it. */
  const char* write_row = "{\"instruction\":{\"op\":\"field_set\",\"base\":1,\"object_id\":7,\"field_ordinal\":3}}";
  assert(interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 0, 0, write_row, 0));
  assert(!interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 0, 1, write_row, 0));
  assert(!interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 0, 2, write_row, 0));
  assert(!interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_BORROW_SLOT, 0, 0, write_row, 1));
  assert(interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT, 0, 0, write_row, 0));
  assert(!interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT, 0, 1, write_row, 0));
  assert(!interval_with_slot_parent_at(EMPTY, EMPTY, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT, 0, 0, write_row, 1));
  assert(!interval(EMPTY, write, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT));
  assert(!interval(EMPTY, END, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT));
  assert(!interval(EMPTY, call, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT));
  assert(!interval_with_slot_parent(EMPTY, other_slot_write, EMPTY, EMPTY, LV4_FIELD_READ_SUPPORT, 1));
  puts("field intervals: strict read and exact per-path Normal/Fault discharge PASS");
  return 0;
}
