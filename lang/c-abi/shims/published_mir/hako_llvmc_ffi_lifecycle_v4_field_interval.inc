/* Query purpose belongs to the same interval proof, not a source classifier. */
typedef enum {
  LV4_FIELD_UNCHANGED, LV4_FIELD_SUPPORT_END,
  LV4_FIELD_ARRAY_DRAIN, LV4_FIELD_OBJECT_DRAIN, LV4_FIELD_DYNAMIC_DRAIN, LV4_FIELD_BORROW_SLOT
} hako_lv4_field_interval_kind;

/* An argument-free Call to a scalar-leaf body mutates no Map slot or residence. */
static int lv4_call_scalar_leaf(const hako_lv4_function_index* fn, yyjson_val* op) {
  yyjson_val* call = yyjson_obj_get(op, "call");
  uint32_t target = lv4_u(call, "target");
  return fn->peers && target && target < fn->peer_count && fn->peers[target].leaf_scalar &&
      !yyjson_arr_size(yyjson_obj_get(call, "args"));
}

/* One reverse interval owner. Reads require an unchanged residence; wrapper
 * Home cleanup requires exactly one child discharge after the SAME Normal. */
static int lv4_field_interval_walk(hako_lv4_function_index* fn,
    uint32_t use, uint32_t born, const hako_lv4_value* base,
    uint32_t object, uint32_t field, uint32_t child, hako_lv4_field_interval_kind query,
    unsigned char* seen, size_t* queue) {
  int slot = query == LV4_FIELD_BORROW_SLOT;
  int released = query != LV4_FIELD_UNCHANGED && !slot;
  int full = query >= LV4_FIELD_ARRAY_DRAIN && !slot;
  hako_lv4_block* current = lv4_index_block(fn, use);
  if (!current || !hako_physical_block_dominates(fn->function, born, use)) return 0;
  size_t head = 0, tail = 1;
  int reached = 0;
  queue[0] = 2 * (size_t)(current - fn->blocks);
  while (head < tail) {
    int prefix = head == 0;
    size_t state = queue[head++];
    current = &fn->blocks[state / 2];
    unsigned char discharged = (unsigned char)(state % 2);
    /* The initial use is before its own terminator. A reverse backedge may
     * visit that same block's full terminator, just as the read suffix does. */
    if (!prefix) {
      yyjson_val* op = yyjson_obj_get(lv4_term(current->block), "operation");
      if (slot && ((lv4_is(op, "kind", "ordinary_call") && !lv4_call_scalar_leaf(fn, op)) || lv4_is(op, "kind", "birth_call") ||
          lv4_is(op, "kind", "object_end_if_owned") ||
          lv4_is(op, "kind", "map_field_install_indexed") || lv4_is(op, "kind", "map_field_delete_indexed") || lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value") ||
          lv4_is(op, "kind", "map_install_text") || lv4_is(op, "kind", "map_install_empty_array") ||
          lv4_is(op, "kind", "map_install_borrowed_array") || lv4_is(op, "kind", "map_end"))) return 0;
      if (full && !discharged && (lv4_is(op, "kind", "ordinary_call") || lv4_is(op, "kind", "birth_call")))
        return 0; /* No callee nonmutation proof is lent to this cleanup suffix. */
      int drain = lv4_is(op, "kind", "object_field_release") || lv4_is(op, "kind", "field_residence_release");
      int write = lv4_is(op, "kind", "object_field_set") || lv4_is(op, "kind", "field_set");
      if ((full || slot) && (lv4_is(op, "kind", "home_release") || lv4_is(op, "kind", "home_release_if_live") ||
          lv4_is(op, "kind", "reclaim_unpublished"))) {
        int same = lv4_field_borrow_same_path(fn, lv4_value_alias_root(fn,
            lv4_index_value(fn, lv4_u(op, "value"))), base);
        if (same != 0) return 0; /* Never borrow a previous lifetime's drain. */
      }
      if ((drain || write) && lv4_u(op, "object_id") == object &&
          (full || slot || lv4_u(op, "field_ordinal") == field)) {
        hako_lv4_value* actual = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "base")));
        int same = lv4_field_borrow_same_path(fn, actual, base);
        if (same < 0 || (!released && same)) return 0;
        if (same) {
          uint32_t ordinal = lv4_u(op, "field_ordinal");
          if (full && ordinal != field) {
            if (drain && ((!discharged && ordinal > field) || (discharged && ordinal < field))) return 0;
          } else if (full && write) {
            if (!discharged) return 0; /* Write after End; old writes before End remain legal. */
          } else {
            int matches = query == LV4_FIELD_ARRAY_DRAIN ?
                lv4_is(op, "kind", "field_residence_release") && !yyjson_obj_get(op, "transaction") :
                query == LV4_FIELD_DYNAMIC_DRAIN ?
                lv4_is(op, "kind", "field_residence_release") && lv4_is(op, "transaction", "take_dynamic") :
                lv4_is(op, "kind", "object_field_release") && lv4_u(op, "child_object_id") == child;
            if (!matches || discharged) return 0;
            discharged = 1; /* Both exits consume this one attempted End. */
          }
        }
      }
    }
    yyjson_arr_iter rows = yyjson_arr_iter_with(yyjson_obj_get(current->block, "instructions"));
    yyjson_val* row;
    while ((row = yyjson_arr_iter_next(&rows))) {
      yyjson_val* ins = yyjson_obj_get(row, "instruction");
      if ((slot || (released && (!full || !discharged))) && lv4_is(ins, "op", "field_set") && lv4_u(ins, "object_id") == object &&
          lv4_u(ins, "field_ordinal") == field &&
          lv4_field_borrow_same_path(fn, lv4_value_alias_root(fn,
              lv4_index_value(fn, lv4_u(ins, "base"))), base) != 0) return 0;
    }
    if (current->id == born) {
      if (released && !discharged) return 0;
      reached = 1; continue;
    }
    if (fn->pred_valid) {
      size_t at = (size_t)(current - fn->blocks);
      for (size_t k = fn->pred_off[at]; k < fn->pred_off[at + 1]; k++) {
        size_t state = 2 * fn->pred_idx[k] + discharged;
        if (seen[state]) continue;
        if (tail >= 2 * fn->block_count + 1) return 0;
        seen[state] = 1; queue[tail++] = state;
      }
      continue;
    }
    for (size_t pi = 0; pi < fn->block_count; pi++) {
      yyjson_arr_iter edges = yyjson_arr_iter_with(yyjson_obj_get(fn->blocks[pi].block, "edges"));
      yyjson_val* edge;
      while ((edge = yyjson_arr_iter_next(&edges))) {
        size_t state = 2 * pi + discharged;
        if (lv4_u(edge, "target") != current->id || seen[state]) continue;
        if (tail >= 2 * fn->block_count + 1) return 0;
        seen[state] = 1; queue[tail++] = state;
      }
    }
  }
  return reached;
}

