/* Physical interval contract only; no source, Home or whole-flow admission. */
#include "../shims/hako_llvmc_ffi.c"
#include <assert.h>

static int interval_with_slot_parent(const char* release, const char* left, const char* right,
    const char* finish, int discharged, int slot_parent) {
  char json[8192];
  int size = snprintf(json, sizeof(json),
      "{\"entry\":0,\"blocks\":["
      "{\"id\":0,\"instructions\":[],\"terminator\":{\"instruction\":{}},\"edges\":[{\"target\":1}]},"
      "{\"id\":1,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[{\"target\":2},{\"target\":3}]},"
      "{\"id\":2,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[{\"target\":4}]},"
      "{\"id\":3,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[{\"target\":4}]},"
      "{\"id\":4,\"instructions\":[],\"terminator\":{\"instruction\":%s},\"edges\":[]}]}",
      release, left, right, finish);
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
  int valid = discharged >= LV4_FIELD_ARRAY_DRAIN ?
      lv4_field_interval(&fn, 4, 0, parent, 7, 3, 11, (hako_lv4_field_interval_kind)discharged) :
      discharged ? lv4_field_dependency_discharged(&fn, 4, 0, parent, 7, 3, 11) :
      lv4_field_borrow_suffix(&fn, 4, 0, parent, 7, 3);
  yyjson_doc_free(doc);
  return valid;
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

#define END "{\"op\":\"invoke\",\"operation\":{\"kind\":\"object_field_release\",\"base\":1,\"object_id\":7,\"field_ordinal\":3,\"child_object_id\":11}}"
#define EMPTY "{}"

int main(void) {
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
  puts("field intervals: strict read and exact per-path Normal/Fault discharge PASS");
  return 0;
}
