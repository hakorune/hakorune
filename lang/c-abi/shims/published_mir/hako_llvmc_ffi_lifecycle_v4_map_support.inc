/* Borrowed array support travels in the existing worklist snapshot. An install
 * row owns the identity of the parent, key, and ordered child operands. These
 * slots record possibilities at a join, independently of the Map leases. */
enum { LV4_MAP_SUPPORT_LIVE = 1, LV4_MAP_SUPPORT_EXPIRED = 2 };

static size_t lv4_map_support_stride(hako_lv4_function_index* fn) {
  size_t stride = lv4_state_count(fn);
  for (size_t i = 0; i < fn->block_count; i++) {
    hako_lv4_block* block = &fn->blocks[i];
    yyjson_val* op = yyjson_obj_get(lv4_term(block->block), "operation");
    if (lv4_is(op, "kind", "map_array_index_map")) fn->has_map_views = 1;
    if (!lv4_is(op, "kind", "map_install_borrowed_array")) continue;
    size_t len = yyjson_arr_size(yyjson_obj_get(op, "elements"));
    if (len > SIZE_MAX - stride) return 0;
    if (len) block->map_support = stride;
    stride += len;
  }
  fn->has_map_support = stride != lv4_state_count(fn);
  return stride;
}

static yyjson_val* lv4_map_operation(hako_lv4_function_index* fn, size_t origin) {
  yyjson_val* block = yyjson_arr_get(hako_physical_blocks(fn->function), origin);
  return yyjson_obj_get(lv4_term(block), "operation");
}

static yyjson_val* lv4_map_install_key(hako_lv4_function_index* fn, yyjson_val* op) {
  hako_lv4_value* key = lv4_index_value(fn, lv4_u(op, "key"));
  if (!key || key->origin < 0 || (size_t)key->origin >= fn->block_count) return NULL;
  yyjson_val* producer = lv4_map_operation(fn, (size_t)key->origin);
  return lv4_is(producer, "kind", "map_prepare_key") ? yyjson_obj_get(producer, "utf8") : NULL;
}

static int lv4_map_key_equal(yyjson_val* a, yyjson_val* b) {
  const char* x = yyjson_get_str(a);
  const char* y = yyjson_get_str(b);
  size_t len = yyjson_get_len(a);
  return x && y && len == yyjson_get_len(b) && !memcmp(x, y, len);
}

/* Child IDs refer to exact Map producers or distinct borrowed formals. Duplicate
 * elements carry the same support and expire together. They never add a lease.
 * A zero slot is absent. Live | Expired still requires rejection at a read. */
static int lv4_map_support_allowed(hako_lv4_function_index* fn,
    const hako_lv4_state* state, uint32_t parent, yyjson_val* key,
    yyjson_val* index, int escapes) {
  if (!fn->has_map_support) return 1;
  for (size_t i = 0; i < fn->block_count; i++) {
    size_t slot = fn->blocks[i].map_support;
    if (!slot || !state[slot]) continue;
    yyjson_val* install = lv4_map_operation(fn, fn->blocks[i].ordinal);
    if (lv4_u(install, "map") != parent ||
        (key && !lv4_map_key_equal(key, lv4_map_install_key(fn, install)))) continue;
    if (escapes) return 0;
    size_t len = yyjson_arr_size(yyjson_obj_get(install, "elements"));
    if (index) {
      uint64_t element = yyjson_is_sint(index) ?
          (uint64_t)yyjson_get_sint(index) : yyjson_get_uint(index);
      if (element < len && (state[slot + (size_t)element] & LV4_MAP_SUPPORT_EXPIRED)) return 0;
    } else {
      for (size_t j = 0; j < len; j++)
        if (state[slot + j] & LV4_MAP_SUPPORT_EXPIRED) return 0;
    }
  }
  return 1;
}

/* A successful replacement removes only its exact entry. An End or transfer
 * removes the parent and expires any other entry supported by this Map. A view
 * remembers a replacement or a parent End even if its slot is reused later. */
