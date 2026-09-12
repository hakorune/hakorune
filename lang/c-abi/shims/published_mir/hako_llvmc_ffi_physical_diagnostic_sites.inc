/* Physical diagnostic-site admission; included by the sole v2 parser. */
/* This checks wire identities, not source semantics or runtime reachability. */

static int hako_physical_checked_sites_unique(yyjson_val* functions, uint64_t process_result_site) {
  uint64_t* sites = NULL;
  size_t count = 0, capacity = 0, fi, bi, ri;
  for (fi = 0; functions && fi < yyjson_arr_size(functions); fi++) {
    yyjson_val* blocks = hako_physical_blocks(yyjson_arr_get(functions, fi));
    for (bi = 0; blocks && bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* rows = yyjson_obj_get(block, "instructions");
      yyjson_val* term = yyjson_obj_get(block, "terminator");
      for (ri = 0; ri <= yyjson_arr_size(rows); ri++) {
        yyjson_val* row = ri == yyjson_arr_size(rows) ? term : yyjson_arr_get(rows, ri);
        yyjson_val* instruction = yyjson_obj_get(row, "instruction");
        yyjson_val* operation;
        const char* kind;
        uint64_t site;
        size_t i;
        if (!hako_physical_string(yyjson_obj_get(instruction, "op"), "invoke")) continue;
        operation = yyjson_obj_get(instruction, "operation");
        kind = yyjson_get_str(yyjson_obj_get(operation, "kind"));
        if (!kind || (strcmp(kind, "new_box") && strcmp(kind, "field_set") &&
            strcmp(kind, "home_release") && strcmp(kind, "reclaim_unpublished") &&
            strcmp(kind, "array_new") && strcmp(kind, "array_claim") && strcmp(kind, "array_append") && !hako_physical_map_kind(kind))) continue;
        if (!hako_physical_u64(yyjson_obj_get(operation, "site"), &site)) { free(sites); return 0; }
        if (site == process_result_site) { free(sites); return 0; }
        for (i = 0; i < count; i++) if (sites[i] == site) { free(sites); return 0; }
        if (count == capacity) {
          size_t next = capacity ? capacity * 2u : 8u;
          uint64_t* grown = realloc(sites, next * sizeof(*sites));
          if (!grown) { free(sites); return 0; }
          sites = grown; capacity = next;
        }
        sites[count++] = site;
      }
    }
  }
  free(sites);
  /* An empty checked set is valid; the required process site remains separate. */
  return 1;
}