static int lv4_field_interval(hako_lv4_function_index* fn,
    uint32_t use, uint32_t born, const hako_lv4_value* base,
    uint32_t object, uint32_t field, uint32_t child, hako_lv4_field_interval_kind query) {
  size_t count = fn->block_count;
  if (!count || count > (SIZE_MAX - 1) / 2) return 0;
  size_t capacity = 2 * count + 1;
  unsigned char* seen = lv4_checked_calloc(2 * count, sizeof(*seen));
  size_t* queue = lv4_checked_calloc(capacity, sizeof(*queue));
  int valid = seen && queue && lv4_field_interval_walk(
      fn, use, born, base, object, field, child, query, seen, queue);
  free(seen); free(queue);
  return valid;
}

static int lv4_field_borrow_suffix(hako_lv4_function_index* fn, uint32_t use,
    uint32_t born, const hako_lv4_value* base, uint32_t object, uint32_t field) {
  return lv4_field_interval(fn, use, born, base, object, field, 0, LV4_FIELD_UNCHANGED);
}

static int lv4_field_dependency_discharged(hako_lv4_function_index* fn,
    uint32_t use, uint32_t born, const hako_lv4_value* base,
    uint32_t object, uint32_t field, uint32_t child) {
  return lv4_field_interval(fn, use, born, base, object, field, child, LV4_FIELD_SUPPORT_END);
}

static int lv4_map_slot_interval(hako_lv4_function_index* fn, uint32_t use, uint32_t born,
    const hako_lv4_value* base, uint32_t object, uint32_t field) {
  return lv4_field_interval(fn, use, born, base, object, field, 0, LV4_FIELD_BORROW_SLOT);
}