static void lv4_map_support_change(hako_lv4_function_index* fn, hako_lv4_state* state,
    uint32_t map, yyjson_val* key) {
  if (!fn->has_map_support && !fn->has_map_views) return;
  for (size_t i = 0; i < fn->block_count; i++) {
    size_t slot = fn->blocks[i].map_support;
    yyjson_val* op = lv4_map_operation(fn, fn->blocks[i].ordinal);
    if (slot && state[slot]) {
      yyjson_val* elements = yyjson_obj_get(op, "elements");
      size_t len = yyjson_arr_size(elements);
      if (lv4_u(op, "map") == map &&
          (!key || lv4_map_key_equal(key, lv4_map_install_key(fn, op)))) {
        memset(state + slot, 0, len * sizeof(*state));
      } else if (!key) {
        for (size_t j = 0; j < len; j++)
          if (yyjson_get_uint(yyjson_arr_get(elements, j)) == map)
            state[slot + j] = LV4_MAP_SUPPORT_EXPIRED;
      }
    }
    size_t origin = fn->blocks[i].ordinal;
    if (state[origin] == LV4_LIVE && lv4_is(op, "kind", "map_array_index_map") &&
        lv4_u(op, "map") == map && (!key || lv4_map_key_equal(key, yyjson_obj_get(op, "utf8"))))
      state[origin] = LV4_MAP_VIEW_EXPIRED;
  }
}

/* Installations consume the key on both outcomes. Only Normal replaces its
 * parent entry and activates the new support. */
static int lv4_map_install_commit(hako_lv4_function_index* fn,
    yyjson_val* block, yyjson_val* op, hako_lv4_state* normal) {
  yyjson_val* key = lv4_map_install_key(fn, op);
  if (!yyjson_is_str(key)) return 0;
  lv4_map_support_change(fn, normal, lv4_u(op, "map"), key);
  hako_lv4_block* indexed = lv4_index_block(fn, lv4_u(block, "id"));
  if (!indexed) return 0;
  size_t slot = indexed->map_support;
  if (slot) {
    size_t len = yyjson_arr_size(yyjson_obj_get(op, "elements"));
    for (size_t i = 0; i < len; i++) normal[slot + i] = LV4_MAP_SUPPORT_LIVE;
  }
  return 1;
}

static int lv4_map_install_apply(hako_lv4_function_index* fn, size_t bi,
    yyjson_val* block, yyjson_val* op, hako_lv4_state* state, hako_lv4_state* normal) {
  uint32_t parent = lv4_u(op, "map"), key_id = lv4_u(op, "key");
  if (!lv4_indexed_live(fn, parent, LV4_MAP, state, 0, 0) ||
      !lv4_indexed_live(fn, key_id, LV4_KEY, state, 0, 0)) return 0;
  if (lv4_is(op, "kind", "map_install_borrowed_array")) {
    yyjson_val* elements = yyjson_obj_get(op, "elements");
    size_t len = yyjson_arr_size(elements);
    if (!len) return 0;
    for (size_t i = 0; i < len; i++) {
      yyjson_val* element = yyjson_arr_get(elements, i);
      if (!yyjson_is_uint(element) || yyjson_get_uint(element) > UINT32_MAX ||
          !lv4_indexed_map_readable(fn, (uint32_t)yyjson_get_uint(element), state)) return 0;
    }
  } else if (lv4_is(op, "kind", "map_install_value")) {
    hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
    uint32_t kind = lv4_u(op, "value_kind");
    if (!value || !((kind == NYRT_MAP_VALUE_I64 && value->kind == LV4_I64) ||
        (kind == NYRT_MAP_VALUE_BOOL && value->kind == LV4_BOOL) ||
        (kind == NYRT_MAP_VALUE_BORROWED_HANDLE && value->kind == LV4_I64))) return 0;
  } else if (lv4_is(op, "kind", "map_install_indexed")) {
    hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
    if (!value || value->origin < 0 ||
        !lv4_indexed_live(fn, value->id, LV4_HANDLE, state, lv4_u(op, "object_id"), 1)) return 0;
    normal[value->origin] = lv4_map_transfer(lv4_index_value(fn, parent)->origin);
  }
  if (!lv4_map_install_commit(fn, block, op, normal)) return 0;
  int origin = lv4_index_value(fn, key_id)->origin;
  normal[origin] = state[origin] = 0;
  normal[bi] = LV4_LIVE;
  return 1;
}

static int lv4_map_array_readable(hako_lv4_function_index* fn, yyjson_val* op,
    const hako_lv4_state* state) {
  uint32_t parent = lv4_u(op, "map");
  return lv4_indexed_map_readable(fn, parent, state) &&
      lv4_map_support_allowed(fn, state, parent, yyjson_obj_get(op, "utf8"),
          yyjson_obj_get(op, "index"), 0);
}
