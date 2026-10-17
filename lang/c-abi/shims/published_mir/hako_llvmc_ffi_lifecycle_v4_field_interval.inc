/* One reverse interval owner. Reads require an unchanged residence; wrapper
 * Home cleanup requires exactly one child discharge after the SAME Normal. */
static int lv4_field_interval_walk(hako_lv4_function_index* fn,
    uint32_t use, uint32_t born, const hako_lv4_value* base,
    uint32_t object, uint32_t field, uint32_t child, int released,
    unsigned char* seen, size_t* queue) {
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
      if ((lv4_is(op, "kind", "object_field_release") || lv4_is(op, "kind", "field_residence_release") ||
          lv4_is(op, "kind", "object_field_set") || lv4_is(op, "kind", "field_set")) &&
          lv4_u(op, "object_id") == object && lv4_u(op, "field_ordinal") == field) {
        hako_lv4_value* actual = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "base")));
        int same = lv4_field_borrow_same_path(fn, actual, base);
        if (same < 0 || (!released && same)) return 0;
        if (same) {
          if (!lv4_is(op, "kind", "object_field_release") ||
              lv4_u(op, "child_object_id") != child || discharged) return 0;
          discharged = 1; /* Both Normal and Fault consume the same slot. */
        }
      }
    }
    yyjson_arr_iter rows = yyjson_arr_iter_with(yyjson_obj_get(current->block, "instructions"));
    yyjson_val* row;
    while ((row = yyjson_arr_iter_next(&rows))) {
      yyjson_val* ins = yyjson_obj_get(row, "instruction");
      if (released && lv4_is(ins, "op", "field_set") && lv4_u(ins, "object_id") == object &&
          lv4_u(ins, "field_ordinal") == field &&
          lv4_field_borrow_same_path(fn, lv4_value_alias_root(fn,
              lv4_index_value(fn, lv4_u(ins, "base"))), base) != 0) return 0;
    }
    if (current->id == born) {
      if (released && !discharged) return 0;
      reached = 1; continue;
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
    uint32_t object, uint32_t field, uint32_t child, int released) {
  size_t count = fn->block_count;
  if (!count || count > (SIZE_MAX - 1) / 2) return 0;
  size_t capacity = 2 * count + 1;
  unsigned char* seen = lv4_checked_calloc(2 * count, sizeof(*seen));
  size_t* queue = lv4_checked_calloc(capacity, sizeof(*queue));
  int valid = seen && queue && lv4_field_interval_walk(
      fn, use, born, base, object, field, child, released, seen, queue);
  free(seen); free(queue);
  return valid;
}

static int lv4_field_borrow_suffix(hako_lv4_function_index* fn, uint32_t use,
    uint32_t born, const hako_lv4_value* base, uint32_t object, uint32_t field) {
  return lv4_field_interval(fn, use, born, base, object, field, 0, 0);
}

static int lv4_field_dependency_discharged(hako_lv4_function_index* fn,
    uint32_t use, uint32_t born, const hako_lv4_value* base,
    uint32_t object, uint32_t field, uint32_t child) {
  return lv4_field_interval(fn, use, born, base, object, field, child, 1);
}
