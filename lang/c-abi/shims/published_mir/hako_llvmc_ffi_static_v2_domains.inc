/* Transient physical consistency over the retained row/body graph. Neither
 * reachability nor domain propagation consumes an actual-emission ledger. */
enum HakoStaticV2Domain {
  V2_DOMAIN_STRING = 1u << 6,
  V2_DOMAIN_UNRESOLVED = 1u << 7
};

static unsigned char hako_static_v2_seed_domain(
    const hako_llvmc_value_projection_v2* row, yyjson_val* ins) {
  if (hako_static_v2_is_transfer(row->action)) return 0;
  if (row->value_kind == HAKO_LLVMC_MAP_VALUE_HANDLE &&
      (row->operation == HAKO_LLVMC_MAP_PHYSICAL_STRING_CONCAT ||
       hako_static_v2_text(yyjson_obj_get(ins, "op"), "const") ||
       (hako_static_v2_text(yyjson_obj_get(ins, "op"), "variant_project") &&
        hako_static_v2_text(yyjson_obj_get(ins, "payload_type"), "String")))) return V2_DOMAIN_STRING;
  return (unsigned char)(1u << row->value_kind);
}

static int hako_static_v2_value_closure(const struct HakoStaticV2Index* index) {
  size_t n = index->frame->value_count;
  unsigned char* reached = n ? calloc(n, 1) : NULL;
  unsigned char* domains = n ? calloc(n, 1) : NULL;
  if (n && (!reached || !domains)) { free(reached); free(domains); return 0; }
  int valid = 0;
  for (size_t mi = 0; mi < index->frame->map_operation_count; mi++) {
    if (index->frame->map_operations[mi].kind != HAKO_LLVMC_MAP_OP_WRITE) continue;
    uint32_t value;
    if (!hako_static_v2_u32(yyjson_obj_get(index->maps[mi].instruction, "value"), &value)) goto done;
    int vi = hako_static_v2_value_find(index, index->maps[mi].function, value);
    if (vi < 0) goto done;
    reached[vi] = 1;
  }
  int changed;
  do {
    changed = 0;
    for (size_t vi = 0; vi < n; vi++) {
      if (!reached[vi]) continue;
      const hako_llvmc_value_projection_v2* row = &index->frame->values[vi];
      uint32_t inputs[16];
      int count = hako_static_v2_value_inputs(row, index->values[vi].instruction, inputs);
      if (count < 0) goto done;
      for (int i = 0; i < count; i++) {
        int child = hako_static_v2_value_find(index, index->values[vi].function, inputs[i]);
        if (child < 0 || (row->flags && !index->frame->values[child].flags)) goto done;
        if (!reached[child]) { reached[child] = 1; changed = 1; }
      }
    }
  } while (changed);
  for (size_t vi = 0; vi < n; vi++) if (!reached[vi]) goto done;
  for (int pass = 0; pass < 2; pass++) {
    do {
      changed = 0;
      for (size_t vi = 0; vi < n; vi++) {
        const hako_llvmc_value_projection_v2* row = &index->frame->values[vi];
        uint32_t inputs[16];
        int count = hako_static_v2_value_inputs(row, index->values[vi].instruction, inputs);
        unsigned char incoming = 0;
        int supplied = 1;
        for (int i = 0; i < count; i++) {
          int child = hako_static_v2_value_find(index, index->values[vi].function, inputs[i]);
          incoming |= domains[child];
          if (!domains[child]) supplied = 0;
        }
        unsigned char next = hako_static_v2_is_transfer(row->action) ? incoming
            : row->action == HAKO_LLVMC_MAP_ACTION_OPERATION && !supplied ? 0
            : hako_static_v2_seed_domain(row, index->values[vi].instruction);
        next |= incoming & V2_DOMAIN_UNRESOLVED;
        next |= domains[vi];
        if (next != domains[vi]) { domains[vi] = next; changed = 1; }
      }
    } while (changed);
    if (!pass) for (size_t vi = 0; vi < n; vi++)
      if (!domains[vi]) domains[vi] = V2_DOMAIN_UNRESOLVED;
  }
  for (size_t vi = 0; vi < n; vi++) {
    if (domains[vi] & V2_DOMAIN_UNRESOLVED) goto done;
    const hako_llvmc_value_projection_v2* row = &index->frame->values[vi];
    if (row->action != HAKO_LLVMC_MAP_ACTION_OPERATION) continue;
    unsigned char expected = 1u << HAKO_LLVMC_MAP_VALUE_I64;
    if (row->operation == HAKO_LLVMC_MAP_PHYSICAL_BOOL_COMPARE || row->operation == HAKO_LLVMC_MAP_PHYSICAL_BOOL_NOT)
      expected = 1u << HAKO_LLVMC_MAP_VALUE_BOOL;
    if (row->operation == HAKO_LLVMC_MAP_PHYSICAL_STRING_COMPARE || row->operation == HAKO_LLVMC_MAP_PHYSICAL_STRING_CONCAT)
      expected = V2_DOMAIN_STRING;
    uint32_t inputs[16];
    int count = hako_static_v2_value_inputs(row, index->values[vi].instruction, inputs);
    for (int i = 0; i < count; i++) {
      int child = hako_static_v2_value_find(index, index->values[vi].function, inputs[i]);
      if (domains[child] != expected) goto done;
    }
  }
  valid = 1;
done:
  free(reached);
  free(domains);
  return valid;
}
